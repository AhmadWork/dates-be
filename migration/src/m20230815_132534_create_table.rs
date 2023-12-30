
use sea_orm_migration::prelude::*;
use sea_orm::EnumIter;
use sea_orm_migration::prelude::extension::postgres::Type;
use crate::m20220101_000001_create_table::User;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
    manager
    .create_type(
               Type::create()
              .as_enum(Growth::Table)
              .values([Growth::New, Growth::Young, Growth::Adult])
            .to_owned(),
    )
    .await?;
             manager.create_table(
                Table::create()
                    .table(Palms::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Palms::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Palms::Dpm).integer().not_null())
                    .col(
                        ColumnDef::new(Palms::Growth)
                            .enumeration(Growth::Table, [Growth::New, Growth::Young, Growth::Adult]))
                    .col(ColumnDef::new(Palms::UserId).integer().not_null())
                                    .foreign_key(ForeignKey::create()
                                    .name("user_id")
                                    .from(Palms::Table, Palms::UserId)
                                    .to(User::Table, User::Id),)
                    .col(ColumnDef::new(User::CreatedAt).timestamp().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Palms::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Palms {
    Table,
    Id,
    Dpm,
    UserId,
    Growth,
}

#[derive(Iden, EnumIter,)]
pub enum Growth {
        Table,
         #[iden = "New"]
            New,
        #[iden = "Young"]
            Young,
        #[iden = "Adult"]
            Adult,
}
