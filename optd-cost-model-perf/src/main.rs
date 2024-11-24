use clap::Parser;
use clap::Subcommand;
use optd_cost_model::common::types::GroupId;
use optd_cost_model::common::types::TableId;
use optd_cost_model::stats::AttributeCombValueStats;
use optd_cost_model::test_utils::tests::create_mock_cost_model_with_memo;
use optd_cost_model::test_utils::tests::MemoGroupInfo;
use optd_cost_model::{CostModel, EstimatedStatistic};
use optd_cost_model_perf::dbms::DataFusionBaseTableStats;
use optd_cost_model_perf::dbms::DatafusionDBMS;
use optd_cost_model_perf::shell;
use optd_cost_model_perf::tpch::q6::init_tpch_q6;
use optd_cost_model_perf::tpch::OperatorNode;
use optd_cost_model_perf::tpch::TpchKitConfig;
use optd_cost_model_perf::tpch::TPCH_KIT_POSTGRES;

use std::collections::HashMap;
use std::fs;

const TPCH_QUERIES: &[&str] = &["6"];

#[derive(Parser)]
struct Cli {
    #[clap(long)]
    #[clap(default_value = "optd_perfbench_workspace")]
    #[clap(
        help = "The directory where artifacts required for performance testing (such as pgdata or TPC-H queries) are generated. See comment of parse_pathstr() to see what paths are allowed (TLDR: absolute and relative both ok)."
    )]
    workspace: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Cardbench {
        #[clap(long)]
        #[clap(default_value = "0.01")]
        scale_factor: f64,

        #[clap(long)]
        #[clap(default_value = "15721")]
        seed: i32,

        #[clap(long)]
        #[clap(value_delimiter = ',', num_args = 1..)]
        // This is the current list of all queries that work in perfbench
        #[clap(default_value = None)]
        #[clap(help = "The queries to get the Q-error of")]
        query_ids: Vec<String>,
        // #[clap(long)]
        // #[clap(default_value = "default_user")]
        // #[clap(help = "The name of a user with superuser privileges")]
        // pguser: String,

        // #[clap(long)]
        // #[clap(default_value = "password")]
        // #[clap(help = "The name of a user with superuser privileges")]
        // pgpassword: String,
    },
}

/// We can only handle single attribute stats for now.
/// TODO: is this true??
fn get_single_attr_stats(
    column_comb_stats: HashMap<Vec<u64>, AttributeCombValueStats>,
) -> HashMap<u64, AttributeCombValueStats> {
    let mut single_attr_stats = HashMap::new();
    for (column_comb, comb_stats) in column_comb_stats {
        if column_comb.len() == 1 {
            single_attr_stats.insert(column_comb[0], comb_stats);
        }
    }
    single_attr_stats
}

/// Compute the estimated statistics for a query.
/// WARNING: This is a VERY naive approach. It assumes that the plan nodes form a linear tree, which is not true in general.
/// However, this assumption is valid for TPC-H Q6.
/// TODO: post-order traversal of the plan tree.
async fn compute_stats(
    table_ids: Vec<TableId>,
    memo: HashMap<GroupId, MemoGroupInfo>,
    operator_nodes: Vec<OperatorNode>,
    base_stats: DataFusionBaseTableStats,
) -> EstimatedStatistic {
    let mut per_attribute_stats = vec![];
    let mut row_counts = vec![];
    for table_id in &table_ids {
        let table_stats = &base_stats[&table_id];
        per_attribute_stats.push(get_single_attr_stats(table_stats.column_comb_stats.clone()));
        row_counts.push(Some(table_stats.row_cnt));
    }
    let cost_model = create_mock_cost_model_with_memo(
        table_ids.clone(),
        per_attribute_stats,
        row_counts,
        memo.into(),
    );
    let mut children_stats = EstimatedStatistic(-1.0);
    for mut operator_node in operator_nodes {
        if children_stats != EstimatedStatistic(-1.0) {
            operator_node.children_stats.push(children_stats);
        }
        let stats = cost_model
            .derive_statistics(
                operator_node.typ,
                &operator_node.predicates,
                &operator_node.children_stats,
                operator_node.context,
            )
            .await
            .unwrap();
        println!(
            "Estimated cardinality for {:?}: {}",
            operator_node.typ, stats.0
        );
        children_stats = stats;
    }
    children_stats
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    let workspace_dpath = shell::parse_pathstr(&cli.workspace)?;
    if !workspace_dpath.exists() {
        fs::create_dir(&workspace_dpath)?;
    }

    match cli.command {
        Commands::Cardbench {
            scale_factor,
            seed,
            query_ids,
            // pguser,
            // pgpassword,
        } => {
            let query_ids = if query_ids.is_empty() {
                TPCH_QUERIES.iter().map(|s| s.to_string()).collect()
            } else {
                query_ids
            };
            // let pgdata_dpath = workspace_dpath.join("pgdata");
            let mut dbms = Box::new(DatafusionDBMS::new(&workspace_dpath).await?);
            let tpch_kit_config = TpchKitConfig {
                dbms: String::from(TPCH_KIT_POSTGRES),
                scale_factor,
                seed,
                query_ids: query_ids.clone(),
            };
            let base_stats = dbms.get_tpch_stats(&tpch_kit_config).await?;
            let (table_ids, memo, operator_nodes) = init_tpch_q6();
            let stats = compute_stats(table_ids, memo, operator_nodes, base_stats).await;
            println!("Estimated cardinality: {}", stats.0);
            Ok(())
        }
    }
}
