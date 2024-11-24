use std::{collections::HashMap, sync::Arc};

use arrow_schema::{DataType, Schema, SchemaRef};
use datafusion::{
    arrow::array::{
        Array, BooleanArray, Date32Array, Float32Array, Float64Array, Int16Array, Int32Array,
        Int8Array, RecordBatch, StringArray, UInt16Array, UInt32Array, UInt8Array,
    },
    parquet::arrow::arrow_reader::ParquetRecordBatchReader,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::values::SerializableOrderedF64,
    stats::utilities::{
        counter::Counter,
        hyperloglog::{self, HyperLogLog},
        misragries::{self, MisraGries},
    },
    stats::{Distribution, MostCommonValues},
};
use crate::{
    common::{types::TableId, values::Value},
    stats::{
        utilities::tdigest::{self, TDigest},
        AttributeCombValue, AttributeCombValueStats, ColumnsIdx, ColumnsType,
    },
    CostModelResult,
};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use rayon::prelude::*;

use super::CostModelStorageManager;

pub type AttrIndices = Vec<u64>;

type FirstPassState = (
    Vec<HyperLogLog<AttributeCombValue>>,
    Vec<MisraGries<AttributeCombValue>>,
    Vec<i32>,
);

type SecondPassState = (
    Vec<Option<TDigest<Value>>>,
    Vec<Counter<AttributeCombValue>>,
    Vec<i32>,
);

enum StatType {
    Full,    // Mcvs, distr, n_distinct, null_frac.
    Partial, // Only mcvs, n_distinct, null_frac.
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct TableStats {
    pub row_cnt: u64,
    #[serde_as(as = "HashMap<serde_with::json::JsonString, _>")]
    pub column_comb_stats: HashMap<AttrIndices, AttributeCombValueStats>,
}

impl TableStats {
    pub fn new(
        row_cnt: u64,
        column_comb_stats: HashMap<AttrIndices, AttributeCombValueStats>,
    ) -> Self {
        Self {
            row_cnt,
            column_comb_stats,
        }
    }

    fn is_type_supported(data_type: &DataType) -> bool {
        matches!(
            data_type,
            DataType::Boolean
                | DataType::Int8
                | DataType::Int16
                | DataType::Int32
                | DataType::UInt8
                | DataType::UInt16
                | DataType::UInt32
                | DataType::Float32
                | DataType::Float64
                | DataType::Utf8
        )
    }

    fn first_pass_stats_id(nb_stats: usize) -> anyhow::Result<FirstPassState> {
        Ok((
            vec![HyperLogLog::<AttributeCombValue>::new(hyperloglog::DEFAULT_PRECISION); nb_stats],
            vec![MisraGries::<AttributeCombValue>::new(misragries::DEFAULT_K_TO_TRACK); nb_stats],
            vec![0; nb_stats],
        ))
    }

    fn second_pass_stats_id(
        comb_stat_types: &[(Vec<usize>, Vec<DataType>, StatType)],
        mgs: &[MisraGries<AttributeCombValue>],
        nb_stats: usize,
    ) -> anyhow::Result<SecondPassState> {
        Ok((
            comb_stat_types
                .iter()
                .map(|(_, _, stat_type)| match stat_type {
                    StatType::Full => Some(TDigest::new(tdigest::DEFAULT_COMPRESSION)),
                    StatType::Partial => None,
                })
                .collect(),
            mgs.iter()
                .map(|mg| {
                    let mfk = mg.most_frequent_keys().into_iter().cloned().collect_vec();
                    Counter::new(&mfk)
                })
                .collect(),
            vec![0; nb_stats],
        ))
    }

    fn get_stats_types(
        combinations: &[ColumnsIdx],
        schema: &SchemaRef,
    ) -> Vec<(ColumnsIdx, ColumnsType, StatType)> {
        let col_types: Vec<DataType> = schema
            .fields()
            .iter()
            .map(|f| f.data_type().clone())
            .collect();

        combinations
            .iter()
            .map(|cols_idx| {
                let cols_type: Vec<DataType> =
                    cols_idx.iter().map(|&col| col_types[col].clone()).collect();
                let stat_type = if cols_idx.len() == 1 {
                    StatType::Full
                } else {
                    StatType::Partial
                };

                (cols_idx.clone(), cols_type, stat_type)
            })
            .filter(|(_, cols_type, _)| cols_type.iter().all(Self::is_type_supported))
            .collect()
    }

