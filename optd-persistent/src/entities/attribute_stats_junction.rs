//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "attribute_stats_junction")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub attr_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub stats_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::attribute_stat::Entity",
        from = "Column::StatsId",
        to = "super::attribute_stat::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    AttributeStat,
    #[sea_orm(
        belongs_to = "super::table_attribute::Entity",
        from = "Column::AttrId",
        to = "super::table_attribute::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    TableAttribute,
}

impl Related<super::attribute_stat::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AttributeStat.def()
    }
}

impl Related<super::table_attribute::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TableAttribute.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
