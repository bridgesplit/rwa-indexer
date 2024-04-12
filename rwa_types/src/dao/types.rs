use super::sea_orm_active_enums::{
    AssetControllerVersion, ComparisionType, DataAccountType, DataRegistryVersion,
    IdentityAccountVersion, IdentityRegistryVersion, PolicyEngineVersion, PolicyType,
};
use data_registry::state::DataAccountType as ProgramDataAccountType;
use policy_engine::{Policy, PolicyType as ProgramPolicyType};

impl From<u8> for AssetControllerVersion {
    fn from(version: u8) -> Self {
        match version {
            1 => AssetControllerVersion::V1,
            _ => AssetControllerVersion::V1,
        }
    }
}

impl From<u8> for DataRegistryVersion {
    fn from(version: u8) -> Self {
        match version {
            1 => DataRegistryVersion::V1,
            _ => DataRegistryVersion::V1,
        }
    }
}

impl From<u8> for IdentityRegistryVersion {
    fn from(version: u8) -> Self {
        match version {
            1 => IdentityRegistryVersion::V1,
            _ => IdentityRegistryVersion::V1,
        }
    }
}

impl From<u8> for IdentityAccountVersion {
    fn from(version: u8) -> Self {
        match version {
            1 => IdentityAccountVersion::V1,
            _ => IdentityAccountVersion::V1,
        }
    }
}

impl From<ProgramDataAccountType> for DataAccountType {
    fn from(da: ProgramDataAccountType) -> Self {
        match da {
            ProgramDataAccountType::Title => DataAccountType::Title,
            ProgramDataAccountType::Legal => DataAccountType::Legal,
            ProgramDataAccountType::Tax => DataAccountType::Tax,
            ProgramDataAccountType::Miscellaneous => DataAccountType::Miscellaneous,
        }
    }
}

impl From<u8> for PolicyEngineVersion {
    fn from(version: u8) -> Self {
        match version {
            1 => PolicyEngineVersion::V1,
            _ => PolicyEngineVersion::V1,
        }
    }
}

impl From<u8> for ComparisionType {
    fn from(comparision_type: u8) -> Self {
        match comparision_type {
            1 => ComparisionType::Or,
            2 => ComparisionType::And,
            _ => ComparisionType::Or,
        }
    }
}

impl From<Policy> for PolicyType {
    fn from(ptype: Policy) -> Self {
        match ptype.policy_type {
            ProgramPolicyType::IdentityApproval => PolicyType::IdentityApproval,
            ProgramPolicyType::TransactionAmountLimit { .. } => PolicyType::TransactionAmountLimit,
            ProgramPolicyType::TransactionAmountVelocity { .. } => {
                PolicyType::TransactionAmountVelocity
            }
            ProgramPolicyType::TransactionCountVelocity { .. } => {
                PolicyType::TransactionCountVelocity
            }
        }
    }
}
