use std::collections::HashSet;

use crate::utils::DisjointSets;

pub type BaseTableAttrRefs = Vec<AttrRef>;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct BaseTableAttrRef {
    pub table: String,
    pub attr_idx: usize,
}

#[derive(Clone, Debug)]
pub enum AttrRef {
    BaseTableAttrRef(BaseTableAttrRef),
    /// TODO: Better representation of derived attributes (e.g. t.v1 + t.v2).
    Derived,
}

impl AttrRef {
    pub fn base_table_attr_ref(table: String, attr_idx: usize) -> Self {
        AttrRef::BaseTableAttrRef(BaseTableAttrRef { table, attr_idx })
    }
}

impl From<BaseTableAttrRef> for AttrRef {
    fn from(attr: BaseTableAttrRef) -> Self {
        AttrRef::BaseTableAttrRef(attr)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EqPredicate {
    pub left: BaseTableAttrRef,
    pub right: BaseTableAttrRef,
}

impl EqPredicate {
    pub fn new(left: BaseTableAttrRef, right: BaseTableAttrRef) -> Self {
        Self { left, right }
    }
}

/// `SemanticCorrelation` represents the semantic correlation between attributes in a
/// query. "Semantic" means that the attributes are correlated based on the
/// semantics of the query, not the statistics.
///
/// `SemanticCorrelation` contains equal attributes denoted by disjoint sets of base
/// table attributes, e.g. {{ t1.c1 = t2.c1 = t3.c1 }, { t1.c2 = t2.c2 }}.

#[derive(Clone, Debug, Default)]
pub struct SemanticCorrelation {
    /// A disjoint set of base table attributes with equal values in the same row.
    disjoint_eq_attr_sets: DisjointSets<BaseTableAttrRef>,
    /// The predicates that define the equalities.
    eq_predicates: HashSet<EqPredicate>,
}

impl SemanticCorrelation {
    pub fn new() -> Self {
        Self {
            disjoint_eq_attr_sets: DisjointSets::new(),
            eq_predicates: HashSet::new(),
        }
    }

    pub fn add_predicate(&mut self, predicate: EqPredicate) {
        let left = &predicate.left;
        let right = &predicate.right;

        // Add the indices to the set if they do not exist.
        if !self.disjoint_eq_attr_sets.contains(left) {
            self.disjoint_eq_attr_sets
                .make_set(left.clone())
                .expect("just checked left attribute index does not exist");
        }
        if !self.disjoint_eq_attr_sets.contains(right) {
            self.disjoint_eq_attr_sets
                .make_set(right.clone())
                .expect("just checked right attribute index does not exist");
        }
        // Union the attributes.
        self.disjoint_eq_attr_sets
            .union(left, right)
            .expect("both attribute indices should exist");

        // Keep track of the predicate.
        self.eq_predicates.insert(predicate);
    }

    /// Determine if two attributes are in the same set.
    pub fn is_eq(&mut self, left: &BaseTableAttrRef, right: &BaseTableAttrRef) -> bool {
        self.disjoint_eq_attr_sets
            .same_set(left, right)
            .unwrap_or(false)
    }

    pub fn contains(&self, base_attr_ref: &BaseTableAttrRef) -> bool {
        self.disjoint_eq_attr_sets.contains(base_attr_ref)
    }

    /// Get the number of attributes that are equal to `attr`, including `attr` itself.
    pub fn num_eq_attributes(&mut self, attr: &BaseTableAttrRef) -> usize {
        self.disjoint_eq_attr_sets.set_size(attr).unwrap()
    }

    /// Find the set of predicates that define the equality of the set of attributes `attr` belongs to.
    pub fn find_predicates_for_eq_attr_set(&mut self, attr: &BaseTableAttrRef) -> Vec<EqPredicate> {
        let mut predicates = Vec::new();
        for predicate in &self.eq_predicates {
            let left = &predicate.left;
            let right = &predicate.right;
            if (left != attr && self.disjoint_eq_attr_sets.same_set(attr, left).unwrap())
                || (right != attr && self.disjoint_eq_attr_sets.same_set(attr, right).unwrap())
            {
                predicates.push(predicate.clone());
            }
        }
        predicates
    }

    /// Find the set of attributes that define the equality of the set of attributes `attr` belongs to.
    pub fn find_attrs_for_eq_attribute_set(
        &mut self,
        attr: &BaseTableAttrRef,
    ) -> HashSet<BaseTableAttrRef> {
        let predicates = self.find_predicates_for_eq_attr_set(attr);
        predicates
            .into_iter()
            .flat_map(|predicate| vec![predicate.left, predicate.right])
            .collect()
    }

    /// Union two `EqBaseTableattributesets` to produce a new disjoint sets.
    pub fn union(x: Self, y: Self) -> Self {
        let mut eq_attr_sets = Self::new();
        for predicate in x
            .eq_predicates
            .into_iter()
            .chain(y.eq_predicates.into_iter())
        {
            eq_attr_sets.add_predicate(predicate);
        }
        eq_attr_sets
    }

    pub fn merge(x: Option<Self>, y: Option<Self>) -> Option<Self> {
        let eq_attr_sets = match (x, y) {
            (Some(x), Some(y)) => Self::union(x, y),
            (Some(x), None) => x.clone(),
            (None, Some(y)) => y.clone(),
            _ => return None,
        };
        Some(eq_attr_sets)
    }
}

#[derive(Clone, Debug)]
pub struct GroupattributeRefs {
    attribute_refs: BaseTableAttrRefs,
    /// Correlation of the output attributes of the group.
    output_correlation: Option<SemanticCorrelation>,
}

impl GroupattributeRefs {
    pub fn new(
        attribute_refs: BaseTableAttrRefs,
        output_correlation: Option<SemanticCorrelation>,
    ) -> Self {
        Self {
            attribute_refs,
            output_correlation,
        }
    }

    pub fn base_table_attribute_refs(&self) -> &BaseTableAttrRefs {
        &self.attribute_refs
    }

    pub fn output_correlation(&self) -> Option<&SemanticCorrelation> {
        self.output_correlation.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_base_table_attribute_sets() {
        let attr1 = BaseTableAttrRef {
            table: "t1".to_string(),
            attr_idx: 1,
        };
        let attr2 = BaseTableAttrRef {
            table: "t2".to_string(),
            attr_idx: 2,
        };
        let attr3 = BaseTableAttrRef {
            table: "t3".to_string(),
            attr_idx: 3,
        };
        let attr4 = BaseTableAttrRef {
            table: "t4".to_string(),
            attr_idx: 4,
        };
        let pred1 = EqPredicate::new(attr1.clone(), attr2.clone());
        let pred2 = EqPredicate::new(attr3.clone(), attr4.clone());
        let pred3 = EqPredicate::new(attr1.clone(), attr3.clone());

        let mut eq_attr_sets = SemanticCorrelation::new();

        // (1, 2)
        eq_attr_sets.add_predicate(pred1.clone());
        assert!(eq_attr_sets.is_eq(&attr1, &attr2));

        // (1, 2), (3, 4)
        eq_attr_sets.add_predicate(pred2.clone());
        assert!(eq_attr_sets.is_eq(&attr3, &attr4));
        assert!(!eq_attr_sets.is_eq(&attr2, &attr3));

        let predicates = eq_attr_sets.find_predicates_for_eq_attr_set(&attr1);
        assert_eq!(predicates.len(), 1);
        assert!(predicates.contains(&pred1));

        let predicates = eq_attr_sets.find_predicates_for_eq_attr_set(&attr3);
        assert_eq!(predicates.len(), 1);
        assert!(predicates.contains(&pred2));

        // (1, 2, 3, 4)
        eq_attr_sets.add_predicate(pred3.clone());
        assert!(eq_attr_sets.is_eq(&attr1, &attr3));
        assert!(eq_attr_sets.is_eq(&attr2, &attr4));
        assert!(eq_attr_sets.is_eq(&attr1, &attr4));

        let predicates = eq_attr_sets.find_predicates_for_eq_attr_set(&attr1);
        assert_eq!(predicates.len(), 3);
        assert!(predicates.contains(&pred1));
        assert!(predicates.contains(&pred2));
        assert!(predicates.contains(&pred3));
    }
}
