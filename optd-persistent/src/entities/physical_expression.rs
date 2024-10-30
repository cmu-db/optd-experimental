//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "physical_expression")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub fingerprint: i64,
    pub data: Json,
    pub group_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::cascades_group::Entity",
        from = "Column::GroupId",
        to = "super::cascades_group::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    CascadesGroup,
    #[sea_orm(has_many = "super::physical_group_junction::Entity")]
    PhysicalGroupJunction,
    #[sea_orm(has_many = "super::physical_property::Entity")]
    PhysicalProperty,
}

impl Related<super::physical_group_junction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PhysicalGroupJunction.def()
    }
}

impl Related<super::physical_property::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PhysicalProperty.def()
    }
}

impl Related<super::cascades_group::Entity> for Entity {
    fn to() -> RelationDef {
        super::physical_group_junction::Relation::CascadesGroup.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::physical_group_junction::Relation::PhysicalExpression
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}