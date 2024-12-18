//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub token: Option<String>,
    pub coins: i32,
    pub dates: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::palms::Entity")]
    Palms,
}

impl Related<super::palms::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Palms.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
