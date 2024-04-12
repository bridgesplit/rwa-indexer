//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use super::sea_orm_active_enums::DataRegistryVersion;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "data_registry")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Vec<u8>,
    pub asset_mint: Vec<u8>,
    pub authority: Vec<u8>,
    pub delegate: Vec<u8>,
    pub version: DataRegistryVersion,
    pub closed: bool,
    pub slot_updated: i64,
    pub created_at: DateTime,
    pub last_updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
