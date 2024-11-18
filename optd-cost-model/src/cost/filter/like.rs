use datafusion::arrow::{array::StringArray, compute::like};

use crate::{
    common::{
        nodes::{PredicateType, ReprPredicateNode},
        predicates::{
            attr_ref_pred::AttrRefPred, constant_pred::ConstantPred, like_pred::LikePred,
        },
    },
    cost_model::CostModelImpl,
    stats::{
        AttributeCombValue, FIXED_CHAR_SEL_FACTOR, FULL_WILDCARD_SEL_FACTOR, UNIMPLEMENTED_SEL,
    },
    storage::CostModelStorageManager,
    CostModelResult,
};

impl<S: CostModelStorageManager> CostModelImpl<S> {
    /// Compute the selectivity of a (NOT) LIKE expression.
    ///
    /// The logic is somewhat similar to Postgres but different. Postgres first estimates the
    /// histogram part of the population and then add up data for any MCV values. If the
    /// histogram is large enough, it just uses the number of matches in the histogram,
    /// otherwise it estimates the fixed prefix and remainder of pattern separately and
    /// combine them.
    ///
    /// Our approach is simpler and less selective. Firstly, we don't use histogram. The selectivity
    /// is composed of MCV frequency and non-MCV selectivity. MCV frequency is computed by
    /// adding up frequencies of MCVs that match the pattern. Non-MCV  selectivity is computed
    /// in the same way that Postgres computes selectivity for the wildcard part of the pattern.
    pub(crate) async fn get_like_selectivity(&self, like_expr: &LikePred) -> CostModelResult<f64> {
        let child = like_expr.child();

        // Check child is a attribute ref.
        if !matches!(child.typ, PredicateType::AttrRef) {
            return Ok(UNIMPLEMENTED_SEL);
        }

        // Check pattern is a constant.
        let pattern = like_expr.pattern();
        if !matches!(pattern.typ, PredicateType::Constant(_)) {
            return Ok(UNIMPLEMENTED_SEL);
        }

        let attr_ref_pred = AttrRefPred::from_pred_node(child).unwrap();
        let attr_ref_idx = attr_ref_pred.attr_index();
        let table_id = attr_ref_pred.table_id();

        // TODO: Consider attribute is a derived attribute
        let pattern = ConstantPred::from_pred_node(pattern)
            .expect("we already checked pattern is a constant")
            .value()
            .as_str();

        // Compute the selectivity exculuding MCVs.
        // See Postgres `like_selectivity`.
        let non_mcv_sel = pattern
            .chars()
            .fold(1.0, |acc, c| {
                if c == '%' {
                    acc * FULL_WILDCARD_SEL_FACTOR
                } else {
                    acc * FIXED_CHAR_SEL_FACTOR
                }
            })
            .min(1.0);

        // Compute the selectivity in MCVs.
        // TODO: Handle the case where `attribute_stats` is None.
        if let Some(attribute_stats) = self
            .get_attribute_comb_stats(table_id, &[attr_ref_idx])
            .await?
        {
            let (mcv_freq, null_frac) = {
                let pred = Box::new(move |val: &AttributeCombValue| {
                    let string =
                        StringArray::from(vec![val[0].as_ref().unwrap().as_str().as_ref()]);
                    let pattern = StringArray::from(vec![pattern.as_ref()]);
                    like(&string, &pattern).unwrap().value(0)
                });
                (
                    attribute_stats.mcvs.freq_over_pred(pred),
                    attribute_stats.null_frac,
                )
            };

            let result = non_mcv_sel + mcv_freq;

            Ok(if like_expr.negated() {
                1.0 - result - null_frac
            } else {
                result
            }
            // Postgres clamps the result after histogram and before MCV. See Postgres
            // `patternsel_common`.
            .clamp(0.0001, 0.9999))
        } else {
            Ok(UNIMPLEMENTED_SEL)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        common::{types::TableId, values::Value},
        cost_model::tests::*,
        stats::{
            utilities::{counter::Counter, simple_map::SimpleMap},
            MostCommonValues, FIXED_CHAR_SEL_FACTOR, FULL_WILDCARD_SEL_FACTOR,
        },
    };

    #[tokio::test]
    async fn test_like_no_nulls() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::new(vec![
                (vec![Some(Value::String("abcd".into()))], 0.1),
                (vec![Some(Value::String("abc".into()))], 0.1),
            ])),
            None,
            2,
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_mock_cost_model(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
        );

        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_like_selectivity(&like(table_id, 0, "%abcd%", false))
                .await
                .unwrap(),
            0.1 + FULL_WILDCARD_SEL_FACTOR.powi(2) * FIXED_CHAR_SEL_FACTOR.powi(4)
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_like_selectivity(&like(table_id, 0, "%abc%", false))
                .await
                .unwrap(),
            0.1 + 0.1 + FULL_WILDCARD_SEL_FACTOR.powi(2) * FIXED_CHAR_SEL_FACTOR.powi(3)
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_like_selectivity(&like(table_id, 0, "%abc%", true))
                .await
                .unwrap(),
            1.0 - (0.1 + 0.1 + FULL_WILDCARD_SEL_FACTOR.powi(2) * FIXED_CHAR_SEL_FACTOR.powi(3))
        );
    }

    #[tokio::test]
    async fn test_like_with_nulls() {
        let null_frac = 0.5;
        let mut mcvs_counts = HashMap::new();
        mcvs_counts.insert(vec![Some(Value::String("abcd".into()))], 1);
        let mcvs_total_count = 10;
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::Counter(Counter::new_from_existing(mcvs_counts, mcvs_total_count)),
            None,
            2,
            null_frac,
        );
        let table_id = TableId(0);
        let cost_model = create_mock_cost_model(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
        );

        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_like_selectivity(&like(table_id, 0, "%abcd%", false))
                .await
                .unwrap(),
            0.1 + FULL_WILDCARD_SEL_FACTOR.powi(2) * FIXED_CHAR_SEL_FACTOR.powi(4)
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_like_selectivity(&like(table_id, 0, "%abcd%", true))
                .await
                .unwrap(),
            1.0 - (0.1 + FULL_WILDCARD_SEL_FACTOR.powi(2) * FIXED_CHAR_SEL_FACTOR.powi(4))
                - null_frac
        );
    }
}
