use near_workspaces::result::{ExecutionFinalResult, ViewResultDetails};
use near_workspaces::types::NearToken;
use serde::{Deserialize, Serialize};

pub(crate) type ViewExecutionResult = near_workspaces::Result<ViewResultDetails>;
pub(crate) type CallExecutionResult = near_workspaces::Result<ExecutionFinalResult>;

pub type Timestamp = u64;
pub type TokenDecimals = u8;

/// One yoctoNEAR, used for attached deposit in FT/NEP-148 calls.
pub const ONE_YOCTO: NearToken = NearToken::from_yoctonear(1);

/// NEP-148 fungible token metadata for contract `new` init.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FungibleTokenMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub decimals: TokenDecimals,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_hash: Option<String>,
}

/// NEP-148 storage balance bounds (view result). Min/max are strings in JSON (yoctoNEAR).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageBalanceBounds {
    pub min: String,
    pub max: Option<String>,
}

impl StorageBalanceBounds {
    /// Minimum storage deposit in yoctoNEAR.
    pub fn min_yocto(&self) -> u128 {
        self.min
            .parse()
            .unwrap_or_else(|_| panic!("invalid storage_balance_bounds min: {}", self.min))
    }
}
