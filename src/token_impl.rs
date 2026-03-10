//! Fungible token (NEP-148) helpers for [SandboxContext].

use near_workspaces::{Account, types::NearToken};
use serde::Serialize;
use serde_json::json;

use crate::CallExecutionDetailsExtension;
use crate::CallExecutionResult;
use crate::ONE_YOCTO;
use crate::StorageBalanceBounds;
use crate::U128Extensions;
use crate::ViewResultDetailsExtension;

use crate::SandboxContext;

impl SandboxContext {
    /// Initialize an already-deployed token contract with custom metadata.
    /// Use this only if you deployed a token via [crate::SandboxContextBuilder::add_token_with_wasm]
    /// and did not init it in the builder (e.g. custom init flow).
    // pub async fn init_token(
    //     &self,
    //     token_name: &str,
    //     total_supply_human: u128,
    //     metadata: crate::FungibleTokenMetadata,
    // ) {
    //     let info = self.token_expect(token_name);
    //     let total_supply = total_supply_human.to_decimals(info.decimals()).to_string();
    //     info.contract()
    //         .call("new")
    //         .args_json(json!({
    //             "owner_id": info.contract().id(),
    //             "total_supply": total_supply,
    //             "metadata": metadata,
    //         }))
    //         .transact()
    //         .await
    //         .assert_success(&format!("init token '{token_name}'"));
    // }

    pub async fn storage_deposit(&self, token_name: &str, account_id: &near_workspaces::AccountId) {
        let info = self.token_expect(token_name);
        let bounds = self.storage_balance_bounds(token_name).await;
        info.contract()
            .call("storage_deposit")
            .args_json(json!({ "account_id": account_id }))
            .deposit(NearToken::from_yoctonear(bounds.min_yocto()))
            .transact()
            .await
            .assert_success(&format!(
                "storage deposit '{account_id}' for token '{token_name}'"
            ));
    }

    pub async fn storage_unregister(&self, predecessor: &Account, token_name: &str) {
        let info = self.token_expect(token_name);
        predecessor
            .call(info.contract().id(), "storage_unregister")
            .args_json(json!({ "force": true }))
            .deposit(ONE_YOCTO)
            .transact()
            .await
            .assert_success(&format!(
                "storage unregister for token '{token_name}', account {:?}",
                predecessor.id()
            ));
    }

    pub async fn ft_transfer(
        &self,
        token_name: &str,
        receiver_id: &near_workspaces::AccountId,
        amount_human: u128,
    ) {
        let info = self.token_expect(token_name);
        let amount = amount_human.to_decimals(info.decimals()).to_string();

        info.contract()
            .call("ft_transfer")
            .args_json(json!({
                "receiver_id": receiver_id,
                "amount": amount,
            }))
            .deposit(ONE_YOCTO)
            .transact()
            .await
            .assert_success(&format!("ft_transfer token '{token_name}'"));
    }

    pub async fn ft_transfer_call<T>(
        &self,
        predecessor: &Account,
        receiver_id: &near_workspaces::AccountId,
        token_name: &str,
        amount_human: u128,
        msg: T,
    ) -> CallExecutionResult
    where
        T: Serialize,
    {
        let info = self.token_expect(token_name);
        let amount = amount_human.to_decimals(info.decimals()).to_string();

        predecessor
            .call(info.contract().id(), "ft_transfer_call")
            .args_json(json!({
                "receiver_id": receiver_id,
                "amount": amount,
                "msg": serde_json::to_string(&msg).unwrap(),
            }))
            .deposit(ONE_YOCTO)
            .max_gas()
            .transact()
            .await
    }

    /// Returns balance as u128 (raw amount in contract units).
    pub async fn ft_balance_of(
        &self,
        token_name: &str,
        account_id: &near_workspaces::AccountId,
    ) -> u128 {
        let info = self.token_expect(token_name);
        let raw: String = info
            .contract()
            .view("ft_balance_of")
            .args_json(json!({ "account_id": account_id }))
            .await
            .get_result(&format!("ft_balance_of token '{token_name}'"));
        raw.parse::<u128>()
            .unwrap_or_else(|_| panic!("ft_balance_of: invalid balance string '{raw}'"))
    }

    /// Returns balance in human units (stripped by token decimals).
    pub async fn ft_balance_of_human(
        &self,
        token_name: &str,
        account_id: &near_workspaces::AccountId,
    ) -> u128 {
        let raw = self.ft_balance_of(token_name, account_id).await;
        let decimals = self.token_expect(token_name).decimals();
        raw.strip_decimals(decimals)
    }

    /// Returns raw balance (with decimals) as string from the contract.
    pub async fn ft_balance_of_raw(
        &self,
        token_name: &str,
        account_id: &near_workspaces::AccountId,
    ) -> String {
        let info = self.token_expect(token_name);
        info.contract()
            .view("ft_balance_of")
            .args_json(json!({ "account_id": account_id }))
            .await
            .get_result(&format!("ft_balance_of token '{token_name}'"))
    }

    pub async fn storage_balance_bounds(&self, token_name: &str) -> StorageBalanceBounds {
        let info = self.token_expect(token_name);
        info.contract()
            .view("storage_balance_bounds")
            .await
            .get_result(&format!("storage_balance_bounds token '{token_name}'"))
    }
}
