//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub epoch_id: i32,
    pub timestamp: DateTimeUtc,
    pub source_variant: String,
    pub data: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::group_winner::Entity")]
    GroupWinner,
    #[sea_orm(has_many = "super::plan_cost::Entity")]
    PlanCost,
    #[sea_orm(has_many = "super::statistic::Entity")]
    Statistic,
}

impl Related<super::group_winner::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GroupWinner.def()
    }
}

impl Related<super::plan_cost::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlanCost.def()
    }
}

impl Related<super::statistic::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Statistic.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
