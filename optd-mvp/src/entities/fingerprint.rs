//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "fingerprint")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub logical_expression_id: i32,
    pub kind: i16,
    pub hash: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::logical_expression::Entity",
        from = "Column::LogicalExpressionId",
        to = "super::logical_expression::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    LogicalExpression,
}

impl Related<super::logical_expression::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LogicalExpression.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
