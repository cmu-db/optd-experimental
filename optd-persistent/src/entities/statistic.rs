//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "statistic")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub table_id: i32,
    pub epoch_id: i32,
    pub created_time: DateTimeUtc,
    pub number_of_attributes: i32,
    pub statistic_type: i32,
    #[sea_orm(column_type = "Float")]
    pub statistic_value: f32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::event::Entity",
        from = "Column::EpochId",
        to = "super::event::Column::EpochId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Event,
    #[sea_orm(has_many = "super::statistic_to_attribute_junction::Entity")]
    StatisticToAttributeJunction,
    #[sea_orm(
        belongs_to = "super::table_metadata::Entity",
        from = "Column::TableId",
        to = "super::table_metadata::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    TableMetadata,
}

impl Related<super::event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}

impl Related<super::statistic_to_attribute_junction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StatisticToAttributeJunction.def()
    }
}

impl Related<super::table_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TableMetadata.def()
    }
}

impl Related<super::attribute::Entity> for Entity {
    fn to() -> RelationDef {
        super::statistic_to_attribute_junction::Relation::Attribute.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::statistic_to_attribute_junction::Relation::Statistic
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}