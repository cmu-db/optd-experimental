//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "logical_children")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub logical_expression_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub group_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::group::Entity",
        from = "Column::GroupId",
        to = "super::group::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Group,
    #[sea_orm(
        belongs_to = "super::logical_expression::Entity",
        from = "Column::LogicalExpressionId",
        to = "super::logical_expression::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    LogicalExpression,
}

impl Related<super::group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl Related<super::logical_expression::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LogicalExpression.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}