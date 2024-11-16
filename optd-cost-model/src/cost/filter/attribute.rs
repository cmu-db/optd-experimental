use std::ops::Bound;

use optd_persistent::CostModelStorageLayer;

use crate::{
    common::{types::TableId, values::Value},
    cost_model::CostModelImpl,
    // TODO: If we return the default value, consider tell the upper level that we cannot
    // compute the selectivity.
    stats::{AttributeCombValue, AttributeCombValueStats, DEFAULT_EQ_SEL, DEFAULT_INEQ_SEL},
    CostModelResult,
};

impl<S: CostModelStorageLayer> CostModelImpl<S> {
    /// Get the selectivity of an expression of the form "attribute equals value" (or "value equals
    /// attribute") Will handle the case of statistics missing
    /// Equality predicates are handled entirely differently from range predicates so this is its
    /// own function
    /// Also, get_attribute_equality_selectivity is a subroutine when computing range
    /// selectivity, which is another     reason for separating these into two functions
    /// is_eq means whether it's == or !=
    pub(crate) async fn get_attribute_equality_selectivity(
        &self,
        table_id: TableId,
        attr_base_index: usize,
        value: &Value,
        is_eq: bool,
    ) -> CostModelResult<f64> {
        // TODO: The attribute could be a derived attribute
        let ret_sel = {
            if let Some(attribute_stats) = self
                .get_attribute_comb_stats(table_id, &[attr_base_index])
                .await?
            {
                let eq_freq =
                    if let Some(freq) = attribute_stats.mcvs.freq(&vec![Some(value.clone())]) {
                        freq
                    } else {
                        let non_mcv_freq = 1.0 - attribute_stats.mcvs.total_freq();
                        // always safe because usize is at least as large as i32
                        let ndistinct_as_usize = attribute_stats.ndistinct as usize;
                        let non_mcv_cnt = ndistinct_as_usize - attribute_stats.mcvs.cnt();
                        if non_mcv_cnt == 0 {
                            return Ok(0.0);
                        }
                        // note that nulls are not included in ndistinct so we don't need to do non_mcv_cnt
                        // - 1 if null_frac > 0
                        (non_mcv_freq - attribute_stats.null_frac) / (non_mcv_cnt as f64)
                    };
                if is_eq {
                    eq_freq
                } else {
                    1.0 - eq_freq - attribute_stats.null_frac
                }
            } else {
                #[allow(clippy::collapsible_else_if)]
                if is_eq {
                    DEFAULT_EQ_SEL
                } else {
                    1.0 - DEFAULT_EQ_SEL
                }
            }
        };

        assert!(
            (0.0..=1.0).contains(&ret_sel),
            "ret_sel ({}) should be in [0, 1]",
            ret_sel
        );
        Ok(ret_sel)
    }

    /// Compute the frequency of values in a attribute less than or equal to the given value.
    fn get_attribute_leq_value_freq(
        per_attribute_stats: &AttributeCombValueStats,
        value: &Value,
    ) -> f64 {
        // because distr does not include the values in MCVs, we need to compute the CDFs there as
        // well because nulls return false in any comparison, they are never included when
        // computing range selectivity
        let distr_leq_freq = per_attribute_stats.distr.as_ref().unwrap().cdf(value);
        let value = value.clone();
        let pred = Box::new(move |val: &AttributeCombValue| *val[0].as_ref().unwrap() <= value);
        let mcvs_leq_freq = per_attribute_stats.mcvs.freq_over_pred(pred);
        let ret_freq = distr_leq_freq + mcvs_leq_freq;
        assert!(
            (0.0..=1.0).contains(&ret_freq),
            "ret_freq ({}) should be in [0, 1]",
            ret_freq
        );
        ret_freq
    }

    /// Compute the frequency of values in a attribute less than the given value.
    async fn get_attribute_lt_value_freq(
        &self,
        attribute_stats: &AttributeCombValueStats,
        table_id: TableId,
        attr_base_index: usize,
        value: &Value,
    ) -> CostModelResult<f64> {
        // depending on whether value is in mcvs or not, we use different logic to turn total_lt_cdf
        // into total_leq_cdf this logic just so happens to be the exact same logic as
        // get_attribute_equality_selectivity implements
        let ret_freq = Self::get_attribute_leq_value_freq(attribute_stats, value)
            - self
                .get_attribute_equality_selectivity(table_id, attr_base_index, value, true)
                .await?;
        assert!(
            (0.0..=1.0).contains(&ret_freq),
            "ret_freq ({}) should be in [0, 1]",
            ret_freq
        );
        Ok(ret_freq)
    }

    /// Get the selectivity of an expression of the form "attribute </<=/>=/> value" (or "value
    /// </<=/>=/> attribute"). Computes selectivity based off of statistics.
    /// Range predicates are handled entirely differently from equality predicates so this is its
    /// own function. If it is unable to find the statistics, it returns DEFAULT_INEQ_SEL.
    /// The selectivity is computed as quantile of the right bound minus quantile of the left bound.
    pub(crate) async fn get_attribute_range_selectivity(
        &self,
        table_id: TableId,
        attr_base_index: usize,
        start: Bound<&Value>,
        end: Bound<&Value>,
    ) -> CostModelResult<f64> {
        // TODO: Consider attribute is a derived attribute
        if let Some(attribute_stats) = self
            .get_attribute_comb_stats(table_id, &[attr_base_index])
            .await?
        {
            let left_quantile = match start {
                Bound::Unbounded => 0.0,
                Bound::Included(value) => {
                    self.get_attribute_lt_value_freq(
                        &attribute_stats,
                        table_id,
                        attr_base_index,
                        value,
                    )
                    .await?
                }
                Bound::Excluded(value) => {
                    Self::get_attribute_leq_value_freq(&attribute_stats, value)
                }
            };
            let right_quantile = match end {
                Bound::Unbounded => 1.0,
                Bound::Included(value) => {
                    Self::get_attribute_leq_value_freq(&attribute_stats, value)
                }
                Bound::Excluded(value) => {
                    self.get_attribute_lt_value_freq(
                        &attribute_stats,
                        table_id,
                        attr_base_index,
                        value,
                    )
                    .await?
                }
            };
            assert!(
                left_quantile <= right_quantile,
                "left_quantile ({}) should be <= right_quantile ({})",
                left_quantile,
                right_quantile
            );
            Ok(right_quantile - left_quantile)
        } else {
            Ok(DEFAULT_INEQ_SEL)
        }
    }
}
