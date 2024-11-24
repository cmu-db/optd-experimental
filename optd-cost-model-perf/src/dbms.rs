use datafusion::{
    arrow::util::display::{ArrayFormatter, FormatOptions},
    common::sql_err,
    error::DataFusionError,
    execution::{
        context::SessionState,
        runtime_env::{RuntimeConfig, RuntimeEnv},
    },
    parquet::arrow::arrow_reader::{
        ArrowReaderMetadata, ParquetRecordBatchReader, ParquetRecordBatchReaderBuilder,
    },
    prelude::{SessionConfig, SessionContext},
    sql::{
        parser::DFParser,
        sqlparser::{dialect::GenericDialect, parser::ParserError},
    },
};
use optd_cost_model::{
    common::types::TableId,
    storage::mock::{BaseTableStats, TableStats},
};
use rayon::prelude::*;

pub type DataFusionBaseTableStats = BaseTableStats;
pub type DataFusionPerTableStats = TableStats;

use crate::tpch::{TpchKit, TpchKitConfig};
use std::{
    collections::HashMap,
    fs::{self, File},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Instant,
};

pub struct DatafusionDBMS {
    workspace_dpath: PathBuf,
}

const WITH_LOGICAL_FOR_TPCH: bool = true;

impl DatafusionDBMS {
    pub async fn new<P: AsRef<Path>>(workspace_dpath: P) -> anyhow::Result<Self> {
        Ok(DatafusionDBMS {
            workspace_dpath: workspace_dpath.as_ref().to_path_buf(),
        })
    }

    async fn new_session_ctx(use_df_logical: bool) -> anyhow::Result<SessionContext> {
        let mut session_config = SessionConfig::from_env()?.with_information_schema(true);

        if !use_df_logical {
            session_config.options_mut().optimizer.max_passes = 0;
        }

        let rn_config = RuntimeConfig::new();
        let runtime_env = RuntimeEnv::new(rn_config.clone())?;
        let ctx = {
            let state =
                SessionState::new_with_config_rt(session_config.clone(), Arc::new(runtime_env));
            SessionContext::new_with_state(state)
        };
        ctx.refresh_catalogs().await?;
        Ok(ctx)
    }

    async fn execute(ctx: &SessionContext, sql: &str) -> anyhow::Result<Vec<Vec<String>>> {
        let sql = unescape_input(sql)?;
        let dialect = Box::new(GenericDialect);
        let statements = DFParser::parse_sql_with_dialect(&sql, dialect.as_ref())?;
        let mut result = Vec::new();
        for statement in statements {
            let df = {
                let plan = ctx.state().statement_to_plan(statement).await?;
                ctx.execute_logical_plan(plan).await?
            };

            let batches = df.collect().await?;

            let options = FormatOptions::default();

            for batch in batches {
                let converters = batch
                    .columns()
                    .iter()
                    .map(|a| ArrayFormatter::try_new(a.as_ref(), &options))
                    .collect::<Result<Vec<_>, _>>()?;
                for row_idx in 0..batch.num_rows() {
                    let mut row = Vec::with_capacity(batch.num_columns());
                    for converter in converters.iter() {
                        let mut buffer = String::with_capacity(8);
                        converter.value(row_idx).write(&mut buffer)?;
                        row.push(buffer);
                    }
                    result.push(row);
                }
            }
        }
        Ok(result)
    }

    async fn create_tpch_tables(ctx: &SessionContext, tpch_kit: &TpchKit) -> anyhow::Result<()> {
        let ddls = fs::read_to_string(&tpch_kit.schema_fpath)?;
        let ddls = ddls
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        for ddl in ddls {
            Self::execute(ctx, ddl).await?;
        }
        Ok(())
    }

    fn build_batch_reader(
        tbl_fpath: PathBuf,
        num_row_groups: usize,
    ) -> impl FnOnce() -> Vec<ParquetRecordBatchReader> {
        move || {
            let groups: Vec<ParquetRecordBatchReader> = (0..num_row_groups)
                .map(|group_num| {
                    let tbl_file = File::open(tbl_fpath.clone()).expect("Failed to open file");
                    let metadata =
                        ArrowReaderMetadata::load(&tbl_file, Default::default()).unwrap();

                    ParquetRecordBatchReaderBuilder::new_with_metadata(
                        tbl_file.try_clone().unwrap(),
                        metadata.clone(),
                    )
                    .with_row_groups(vec![group_num])
                    .build()
                    .unwrap()
                })
                .collect();

            groups
        }
    }

