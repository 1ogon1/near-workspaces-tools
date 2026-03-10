//! Universal sandbox test context: worker, owner, contracts and tokens by name.
//!
//! Use [SandboxContext::builder] to create a context with contracts and/or fungible tokens.
//! Access by name: `ctx.contract("main")`, `ctx.token("usdc")`.

use near_workspaces::{Account, Contract, Worker, network::Sandbox, types::NearToken};
use serde_json::json;
use std::collections::HashMap;

use crate::CallExecutionDetailsExtension;
use crate::FungibleTokenMetadata;
use crate::U128Extensions;
use crate::deploy_contract;
use crate::types::TokenDecimals;

/// Default fungible token (NEP-148) WASM bundled for deployment.
pub const DEFAULT_FUNGIBLE_TOKEN_WASM: &[u8] = include_bytes!("fungible_token.wasm");

/// One contract to deploy: account prefix, balance, WASM.
#[derive(Clone)]
pub struct ContractSpec {
    pub prefix: String,
    pub balance: NearToken,
    pub wasm: Vec<u8>,
}

impl ContractSpec {
    pub fn new(prefix: impl Into<String>, balance: NearToken, wasm: impl AsRef<[u8]>) -> Self {
        Self {
            prefix: prefix.into(),
            balance,
            wasm: wasm.as_ref().to_vec(),
        }
    }
}

/// Deployed fungible token: contract and decimals for amount conversion.
#[derive(Clone)]
pub struct TokenInfo {
    pub contract: Contract,
    pub decimals: TokenDecimals,
}

impl TokenInfo {
    pub fn contract(&self) -> &Contract {
        &self.contract
    }
    pub fn decimals(&self) -> TokenDecimals {
        self.decimals
    }
}

/// Default sandbox test environment: worker, root account (owner), contracts and tokens by name.
///
/// Use [SandboxContext::builder] to add contracts and/or tokens, then [SandboxContextBuilder::build].
///
/// # Example (one contract)
///
/// ```ignore
/// let ctx = SandboxContext::builder()
///     .add_contract("main", NearToken::from_near(100), MAIN_WASM)
///     .build()
///     .await?;
/// let contract = ctx.contract("main").unwrap();
/// ```
///
/// # Example (tokens)
///
/// ```ignore
/// let ctx = SandboxContext::builder()
///     .add_token("usdc", 6)
///     .add_token("wrap", 24)
///     .build()
///     .await?;
/// ctx.token_expect("usdc").contract();
/// ctx.ft_balance_of("usdc", &account_id).await;
/// ```
pub struct SandboxContext {
    worker: Worker<Sandbox>,
    owner: Account,
    contracts: HashMap<String, Contract>,
    tokens: HashMap<String, TokenInfo>,
}

impl SandboxContext {
    /// Start building a sandbox context. Add contracts/tokens, then [SandboxContextBuilder::build].
    pub fn builder() -> SandboxContextBuilder {
        SandboxContextBuilder {
            specs: Vec::new(),
            token_specs: Vec::new(),
        }
    }

    /// Worker for the sandbox.
    pub fn worker(&self) -> &Worker<Sandbox> {
        &self.worker
    }

    /// Root account (owner) for the sandbox.
    pub fn owner(&self) -> &Account {
        &self.owner
    }

    /// Get a deployed contract by the name you passed to [SandboxContextBuilder::add_contract].
    pub fn contract(&self, name: &str) -> Option<&Contract> {
        self.contracts.get(name)
    }

    /// Get contract by name; panics if not found.
    pub fn contract_expect(&self, name: &str) -> &Contract {
        self.contract(name)
            .unwrap_or_else(|| panic!("contract not found: {}", name))
    }

    /// Number of deployed contracts.
    pub fn contract_count(&self) -> usize {
        self.contracts.len()
    }

