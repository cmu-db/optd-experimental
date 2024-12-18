#![allow(unused)]

mod arith_encoder;
pub mod utilities;

use crate::common::values::Value;
use serde::{Deserialize, Serialize};
use utilities::counter::Counter;
use utilities::{
    simple_map::{self, SimpleMap},
    tdigest::TDigest,
};

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

// TODO: remove the clone, see the comment in the [`AttributeCombValueStats`]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MostCommonValues {
    Counter(Counter<AttributeCombValue>),
    SimpleFrequency(SimpleMap<AttributeCombValue>),
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
            MostCommonValues::SimpleFrequency(simple_map) => simple_map.m.get(value).copied(),
        }
    }

    pub fn total_freq(&self) -> f64 {
        match self {
            MostCommonValues::Counter(counter) => counter.frequencies().values().sum(),
            MostCommonValues::SimpleFrequency(simple_map) => simple_map.m.values().sum(),
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
            MostCommonValues::SimpleFrequency(simple_map) => simple_map
                .m
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
            MostCommonValues::SimpleFrequency(simple_map) => simple_map.m.len(),
        }
    }

    pub fn empty() -> Self {
        MostCommonValues::SimpleFrequency(SimpleMap::new(vec![]))
    }
}

// TODO: remove the clone, see the comment in the [`AttributeCombValueStats`]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Distribution {
    TDigest(TDigest<Value>),
    SimpleDistribution(SimpleMap<Value>),
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
            Distribution::SimpleDistribution(simple_distribution) => {
                *simple_distribution.m.get(value).unwrap_or(&0.0)
            }
        }
    }

    pub fn empty() -> Self {
        Distribution::SimpleDistribution(SimpleMap::new(vec![]))
    }
}

// TODO: Remove the clone. Now I have to add this because
// persistent.rs doesn't have a memory cache, so we have to
// return AttributeCombValueStats rather than &AttributeCombValueStats.
// But this poses a problem for mock.rs when testing, since mock storage
// only has memory hash map, so we need to return a clone of AttributeCombValueStats.
// Later, if memory cache is added, we should change this to return a reference.
// **and** remove the clone.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeCombValueStats {
    pub mcvs: MostCommonValues,      // Does NOT contain full nulls.
    pub distr: Option<Distribution>, // Does NOT contain mcvs; optional.
    pub ndistinct: u64,              // Does NOT contain full nulls.
    pub null_frac: f64,              // % of full nulls.
}

impl AttributeCombValueStats {
    pub fn new(
        mcvs: MostCommonValues,
        distr: Option<Distribution>,
        ndistinct: u64,
        null_frac: f64,
    ) -> Self {
        Self {
            mcvs,
            ndistinct,
            null_frac,
            distr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Counter, MostCommonValues};
    use crate::{common::values::Value, stats::AttributeCombValue};
    use serde_json::json;

    #[test]
    fn test_most_common_values() {
        let elem1 = vec![Some(Value::Int32(1))];
        let elem2 = vec![Some(Value::Int32(2))];
        let mut counter = Counter::new(&[elem1.clone(), elem2.clone()]);

        let elems = vec![elem2.clone(), elem1.clone(), elem2.clone(), elem2.clone()];
        counter.aggregate(&elems);

        let mcvs = MostCommonValues::Counter(counter);
        assert_eq!(mcvs.freq(&elem1), Some(0.25));
        assert_eq!(mcvs.freq(&elem2), Some(0.75));
        assert_eq!(mcvs.total_freq(), 1.0);

        let elem1_cloned = elem1.clone();
        let pred1 = Box::new(move |x: &AttributeCombValue| x == &elem1_cloned);
        let pred2 = Box::new(move |x: &AttributeCombValue| x != &elem1);
        assert_eq!(mcvs.freq_over_pred(pred1), 0.25);
        assert_eq!(mcvs.freq_over_pred(pred2), 0.75);

        assert_eq!(mcvs.cnt(), 2);
    }

    #[test]
    fn test_most_common_values_serde() {
        let elem1 = vec![Some(Value::Int32(1))];
        let elem2 = vec![Some(Value::Int32(2))];
        let mut counter = Counter::new(&[elem1.clone(), elem2.clone()]);

        let elems = vec![elem2.clone(), elem1.clone(), elem2.clone(), elem2.clone()];
        counter.aggregate(&elems);

        let mcvs = MostCommonValues::Counter(counter);
        let serialized = serde_json::to_value(&mcvs).unwrap();
        println!("serialized: {:?}", serialized);

        let deserialized: MostCommonValues = serde_json::from_value(serialized).unwrap();
        assert_eq!(mcvs.freq(&elem1), Some(0.25));
        assert_eq!(mcvs.freq(&elem2), Some(0.75));
        assert_eq!(mcvs.total_freq(), 1.0);

        let elem1_cloned = elem1.clone();
        let pred1 = Box::new(move |x: &AttributeCombValue| x == &elem1_cloned);
        let pred2 = Box::new(move |x: &AttributeCombValue| x != &elem1);
        assert_eq!(mcvs.freq_over_pred(pred1), 0.25);
        assert_eq!(mcvs.freq_over_pred(pred2), 0.75);

        assert_eq!(mcvs.cnt(), 2);
    }

    // TODO: Add tests for Distribution
}
