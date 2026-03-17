use std::fmt::Debug;

use async_trait::async_trait;
use near_workspaces::{Worker, network::Sandbox, result::ExecutionSuccess};
use serde::de::DeserializeOwned;

use crate::{Timestamp, TokenDecimals};

pub trait ViewResultDetailsExtension {
    fn get_result<T: DeserializeOwned>(self, action: &str) -> T;
}

pub trait ViewResultDetailsHelper {
    fn show_logs(&self);
}

pub trait CallExecutionDetailsExtension {
    fn assert_failure(self, action: &str, expect: &str);
    fn assert_success(self, action: &str) -> ExecutionSuccess;
}

#[async_trait]
pub trait CallExecutionDetailsHelper {
    fn show_logs(&self);
    fn show_outcomes(&self, show: bool);
    fn get_result<T: DeserializeOwned>(self) -> T;
    fn assert_log(self, action: &str, expect: &str) -> Self;
    async fn get_block_timestamp(&self, worker: &Worker<Sandbox>) -> Timestamp;
}

pub trait TimestampExtension {
    fn sec_to_ms(self) -> Timestamp;
    fn ns_to_sec(self) -> Timestamp;
}

pub trait U128Extensions {
    fn to_decimals(self, decimals: TokenDecimals) -> u128;
    fn strip_decimals(self, decimals: TokenDecimals) -> u128;
}

pub trait GenericExtension<T: PartialEq + Debug> {
    fn assert_eq(self, expect: T, message: &str) -> T;
}