    fn to_typed_column(col: &Arc<dyn Array>, col_type: &DataType) -> Vec<Option<Value>> {
        macro_rules! simple_col_cast {
            ({ $col:expr, $array_type:path, $value_type:path }) => {
                $col.as_any()
                    .downcast_ref::<$array_type>()
                    .unwrap()
                    .iter()
                    .map(|x| x.map($value_type))
                    .collect_vec()
            };
        }

        macro_rules! float_col_cast {
            ({ $col:expr, $array_type:path }) => {
                $col.as_any()
                    .downcast_ref::<$array_type>()
                    .unwrap()
                    .iter()
                    .map(|x| {
                        x.map(|y| {
                            Value::Float(SerializableOrderedF64(OrderedFloat::from(y as f64)))
                        })
                    })
                    .collect_vec()
            };
        }

        macro_rules! utf8_col_cast {
            ({ $col:expr }) => {
                col.as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .iter()
                    .map(|x| x.map(|y| Value::String(y.to_string().into())))
                    .collect::<Vec<_>>()
            };
        }

        match col_type {
            DataType::Boolean => simple_col_cast!({col, BooleanArray, Value::Bool}),
            DataType::Int8 => simple_col_cast!({col, Int8Array, Value::Int8}),
            DataType::Int16 => simple_col_cast!({col, Int16Array, Value::Int16}),
            DataType::Int32 => simple_col_cast!({col, Int32Array, Value::Int32}),
            DataType::UInt8 => simple_col_cast!({col, UInt8Array, Value::UInt8}),
            DataType::UInt16 => simple_col_cast!({col, UInt16Array, Value::UInt16}),
            DataType::UInt32 => simple_col_cast!({col, UInt32Array, Value::UInt32}),
            DataType::Float32 => float_col_cast!({ col, Float32Array }),
            DataType::Float64 => float_col_cast!({ col, Float64Array }),
            DataType::Date32 => simple_col_cast!({col, Date32Array, Value::Date32}),
            DataType::Utf8 => utf8_col_cast!({ col }),
            _ => unreachable!(),
        }
    }

    fn get_column_combs(
        batch: &RecordBatch,
        comb_stat_types: &[(ColumnsIdx, ColumnsType, StatType)],
    ) -> Vec<Vec<AttributeCombValue>> {
        comb_stat_types
            .iter()
            .map(|(comb, types, _)| {
                let mut column_comb_values =
                    vec![AttributeCombValue::with_capacity(comb.len()); batch.num_rows()];

                for (&col_idx, typ) in comb.iter().zip(types.iter()) {
                    let column_values = Self::to_typed_column(batch.column(col_idx), typ);

                    for (row_values, value) in
                        column_comb_values.iter_mut().zip(column_values.iter())
                    {
                        // This redundant copy is faster than making to_typed_column return an
                        // iterator!
                        row_values.push(value.clone());
                    }
                }

                column_comb_values
            })
            .collect()
    }

    fn generate_partial_stats(
        column_combs: &[Vec<AttributeCombValue>],
        mgs: &mut [MisraGries<AttributeCombValue>],
        hlls: &mut [HyperLogLog<AttributeCombValue>],
        null_counts: &mut [i32],
    ) {
        column_combs
            .iter()
            .zip(mgs)
            .zip(hlls)
            .zip(null_counts)
            .for_each(|(((column_comb, mg), hll), count)| {
                let filtered_nulls = column_comb
                    .iter()
                    .filter(|row| row.iter().any(|val| val.is_some()));

                *count += column_comb.len() as i32;

                filtered_nulls.for_each(|e| {
                    mg.insert_element(e, 1);
                    hll.process(e);
                    *count -= 1;
                });
            });
    }

    fn generate_full_stats(
        column_combs: &[Vec<AttributeCombValue>],
        cnts: &mut [Counter<AttributeCombValue>],
        distrs: &mut [Option<TDigest<Value>>],
        row_counts: &mut [i32],
    ) {
        column_combs
            .iter()
            .zip(cnts)
            .zip(distrs)
            .zip(row_counts)
            .for_each(|(((column_comb, cnt), distr), count)| {
                let nb_rows = column_comb.len() as i32;
                *count += nb_rows;
                cnt.aggregate(column_comb);

                if let Some(d) = distr.as_mut() {
                    let filtered_values: Vec<_> = column_comb
                        .iter()
                        .filter(|row| !cnt.is_tracking(row))
                        .filter_map(|row| row.first().and_then(|v| v.as_ref()))
                        .cloned()
                        .collect();

                    d.norm_weight += nb_rows as usize;
                    d.merge_values(&filtered_values);
                }
            });
    }

