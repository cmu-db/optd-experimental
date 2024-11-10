//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "table_attribute")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub table_id: i32,
    pub name: String,
    pub compression_method: String,
    pub r#type: i32,
    pub base_col_number: i32,
    pub is_not_null: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::attribute_stats_junction::Entity")]
    AttributeStatsJunction,
    #[sea_orm(has_many = "super::constraint_attribute_junction::Entity")]
    ConstraintAttributeJunction,
    #[sea_orm(has_many = "super::foreign_constraint_ref_attribute_junction::Entity")]
    ForeignConstraintRefAttributeJunction,
    #[sea_orm(
        belongs_to = "super::table_metadata::Entity",
        from = "Column::TableId",
        to = "super::table_metadata::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    TableMetadata,
}

impl Related<super::attribute_stats_junction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AttributeStatsJunction.def()
    }
}

impl Related<super::constraint_attribute_junction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConstraintAttributeJunction.def()
    }
}

impl Related<super::foreign_constraint_ref_attribute_junction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ForeignConstraintRefAttributeJunction.def()
    }
}

impl Related<super::table_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TableMetadata.def()
    }
}

impl Related<super::attribute_stat::Entity> for Entity {
    fn to() -> RelationDef {
        super::attribute_stats_junction::Relation::AttributeStat.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::attribute_stats_junction::Relation::TableAttribute
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}