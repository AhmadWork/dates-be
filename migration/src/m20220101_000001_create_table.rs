use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // type of string
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Token).string().null())
                    .col(ColumnDef::new(User::Coins).integer().not_null())
                    .col(ColumnDef::new(User::Dates).integer().not_null())
                    .col(ColumnDef::new(User::CreatedAt).timestamp().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())

                    .to_owned(),
           )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
// if explicitly define to table name, then to write name at rename area
#[iden(rename = "users")]
pub enum User {
    Table,
    Id,
    Name,
    Email,
    Password,
    Coins,
    Dates,
    Token,
    CreatedAt,
    UpdatedAt
}
