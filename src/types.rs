use near_workspaces::result::{ExecutionFinalResult, ViewResultDetails};

pub(crate) type ViewExecutionResult = near_workspaces::Result<ViewResultDetails>;
pub(crate) type CallExecutionResult = near_workspaces::Result<ExecutionFinalResult>;

pub type Timestamp = u64;
pub type TokenDecimals = u8;
