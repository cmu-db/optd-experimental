use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

// impl MigrationName for Migration {
//     fn name(&self) -> &str {
//         "m20241026_000001_create_tables" // Make sure this matches with the file name
//     }
// }

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Fingerprint::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Fingerprint::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Fingerprint::Inner).big_unsigned().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Group::GroupId)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Group::Winner).integer().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(LogicalExpression::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LogicalExpression::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LogicalExpression::FingerprintId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(LogicalExpression::Data).json().not_null())
                    .col(
                        ColumnDef::new(LogicalExpression::GroupId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-logical_expression-fingerprint_id")
                            .from(LogicalExpression::Table, LogicalExpression::FingerprintId)
                            .to(Fingerprint::Table, Fingerprint::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-logical_expression-group_id")
                            .from(LogicalExpression::Table, LogicalExpression::GroupId)
                            .to(Group::Table, Group::GroupId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PhysicalExpression::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PhysicalExpression::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PhysicalExpression::FingerprintId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PhysicalExpression::Data).json().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-physical_expression-fingerprint_id")
                            .from(PhysicalExpression::Table, PhysicalExpression::FingerprintId)
                            .to(Fingerprint::Table, Fingerprint::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(LogicalProperty::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LogicalProperty::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LogicalProperty::GroupId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(LogicalProperty::Data).json().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-logical_property-group_id")
                            .from(LogicalProperty::Table, LogicalProperty::GroupId)
                            .to(Group::Table, Group::GroupId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PhysicalProperty::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PhysicalProperty::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PhysicalProperty::PhysicalExpressionId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PhysicalProperty::Data).json().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-physical_property-physical_expression_id")
                            .from(
                                PhysicalProperty::Table,
                                PhysicalProperty::PhysicalExpressionId,
                            )
                            .to(PhysicalExpression::Table, PhysicalExpression::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PhysicalProperty::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(LogicalProperty::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PhysicalExpression::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(LogicalExpression::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Fingerprint::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Fingerprint {
    Table,
    Id,
    Inner,
}

#[derive(DeriveIden)]
enum Group {
    Table,
    GroupId,
    Winner,
}

#[derive(DeriveIden)]
enum LogicalExpression {
    Table,
    Id,
    FingerprintId,
    Data,
    GroupId,
}

#[derive(DeriveIden)]
enum PhysicalExpression {
    Table,
    Id,
    FingerprintId,
    Data,
}

#[derive(DeriveIden)]
enum LogicalProperty {
    Table,
    Id,
    GroupId,
    Data,
}

#[derive(DeriveIden)]
enum PhysicalProperty {
    Table,
    Id,
    PhysicalExpressionId,
    Data,
}