    /// Need to guarantee that each table has a unique ID.
    fn gen_table_id(tbl_paths: Vec<PathBuf>) -> HashMap<String, TableId> {
        let mut tbl_id = 0;
        let mut tbl_id_map = HashMap::new();
        for tbl_fpath in tbl_paths {
            let tbl_name = TpchKit::get_tbl_name_from_tbl_fpath(&tbl_fpath);
            tbl_id_map.insert(tbl_name.to_string(), TableId(tbl_id));
            println!("Table {:?} has ID {:?}", tbl_name, tbl_id);
            tbl_id += 1;
        }
        tbl_id_map
    }

    fn gen_base_stats(tbl_paths: Vec<PathBuf>) -> anyhow::Result<DataFusionBaseTableStats> {
        let base_table_stats = Mutex::new(DataFusionBaseTableStats::default());
        let tbl_id_map = Self::gen_table_id(tbl_paths.clone());
        let now = Instant::now();

        tbl_paths.par_iter().for_each(|tbl_fpath| {
            let tbl_name = TpchKit::get_tbl_name_from_tbl_fpath(tbl_fpath);
            let start = Instant::now();

            // We get the schema from the Parquet file, to ensure there's no divergence between
            // the context and the file we are going to read.
            // Further rounds of refactoring should adapt the entry point of stat gen.
            let tbl_file = File::open(tbl_fpath).expect("Failed to open file");
            let parquet =
                ParquetRecordBatchReaderBuilder::try_new(tbl_file.try_clone().unwrap()).unwrap();
            let schema = parquet.schema();
            // println!("Table {:?} schema: {:#?}", tbl_name, schema);

            let nb_cols = schema.fields().len();
            let single_cols = (0..nb_cols).map(|v| vec![v]).collect::<Vec<_>>();

            let stats_result = DataFusionPerTableStats::from_record_batches(
                Self::build_batch_reader(tbl_fpath.clone(), parquet.metadata().num_row_groups()),
                Self::build_batch_reader(tbl_fpath.clone(), parquet.metadata().num_row_groups()),
                single_cols,
                schema.clone(),
            );

            if let Ok(per_table_stats) = stats_result {
                let mut stats = base_table_stats.lock().unwrap();
                stats.insert(tbl_id_map[&tbl_name], per_table_stats);
            }

            println!(
                "Table {:?} took in total {:?}...",
                tbl_name,
                start.elapsed()
            );
        });

        println!("Total execution time {:?}...", now.elapsed());

        let stats = base_table_stats.into_inner();
        let l = stats.unwrap();
        Ok(l)
    }

    pub async fn get_tpch_stats(
        &mut self,
        tpch_kit_config: &TpchKitConfig,
    ) -> anyhow::Result<DataFusionBaseTableStats> {
        // Create tables in a temporary context to get the schema provider.
        let ctx = Self::new_session_ctx(WITH_LOGICAL_FOR_TPCH).await?;
        let tpch_kit = TpchKit::build(&self.workspace_dpath)?;
        Self::create_tpch_tables(&ctx, &tpch_kit).await?;
        let schema_provider = ctx.catalog("datafusion").unwrap().schema("public").unwrap();

        // Generate the tables
        tpch_kit.gen_tables(tpch_kit_config)?;
        tpch_kit
            .make_parquet_files(tpch_kit_config, schema_provider)
            .await?;
        // Compute base statistics on Parquet.
        let tbl_paths = tpch_kit.get_tbl_fpath_vec(tpch_kit_config, "parquet")?;
        assert!(tbl_paths.len() == tpch_kit.get_tbl_fpath_vec(tpch_kit_config, "tbl")?.len());
        Self::gen_base_stats(tbl_paths)
    }
}

pub fn unescape_input(input: &str) -> datafusion::error::Result<String> {
    let mut chars = input.chars();

    let mut result = String::with_capacity(input.len());
    while let Some(char) = chars.next() {
        if char == '\\' {
            if let Some(next_char) = chars.next() {
                // https://static.rust-lang.org/doc/master/reference.html#literals
                result.push(match next_char {
                    '0' => '\0',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '\\' => '\\',
                    _ => {
                        return sql_err!(ParserError::TokenizerError(format!(
                            "unsupported escape char: '\\{}'",
                            next_char
                        ),))
                    }
                });
            }
        } else {
            result.push(char);
        }
    }

    Ok(result)
}
