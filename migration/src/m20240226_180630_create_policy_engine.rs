use enum_iterator::all;
use sea_orm_migration::prelude::*;

use crate::model::table::{
    ComparisionType, Policy, PolicyAccount, PolicyEngine, PolicyEngineVersion, PolicyType,
};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PolicyEngine::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PolicyEngine::Id)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PolicyEngine::AssetMint).binary().not_null())
                    .col(ColumnDef::new(PolicyEngine::Authority).binary().not_null())
                    .col(ColumnDef::new(PolicyEngine::Delegate).binary().not_null())
                    .col(
                        ColumnDef::new(PolicyEngine::MaxTimeframe)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PolicyEngine::Version)
                            .enumeration(
                                PolicyEngine::PolicyEngineVersion,
                                all::<PolicyEngineVersion>().collect::<Vec<_>>(),
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(PolicyEngine::Policies).json_binary())
                    .col(ColumnDef::new(PolicyEngine::Closed).boolean().not_null())
                    .col(
                        ColumnDef::new(PolicyEngine::SlotUpdated)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PolicyEngine::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(PolicyEngine::LastUpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PolicyAccount::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PolicyAccount::Id)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PolicyAccount::PolicyEngine)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PolicyAccount::SlotUpdated)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PolicyAccount::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(PolicyAccount::LastUpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Policy::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Policy::Id).binary().not_null().primary_key())
                    .col(ColumnDef::new(Policy::PolicyAccount).binary().not_null())
                    .col(
                        ColumnDef::new(Policy::PolicyType)
                            .enumeration(
                                Policy::PolicyType,
                                all::<PolicyType>().collect::<Vec<_>>(),
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Policy::IdentityLevels)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Policy::ComparisionType)
                            .enumeration(
                                Policy::ComparisionType,
                                all::<ComparisionType>().collect::<Vec<_>>(),
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(Policy::Limit).big_unsigned())
                    .col(ColumnDef::new(Policy::Timeframe).big_integer())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PolicyAccount::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Policy::Table).to_owned())
            .await?;

        Ok(())
    }
}
