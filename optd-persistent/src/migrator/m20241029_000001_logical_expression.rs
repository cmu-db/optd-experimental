use super::cascades_group::CascadesGroup;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum LogicalExpression {
    Table,
    Id,
    GroupId,
    Fingerprint,
    Data,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241029_000001_logical_expression"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // TODO add expression / root operator variant identifier
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LogicalExpression::Table)
                    .if_not_exists()
                    .col(pk_auto(LogicalExpression::Id))
                    .col(integer(LogicalExpression::GroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-logical_expression-group_id")
                            .from(LogicalExpression::Table, LogicalExpression::GroupId)
                            .to(CascadesGroup::Table, CascadesGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(big_unsigned(LogicalExpression::Fingerprint))
                    .col(json(LogicalExpression::Data))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LogicalExpression::Table).to_owned())
            .await
    }
}
