use std::fmt::Debug;

use async_trait::async_trait;
use near_workspaces::{
    Worker,
    network::Sandbox,
    result::{ExecutionFailure, ExecutionSuccess, ViewResultDetails},
};
use serde::de::DeserializeOwned;

use crate::{
    CallExecutionDetailsExtension, CallExecutionDetailsHelper, CallExecutionResult,
    GenericExtension, SHOW_DEFAULT_OUTPUT, SHOW_LOGS, Timestamp, ViewExecutionResult,
    ViewResultDetailsExtension, ViewResultDetailsHelper,
};

impl ViewResultDetailsExtension for ViewExecutionResult {
    fn get_result<T: DeserializeOwned>(self, action: &str) -> T {
        match self {
            Ok(data) => {
                if SHOW_DEFAULT_OUTPUT {
                    println!("=== VIEW '{action}' TRANSACTION ===\n");
                }

                data.show_logs();
                data.json::<T>().expect("Can't deserialize view result")
            }
            Err(error) => {
                println!("=== VIEW '{action}' TRANSACTION ===\n");

                panic!("\n---> Unhandled exception has occurred.\n{:#?}", error);
            }
        }
    }
}

impl ViewResultDetailsHelper for ViewResultDetails {
    fn show_logs(&self) {
        if SHOW_LOGS && !self.logs.is_empty() {
            println!("{:#?}", self.logs);
        }
    }
}

impl CallExecutionDetailsExtension for CallExecutionResult {
    fn assert_failure(self, action: &str, expect: &str) {
        match self {
            Ok(execution) => match execution.into_result() {
                Ok(result) => {
                    let mut has_error = false;

                    result.receipt_failures().into_iter().for_each(|receipt| {
                        receipt
                            .clone()
                            .into_result()
                            .map_err(|error| {
                                if error
                                    .into_inner()
                                    .expect("Error: get inner error data")
                                    .to_string()
                                    .contains(expect)
                                {
                                    has_error = true;
                                }
                            })
                            .ok();
                    });

                    if SHOW_DEFAULT_OUTPUT || !has_error {
                        println!("=== BEGIN '{action}' TRANSACTION ===");
                    }

                    result.show_logs();
                    result.show_outcomes(!has_error);

                    assert!(
                        has_error,
                        "\n---> Expect error: {expect}.\n---> Found: {:#?}",
                        result.failures()
                    );

                    if SHOW_DEFAULT_OUTPUT {
                        println!("\n---> Got error as expected: {expect}\n");
                        println!("=== END '{action}' TRANSACTION ===\n");
                    }
                }
                Err(error) => {
                    let has_error = error.to_string().contains(expect);

                    if SHOW_DEFAULT_OUTPUT || !has_error {
                        println!("=== BEGIN '{action}' TRANSACTION ===");
                    }

                    error.show_logs();
                    error.show_outcomes(!has_error);

                    assert!(
                        has_error,
                        "\n---> Expect error: {expect}.\n---> Found: {:#?}",
                        error.failures()
                    );

                    if SHOW_DEFAULT_OUTPUT {
                        println!("\n---> Got error as expected: {expect}\n");
                        println!("=== END '{action}' TRANSACTION ===\n");
                    }
                }
            },
            Err(error) => {
                let has_error = error.to_string().contains(expect);

                if SHOW_DEFAULT_OUTPUT || !has_error {
                    println!("=== BEGIN '{action}' TRANSACTION ===");
                }

                assert!(
                    has_error,
                    "\n---> Expect error: {expect}.\n---> Found: {:#?}",
                    error
                );

                if SHOW_DEFAULT_OUTPUT {
                    println!("\n---> Got error as expected: {expect}\n");
                    println!("=== END '{action}' TRANSACTION ===\n");
                }
            }
        }
    }

    fn assert_success(self, action: &str) -> ExecutionSuccess {
        match self {
            Ok(execution) => match execution.into_result() {
                Ok(result) => {
                    let has_error = !result.receipt_failures().is_empty();

                    if SHOW_DEFAULT_OUTPUT || has_error {
                        println!("=== BEGIN '{action}' TRANSACTION ===");
                    }

                    result.show_logs();
                    result.show_outcomes(has_error);

                    assert!(
                        !has_error,
                        "\n---> Got error: {:#?}",
                        result.receipt_failures()
                    );

                    if SHOW_DEFAULT_OUTPUT {
                        println!("=== END '{action}' TRANSACTION ===\n");
                    }

                    result
                }
                Err(error) => {
                    println!("=== BEGIN '{action}' TRANSACTION ===");
                    panic!(
                        "\n---> Inner unhandled exception has occurred.\n---> Error: {:#?}",
                        error.failures()
                    );
                }
            },
            Err(error) => {
                println!("=== BEGIN '{action}' TRANSACTION ===");
                panic!(
                    "\n---> Unhandled exception has occurred.\n---> Error: {:#?}",
                    error
                );
            }
        }
    }
}

#[async_trait]
impl CallExecutionDetailsHelper for ExecutionSuccess {
    fn get_result<T: DeserializeOwned>(self) -> T {
        self.json().expect("Error: can't unwrap result")
    }

    fn show_logs(&self) {
        if SHOW_LOGS && !self.logs().is_empty() {
            println!("{:#?}", self.logs());
        }
    }

    fn show_outcomes(&self, show: bool) {
        if SHOW_DEFAULT_OUTPUT || show {
            println!("=== OUTCOMES {:#?}", self.receipt_outcomes());
            println!("=== GAS BURNT {} TGas ===", self.total_gas_burnt.as_tgas());
        }
    }

    fn assert_log(self, action: &str, expect: &str) -> Self {
        let logs = self.logs();

        if !logs.contains(&expect) {
            println!("=== ASSERT LOG FOR '{action}' ===");
            panic!("\n---> Expect: '{expect}'.\n---> Found: {:#?}", logs);
        }

        self
    }

    async fn get_block_timestamp(&self, worker: &Worker<Sandbox>) -> Timestamp {
        let block_hash = self
            .receipt_outcomes()
            .first()
            .expect("Can't get block")
            .block_hash;

        worker
            .view_block()
            .block_hash(block_hash)
            .await
            .expect("Can't find block by block hash")
            .timestamp()
    }
}

#[async_trait]
impl CallExecutionDetailsHelper for ExecutionFailure {
    fn show_logs(&self) {
        if SHOW_LOGS && !self.logs().is_empty() {
            println!("{:#?}", self.logs());
        }
    }

    fn show_outcomes(&self, show: bool) {
        if SHOW_DEFAULT_OUTPUT || show {
            println!("=== OUTCOMES {:#?}", self.receipt_outcomes());
            println!("=== GAS BURNT {} TGas ===", self.total_gas_burnt.as_tgas());
        }
    }

    fn get_result<T: DeserializeOwned>(self) -> T {
        unimplemented!("Failed transaction has no result")
    }

    fn assert_log(self, _action: &str, _expect: &str) -> Self {
        unimplemented!("No assertion log in failed transaction")
    }

    async fn get_block_timestamp(&self, _worker: &Worker<Sandbox>) -> Timestamp {
        unimplemented!("Failed transaction has no block timestamp")
    }
}

impl<T> GenericExtension<T> for T
where
    T: PartialEq + Debug,
{
    fn assert_eq(self, expect: T, message: &str) -> T {
        assert_eq!(self, expect, "{}", message);
        self
    }
}
