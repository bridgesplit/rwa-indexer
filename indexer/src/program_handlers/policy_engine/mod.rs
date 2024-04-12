use crate::{config::IndexerConfig, error::IndexerError};
use plerkle_serialization::AccountInfo;
use policy_engine::PolicyType;
use rwa_types::dao::{
    policy, policy_account, policy_engine as engine,
    sea_orm_active_enums::{ComparisionType, PolicyEngineVersion},
};
use sea_orm::{
    query::*, sea_query::OnConflict, ActiveValue::Set, ColumnTrait, ConnectionTrait,
    DatabaseConnection, DbBackend, EntityTrait,
};
use serde_json::json;
use transformer::programs::policy_engine::PolicyEngineProgram;

pub async fn handle_policy_engine_program_account<'a, 'b, 'c>(
    account_update: &'a AccountInfo<'a>,
    parsing_result: &'b PolicyEngineProgram,
    db: &'c DatabaseConnection,
    _config: &IndexerConfig,
) -> Result<(), IndexerError> {
    let key = *account_update.pubkey().unwrap();
    let key_bytes = key.0.to_vec();
    match &parsing_result {
        PolicyEngineProgram::PolicyEngine(pe) => {
            let active_model = engine::ActiveModel {
                id: Set(key_bytes.clone()),
                asset_mint: Set(pe.asset_mint.to_bytes().to_vec()),
                authority: Set(pe.authority.to_bytes().to_vec()),
                delegate: Set(pe.delegate.to_bytes().to_vec()),
                max_timeframe: Set(pe.max_timeframe),
                version: Set(PolicyEngineVersion::from(pe.version)),
                closed: Set(false),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };

            let mut query = engine::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([engine::Column::Id])
                        .update_columns([
                            engine::Column::AssetMint,
                            engine::Column::Authority,
                            engine::Column::Delegate,
                            engine::Column::MaxTimeframe,
                            engine::Column::Policies,
                            engine::Column::Version,
                            engine::Column::SlotUpdated,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);

            query.sql = format!(
                "{} WHERE excluded.slot_updated >= policy_engine.slot_updated OR policy_engine.slot_updated IS NULL",
                query.sql);

            let txn = db.begin().await?;
            txn.execute(query)
                .await
                .map_err(|db_err| IndexerError::AssetIndexError(db_err.to_string()))?;
            txn.commit().await?;
            Ok(())
        }
        PolicyEngineProgram::PolicyAccount(pe) => {
            let active_model = policy_account::ActiveModel {
                id: Set(key_bytes.clone()),
                policy_engine: Set(pe.policy_engine.to_bytes().to_vec()),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };
            let txn = db.begin().await?;
            let mut query = policy_account::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([policy_account::Column::Id])
                        .update_columns([
                            policy_account::Column::PolicyEngine,
                            policy_account::Column::SlotUpdated,
                            policy_account::Column::LastUpdatedAt,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);

            query.sql = format!(
                "{} WHERE excluded.slot_updated >= policy_account.slot_updated OR policy_account.slot_updated IS NULL",
                query.sql);

            txn.execute(query)
                .await
                .map_err(|db_err| IndexerError::AssetIndexError(db_err.to_string()))?;
            txn.commit().await?;

            // remove policies that are not in the array of policy accounts anymore and add extra if present using hash
            let policies = pe.policies.clone();

            let current_policies = policy::Entity::find()
                .filter(policy::Column::PolicyAccount.eq(key_bytes.clone()))
                .all(db)
                .await
                .map_err(|db_err| IndexerError::AssetIndexError(db_err.to_string()))?;

            let txn = db.begin().await?;
            txn.begin().await?;
            for policy in policies.clone() {
                let (limit, timeframe) = match policy.policy_type {
                    PolicyType::IdentityApproval => (None, None),
                    PolicyType::TransactionAmountLimit { limit } => (Some(limit as i64), None),
                    PolicyType::TransactionAmountVelocity { limit, timeframe } => {
                        (Some(limit as i64), Some(timeframe))
                    }
                    PolicyType::TransactionCountVelocity { limit, timeframe } => {
                        (Some(limit as i64), Some(timeframe))
                    }
                };

                let policy_type = policy.clone().into();

                let active_model = policy::ActiveModel {
                    id: Set(policy.hash.clone().into_bytes()),
                    policy_account: Set(key_bytes.clone()),
                    policy_type: Set(policy_type),
                    identity_levels: Set(json!(policy.identity_filter.identity_levels)),
                    comparision_type: Set(ComparisionType::from(
                        policy.identity_filter.comparision_type as u8,
                    )),
                    limit: Set(limit), // update when limit is fixed
                    timeframe: Set(timeframe),
                    ..Default::default()
                };

                let query = policy::Entity::insert(active_model)
                    .on_conflict(
                        OnConflict::columns([policy::Column::Id])
                            .update_columns([
                                policy::Column::PolicyType,
                                policy::Column::IdentityLevels,
                                policy::Column::ComparisionType,
                                policy::Column::Limit,
                                policy::Column::Timeframe,
                            ])
                            .to_owned(),
                    )
                    .build(DbBackend::Postgres);
                txn.execute(query).await?;
            }
            txn.commit().await?;

            // delete policies that are in current_policies but not in policies
            for current_policy in current_policies {
                if !policies
                    .clone()
                    .iter()
                    .any(|p| p.hash.clone().into_bytes() == current_policy.id)
                {
                    let active_model = policy::ActiveModel {
                        id: Set(current_policy.id),
                        ..Default::default()
                    };
                    let query = policy::Entity::delete(active_model).build(DbBackend::Postgres);
                    db.execute(query).await?;
                }
            }

            Ok(())
        }
        _ => Err(IndexerError::NotImplemented),
    }?;
    Ok(())
}