    pub fn from_record_batches(
        first_batch_reader: impl FnOnce() -> Vec<ParquetRecordBatchReader>,
        second_batch_reader: impl FnOnce() -> Vec<ParquetRecordBatchReader>,
        combinations: Vec<ColumnsIdx>,
        schema: Arc<Schema>,
    ) -> anyhow::Result<Self> {
        let comb_stat_types = Self::get_stats_types(&combinations, &schema);
        let nb_stats = comb_stat_types.len();

        // 1. FIRST PASS: hlls + mgs + null_cnts.
        let local_partial_stats: Vec<_> = first_batch_reader()
            .into_par_iter()
            .map(|group| {
                group.fold(Self::first_pass_stats_id(nb_stats), |local_stats, batch| {
                    let mut local_stats = local_stats?;

                    match batch {
                        Ok(batch) => {
                            let (hlls, mgs, null_cnts) = &mut local_stats;
                            let comb = Self::get_column_combs(&batch, &comb_stat_types);
                            Self::generate_partial_stats(&comb, mgs, hlls, null_cnts);
                            Ok(local_stats)
                        }
                        Err(e) => Err(e.into()),
                    }
                })
            })
            .collect();

        let (hlls, mgs, null_cnts) = local_partial_stats.into_iter().fold(
            Self::first_pass_stats_id(nb_stats),
            |final_stats, local_stats| {
                let mut final_stats = final_stats?;
                let local_stats = local_stats?;

                let (final_hlls, final_mgs, final_counts) = &mut final_stats;
                let (local_hlls, local_mgs, local_counts) = local_stats;

                for i in 0..nb_stats {
                    final_hlls[i].merge(&local_hlls[i]);
                    final_mgs[i].merge(&local_mgs[i]);
                    final_counts[i] += local_counts[i];
                }

                Ok(final_stats)
            },
        )?;

        // 2. SECOND PASS: mcv + tdigest + row_cnts.
        let local_final_stats: Vec<_> = second_batch_reader()
            .into_par_iter()
            .map(|group| {
                group.fold(
                    Self::second_pass_stats_id(&comb_stat_types, &mgs, nb_stats),
                    |local_stats, batch| {
                        let mut local_stats = local_stats?;

                        match batch {
                            Ok(batch) => {
                                let (distrs, cnts, row_cnts) = &mut local_stats;
                                let comb = Self::get_column_combs(&batch, &comb_stat_types);
                                Self::generate_full_stats(&comb, cnts, distrs, row_cnts);
                                Ok(local_stats)
                            }
                            Err(e) => Err(e.into()),
                        }
                    },
                )
            })
            .collect();

        let (distrs, cnts, row_cnts) = local_final_stats.into_iter().fold(
            Self::second_pass_stats_id(&comb_stat_types, &mgs, nb_stats),
            |final_stats, local_stats| {
                let mut final_stats = final_stats?;
                let local_stats = local_stats?;

                let (final_distrs, final_cnts, final_counts) = &mut final_stats;
                let (local_distrs, local_cnts, local_counts) = local_stats;

                for i in 0..nb_stats {
                    final_cnts[i].merge(&local_cnts[i]);
                    if let (Some(final_distr), Some(local_distr)) =
                        (&mut final_distrs[i], &local_distrs[i])
                    {
                        final_distr.merge(local_distr);
                        final_distr.norm_weight += local_distr.norm_weight;
                    }

                    final_counts[i] += local_counts[i];
                }

                Ok(final_stats)
            },
        )?;

        // 3. ASSEMBLE STATS.
        let row_cnt = row_cnts[0];
        let mut column_comb_stats = HashMap::new();

        let iter_comb = comb_stat_types
            .into_iter()
            .map(|(comb, _, _)| comb.into_iter().map(|x| x as u64).collect())
            .zip(cnts)
            .zip(distrs)
            .zip(hlls)
            .zip(null_cnts.iter())
            .map(|((((comb, cnt), distr), hll), null_cnt)| {
                (
                    comb,
                    cnt,
                    distr.map(Distribution::TDigest),
                    hll,
                    *null_cnt as f64,
                )
            });

        for (comb, cnt, distr, hll, null_cnt) in iter_comb {
            let column_stats = AttributeCombValueStats::new(
                MostCommonValues::Counter(cnt),
                distr,
                hll.n_distinct(),
                null_cnt / (row_cnt as f64),
            );
            column_comb_stats.insert(comb, column_stats);
        }

        Ok(Self {
            row_cnt: row_cnt as u64,
            column_comb_stats,
        })
    }
}

pub type BaseTableStats = HashMap<TableId, TableStats>;

pub struct CostModelStorageMockManagerImpl {
    pub(crate) per_table_stats_map: BaseTableStats,
}

impl CostModelStorageMockManagerImpl {
    pub fn new(per_table_stats_map: BaseTableStats) -> Self {
        Self {
            per_table_stats_map,
        }
    }
}

impl CostModelStorageManager for CostModelStorageMockManagerImpl {
    async fn get_attributes_comb_statistics(
        &self,
        table_id: TableId,
        attr_base_indices: &[u64],
    ) -> CostModelResult<Option<AttributeCombValueStats>> {
        let table_stats = self.per_table_stats_map.get(&table_id);
        match table_stats {
            None => Ok(None),
            Some(table_stats) => match table_stats.column_comb_stats.get(attr_base_indices) {
                None => Ok(None),
                Some(stats) => Ok(Some(stats.clone())),
            },
        }
    }

    async fn get_table_row_count(&self, table_id: TableId) -> CostModelResult<Option<u64>> {
        let table_stats = self.per_table_stats_map.get(&table_id);
        Ok(table_stats.map(|stats| stats.row_cnt))
    }
}
