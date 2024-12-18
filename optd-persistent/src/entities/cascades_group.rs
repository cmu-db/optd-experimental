//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "cascades_group")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub latest_winner: Option<i32>,
    pub in_progress: bool,
    pub is_optimized: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::group_winner::Entity")]
    GroupWinner,
    #[sea_orm(has_many = "super::logical_children::Entity")]
    LogicalChildren,
    #[sea_orm(has_many = "super::logical_expression::Entity")]
    LogicalExpression,
    #[sea_orm(has_many = "super::logical_property::Entity")]
    LogicalProperty,
    #[sea_orm(has_many = "super::physical_children::Entity")]
    PhysicalChildren,
    #[sea_orm(
        belongs_to = "super::physical_expression::Entity",
        from = "Column::LatestWinner",
        to = "super::physical_expression::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    PhysicalExpression,
}

impl Related<super::group_winner::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GroupWinner.def()
    }
}

impl Related<super::logical_children::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LogicalChildren.def()
    }
}

impl Related<super::logical_property::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LogicalProperty.def()
    }
}

impl Related<super::physical_children::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PhysicalChildren.def()
    }
}

impl Related<super::logical_expression::Entity> for Entity {
    fn to() -> RelationDef {
        super::logical_children::Relation::LogicalExpression.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::logical_children::Relation::CascadesGroup.def().rev())
    }
}

impl Related<super::physical_expression::Entity> for Entity {
    fn to() -> RelationDef {
        super::physical_children::Relation::PhysicalExpression.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::physical_children::Relation::CascadesGroup
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