    /// Iterate over (name, contract).
    pub fn contracts(&self) -> impl Iterator<Item = (&str, &Contract)> {
        self.contracts.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// Get deployed token by name.
    pub fn token(&self, name: &str) -> Option<&TokenInfo> {
        self.tokens.get(name)
    }

    /// Get token by name; panics if not found.
    pub fn token_expect(&self, name: &str) -> &TokenInfo {
        self.token(name)
            .unwrap_or_else(|| panic!("token not found: {}", name))
    }

    /// Number of deployed tokens.
    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }
}

/// Spec for deploying one fungible token (NEP-148).
#[derive(Clone)]
pub struct TokenSpec {
    pub name: String,
    pub decimals: TokenDecimals,
    pub wasm: Vec<u8>,
}

/// Builder for [SandboxContext]. Add contracts and/or tokens, then [build](SandboxContextBuilder::build).
pub struct SandboxContextBuilder {
    specs: Vec<ContractSpec>,
    token_specs: Vec<TokenSpec>,
}

impl SandboxContextBuilder {
    /// Add one contract to deploy. Call this 1, 2, or N times.
    pub fn add_contract(
        mut self,
        name: impl Into<String>,
        balance: NearToken,
        wasm: impl AsRef<[u8]>,
    ) -> Self {
        self.specs.push(ContractSpec::new(name, balance, wasm));
        self
    }

    /// Add a contract from a [ContractSpec].
    pub fn add_spec(mut self, spec: ContractSpec) -> Self {
        self.specs.push(spec);
        self
    }

    /// Add a fungible token (NEP-148) using [DEFAULT_FUNGIBLE_TOKEN_WASM].
    pub fn add_token(mut self, name: impl Into<String>, decimals: TokenDecimals) -> Self {
        self.token_specs.push(TokenSpec {
            name: name.into(),
            decimals,
            wasm: DEFAULT_FUNGIBLE_TOKEN_WASM.to_vec(),
        });
        self
    }

    /// Add a fungible token with custom WASM.
    pub fn add_token_with_wasm(
        mut self,
        name: impl Into<String>,
        decimals: TokenDecimals,
        wasm: impl AsRef<[u8]>,
    ) -> Self {
        self.token_specs.push(TokenSpec {
            name: name.into(),
            decimals,
            wasm: wasm.as_ref().to_vec(),
        });
        self
    }

    /// Create sandbox worker, deploy all contracts and tokens, init tokens, return [SandboxContext].
    pub async fn build(self) -> Result<SandboxContext, Box<dyn std::error::Error + Send + Sync>> {
        let worker = near_workspaces::sandbox().await?;
        let owner = worker.root_account().unwrap().clone();

        let mut contracts = HashMap::new();
        for spec in self.specs {
            let contract = deploy_contract(&worker, &spec.prefix, spec.balance, &spec.wasm).await;
            contracts.insert(spec.prefix, contract);
        }

        let mut tokens = HashMap::new();
        for spec in self.token_specs {
            let contract =
                deploy_contract(&worker, &spec.name, NearToken::from_near(100), &spec.wasm).await;
            let metadata = FungibleTokenMetadata {
                spec: "ft-1.0.0".to_string(),
                name: format!("Token {}", spec.name),
                symbol: spec.name.to_uppercase(),
                decimals: spec.decimals,
                icon: None,
                reference: None,
                reference_hash: None,
            };
            let total_supply = 10_000_000u128.to_decimals(spec.decimals).to_string();
            contract
                .call("new")
                .args_json(json!({
                    "owner_id": contract.id(),
                    "total_supply": total_supply,
                    "metadata": metadata,
                }))
                .transact()
                .await
                .assert_success("init token");
            tokens.insert(
                spec.name.clone(),
                TokenInfo {
                    contract,
                    decimals: spec.decimals,
                },
            );
        }

        Ok(SandboxContext {
            owner,
            worker,
            tokens,
            contracts,
        })
    }
}
