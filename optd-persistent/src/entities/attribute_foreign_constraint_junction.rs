//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "attribute_foreign_constraint_junction")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub attribute_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub constraint_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::attribute::Entity",
        from = "Column::AttributeId",
        to = "super::attribute::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Attribute,
    #[sea_orm(
        belongs_to = "super::constraint_metadata::Entity",
        from = "Column::ConstraintId",
        to = "super::constraint_metadata::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ConstraintMetadata,
}

impl Related<super::attribute::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attribute.def()
    }
}

impl Related<super::constraint_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConstraintMetadata.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
