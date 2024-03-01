//! SeaORM Entity. Generated by sea-orm-codegen 0.9.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "asset_controller_version"
)]
pub enum AssetControllerVersion {
    #[sea_orm(string_value = "v0")]
    V0,
    #[sea_orm(string_value = "v1")]
    V1,
}
#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "policy_engine_account_version"
)]
pub enum PolicyEngineAccountVersion {
    #[sea_orm(string_value = "v0")]
    V0,
    #[sea_orm(string_value = "v1")]
    V1,
}
#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "transaction_count_velocity_version"
)]
pub enum TransactionCountVelocityVersion {
    #[sea_orm(string_value = "v0")]
    V0,
    #[sea_orm(string_value = "v1")]
    V1,
}
#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "transaction_amount_limit_version"
)]
pub enum TransactionAmountLimitVersion {
    #[sea_orm(string_value = "v0")]
    V0,
    #[sea_orm(string_value = "v1")]
    V1,
}
#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "data_account_type")]
pub enum DataAccountType {
    #[sea_orm(string_value = "legal")]
    Legal,
    #[sea_orm(string_value = "miscellaneous")]
    Miscellaneous,
    #[sea_orm(string_value = "tax")]
    Tax,
    #[sea_orm(string_value = "title")]
    Title,
}
#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "transaction_amount_velocity_version"
)]
pub enum TransactionAmountVelocityVersion {
    #[sea_orm(string_value = "v0")]
    V0,
    #[sea_orm(string_value = "v1")]
    V1,
}
#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "identity_approval_version"
)]
pub enum IdentityApprovalVersion {
    #[sea_orm(string_value = "v0")]
    V0,
    #[sea_orm(string_value = "v1")]
    V1,
}
#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "data_registry_version"
)]
pub enum DataRegistryVersion {
    #[sea_orm(string_value = "v0")]
    V0,
    #[sea_orm(string_value = "v1")]
    V1,
}
#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "identity_registry_version"
)]
pub enum IdentityRegistryVersion {
    #[sea_orm(string_value = "v0")]
    V0,
    #[sea_orm(string_value = "v1")]
    V1,
}
#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "identity_account_version"
)]
pub enum IdentityAccountVersion {
    #[sea_orm(string_value = "v0")]
    V0,
    #[sea_orm(string_value = "v1")]
    V1,
}
