#![allow(unused)]

mod arith_encoder;
pub mod counter;
pub mod tdigest;

use crate::common::values::Value;
use counter::Counter;
use serde::{Deserialize, Serialize};

// Default n-distinct estimate for derived columns or columns lacking statistics
pub const DEFAULT_NUM_DISTINCT: u64 = 200;
// A placeholder for unimplemented!() for codepaths which are accessed by plannertest
pub const UNIMPLEMENTED_SEL: f64 = 0.01;
// Default statistics. All are from selfuncs.h in Postgres unless specified otherwise
// Default selectivity estimate for equalities such as "A = b"
pub const DEFAULT_EQ_SEL: f64 = 0.005;
// Default selectivity estimate for inequalities such as "A < b"
pub const DEFAULT_INEQ_SEL: f64 = 0.3333333333333333;
// Used for estimating pattern selectivity character-by-character. These numbers
// are not used on their own. Depending on the characters in the pattern, the
// selectivity is multiplied by these factors.
//
// See `FULL_WILDCARD_SEL` and `FIXED_CHAR_SEL` in Postgres.
pub const FULL_WILDCARD_SEL_FACTOR: f64 = 5.0;
pub const FIXED_CHAR_SEL_FACTOR: f64 = 0.2;

pub type AttributeCombValue = Vec<Option<Value>>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MostCommonValues {
    Counter(Counter<AttributeCombValue>),
    // Add more types here...
}

impl MostCommonValues {
    // it is true that we could just expose freq_over_pred() and use that for freq() and
    // total_freq() however, freq() and total_freq() each have potential optimizations (freq()
    // is O(1) instead of     O(n) and total_freq() can be cached)
    // additionally, it makes sense to return an Option<f64> for freq() instead of just 0 if value
    // doesn't exist thus, I expose three different functions
    pub fn freq(&self, value: &AttributeCombValue) -> Option<f64> {
        match self {
            MostCommonValues::Counter(counter) => counter.frequencies().get(value).copied(),
        }
    }

    pub fn total_freq(&self) -> f64 {
        match self {
            MostCommonValues::Counter(counter) => counter.frequencies().values().sum(),
        }
    }

    pub fn freq_over_pred(&self, pred: Box<dyn Fn(&AttributeCombValue) -> bool>) -> f64 {
        match self {
            MostCommonValues::Counter(counter) => counter
                .frequencies()
                .iter()
                .filter(|(val, _)| pred(val))
                .map(|(_, freq)| freq)
                .sum(),
        }
    }

    // returns the # of entries (i.e. value + freq) in the most common values structure
    pub fn cnt(&self) -> usize {
        match self {
            MostCommonValues::Counter(counter) => counter.frequencies().len(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Distribution {
    TDigest(tdigest::TDigest<Value>),
    // Add more types here...
}

impl Distribution {
    pub fn cdf(&self, value: &Value) -> f64 {
        match self {
            Distribution::TDigest(tdigest) => {
                let nb_rows = tdigest.norm_weight;
                if nb_rows == 0 {
                    tdigest.cdf(value)
                } else {
                    tdigest.centroids.len() as f64 * tdigest.cdf(value) / nb_rows as f64
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttributeCombValueStats {
    pub mcvs: MostCommonValues,      // Does NOT contain full nulls.
    pub distr: Option<Distribution>, // Does NOT contain mcvs; optional.
    pub ndistinct: u64,              // Does NOT contain full nulls.
    pub null_frac: f64,              // % of full nulls.
}

impl AttributeCombValueStats {
    pub fn new(
        mcvs: MostCommonValues,
        ndistinct: u64,
        null_frac: f64,
        distr: Option<Distribution>,
    ) -> Self {
        Self {
            mcvs,
            ndistinct,
            null_frac,
            distr,
        }
    }
}

impl From<serde_json::Value> for AttributeCombValueStats {
    fn from(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}