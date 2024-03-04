use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Reports::Table)
                .if_not_exists()
                .col(ColumnDef::new(Reports::Id).integer().not_null().primary_key().auto_increment())
                .col(ColumnDef::new(Reports::CategoryId).integer().unsigned().not_null())
                .col(ColumnDef::new(Reports::Description).text().null())
                .col(ColumnDef::new(Reports::AttachmentsId).integer().unsigned().not_null())
                .col(ColumnDef::new(Reports::Duration).timestamp().not_null())
                .col(ColumnDef::new(Reports::LocationLat).double().not_null())
                .col(ColumnDef::new(Reports::LocationLong).double().not_null())
                .col(ColumnDef::new(Reports::CreatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                .col(ColumnDef::new(Reports::UpdatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                .to_owned(),
        ).await?;

        manager.create_index(Index::create().if_not_exists().name("idx_category_id").table(Reports::Table).col(Reports::CategoryId).to_owned()).await?;
        manager.create_index(Index::create().if_not_exists().name("idx_attachments_id").table(Reports::Table).col(Reports::AttachmentsId).to_owned()).await?;
        manager.create_index(Index::create().if_not_exists().name("idx_duration").table(Reports::Table).col(Reports::Duration).to_owned()).await?;
        manager.create_index(Index::create().if_not_exists().name("idx_location").table(Reports::Table).col(Reports::LocationLat).col(Reports::LocationLong).to_owned()).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_index(Index::drop().if_exists()
                .name("idx_category_id")
                .table(Reports::Table)
                .to_owned()).await?;
        manager.drop_index(Index::drop().if_exists()
            .name("idx_attachments_id")
            .table(Reports::Table)
            .to_owned()).await?;
        manager.drop_index(Index::drop().if_exists()
            .name("idx_duration")
            .table(Reports::Table)
            .to_owned()).await?;
        manager.drop_index(Index::drop().if_exists()
            .name("idx_location")
            .table(Reports::Table)
            .to_owned()).await?;
        manager.drop_table(Table::drop().table(Reports::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Reports {
    Table,
    Id,
    CategoryId,
    Description,
    LocationLat,
    LocationLong,
    CreatedAt,
    UpdatedAt,
    AttachmentsId,
    Duration,
}