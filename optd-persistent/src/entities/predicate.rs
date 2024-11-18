//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "predicate")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub data: Json,
    pub variant: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::predicate_logical_expression_junction::Entity")]
    PredicateLogicalExpressionJunction,
    #[sea_orm(has_many = "super::predicate_physical_expression_junction::Entity")]
    PredicatePhysicalExpressionJunction,
}

impl Related<super::predicate_logical_expression_junction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PredicateLogicalExpressionJunction.def()
    }
}

impl Related<super::predicate_physical_expression_junction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PredicatePhysicalExpressionJunction.def()
    }
}

impl Related<super::logical_expression::Entity> for Entity {
    fn to() -> RelationDef {
        super::predicate_logical_expression_junction::Relation::LogicalExpression.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::predicate_logical_expression_junction::Relation::Predicate
                .def()
                .rev(),
        )
    }
}

impl Related<super::physical_expression::Entity> for Entity {
    fn to() -> RelationDef {
        super::predicate_physical_expression_junction::Relation::PhysicalExpression.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::predicate_physical_expression_junction::Relation::Predicate
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
