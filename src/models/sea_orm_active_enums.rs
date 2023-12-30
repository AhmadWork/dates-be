//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "growth")]
pub enum Growth {
    #[sea_orm(string_value = "Adult")]
    Adult,
    #[sea_orm(string_value = "New")]
    New,
    #[sea_orm(string_value = "Young")]
    Young,
}