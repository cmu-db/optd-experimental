use serde::{Deserialize, Serialize};

use crate::common::values::Value;

pub type ColumnCombValue = Vec<Option<Value>>;

/// Ideally, MostCommonValues would have trait bounds for Serialize and Deserialize. However, I have
/// not figured out how to both have Deserialize as a trait bound and utilize the Deserialize
/// macro, because the Deserialize trait involves lifetimes.
pub trait MostCommonValues: 'static + Send + Sync {
    // it is true that we could just expose freq_over_pred() and use that for freq() and
    // total_freq() however, freq() and total_freq() each have potential optimizations (freq()
    // is O(1) instead of     O(n) and total_freq() can be cached)
    // additionally, it makes sense to return an Option<f64> for freq() instead of just 0 if value
    // doesn't exist thus, I expose three different functions
    fn freq(&self, value: &ColumnCombValue) -> Option<f64>;
    fn total_freq(&self) -> f64;
    fn freq_over_pred(&self, pred: Box<dyn Fn(&ColumnCombValue) -> bool>) -> f64;

    // returns the # of entries (i.e. value + freq) in the most common values structure
    fn cnt(&self) -> usize;
}

/// A more general interface meant to perform the task of a histogram.
///
/// This more general interface is still compatible with histograms but allows
/// more powerful statistics like TDigest.
///
/// Ideally, Distribution would have trait bounds for Serialize and Deserialize.
/// However, I have not figured out how to both have Deserialize as a trait bound
/// and utilize the Deserialize macro, because the Deserialize trait involves lifetimes.
pub trait Distribution: 'static + Send + Sync {
    // Give the probability of a random value sampled from the distribution being <= `value`
    fn cdf(&self, value: &Value) -> f64;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnCombValueStats<M: MostCommonValues, D: Distribution> {
    pub mcvs: M,          // Does NOT contain full nulls.
    pub distr: Option<D>, // Does NOT contain mcvs; optional.
    pub ndistinct: u64,   // Does NOT contain full nulls.
    pub null_frac: f64,   // % of full nulls.
}
