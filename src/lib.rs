mod core_impl;
mod traits;
mod types;

pub use traits::*;
pub use types::*;

pub const SHOW_LOGS: bool = cfg!(feature = "show_logs");
pub const SHOW_DEFAULT_OUTPUT: bool = cfg!(feature = "show_default_output");
