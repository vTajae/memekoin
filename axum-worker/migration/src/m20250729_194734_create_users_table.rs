use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(uuid(Users::Id).primary_key())
                    .col(string_len(Users::Username, 50).unique_key().not_null())
                    .col(string_len(Users::Email, 100).unique_key().not_null())
                    .col(text(Users::PasswordHash).not_null())
                    .col(timestamp_with_time_zone(Users::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Users::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        // Create indexes for faster lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_users_username")
                    .table(Users::Table)
                    .col(Users::Username)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_email")
                    .table(Users::Table)
                    .col(Users::Email)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the users table
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    CreatedAt,
    UpdatedAt,
}
