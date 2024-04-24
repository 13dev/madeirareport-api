use sea_orm::TransactionTrait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let tx = db.begin().await?;
        tx.execute_unprepared(
            r#"
      CREATE TABLE IF NOT EXISTS reports (
        id SERIAL PRIMARY KEY,
        category_id INT NOT NULL,
        description TEXT,
        duration TIMESTAMP NOT NULL,
        location_lat DOUBLE PRECISION NOT NULL,
        location_long DOUBLE PRECISION NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
      );"#).await?;

        tx.execute_unprepared(r#"CREATE INDEX IF NOT EXISTS idx_category_id ON reports (category_id);"#).await?;
        tx.execute_unprepared(r#"CREATE INDEX IF NOT EXISTS idx_duration ON reports (duration);"#).await?;
        tx.execute_unprepared(r#"CREATE INDEX IF NOT EXISTS idx_location ON reports (location_lat, location_long);"#).await?;
        tx.commit().await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let tx = db.begin().await?;
        tx.execute_unprepared(r#"DROP TABLE IF EXISTS reports;"#).await?;
        tx.execute_unprepared(r#"DROP INDEX IF EXISTS idx_category_id;"#).await?;
        tx.execute_unprepared(r#"DROP INDEX IF EXISTS idx_attachments_id;"#).await?;
        tx.execute_unprepared(r#"DROP INDEX IF EXISTS idx_duration;"#).await?;
        tx.execute_unprepared(r#"DROP INDEX IF EXISTS idx_location;"#).await?;
        tx.commit().await?;
        Ok(())
    }
}