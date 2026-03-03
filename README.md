# near-workspaces-tools

Tools and extension traits for [near-workspaces](https://github.com/near/near-workspaces-rs) — testing and simulation for NEAR Protocol smart contracts.

## Features

- **View results**: `get_result<T>()` to deserialize view call results, optional logging
- **Call results**: `assert_success()`, `assert_failure(action, expected_message)` for call outcomes
- **Logs & outcomes**: `show_logs()`, `show_outcomes()`, `assert_log(action, expected_substring)`
- **Block time**: `get_block_timestamp(worker)` for the block that included the transaction
- **Token helpers**: `to_decimals()` / `strip_decimals()` for `u128` amounts, `sec_to_ms()` / `ns_to_sec()` for timestamps

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
near-workspaces-tools = "1.0"
```

With optional features (see below):

```toml
near-workspaces-tools = { version = "1.0", features = ["show_logs", "show_default_output"] }
```

## Usage

```rust
use near_workspaces_tools::*;
use near_workspaces::Worker;

// View call: deserialize result and optionally print logs
let result: YourViewResult = worker
    .view("account_id", "view_method")
    .args_json(())
    .await?
    .get_result("view_method");

// Call: assert success and get result
let success = worker
    .call("account_id", "call_method")
    .args_json(())
    .await?
    .assert_success("call_method");
let value: YourCallResult = success.get_result();

// Call: assert expected failure message
worker
    .call("account_id", "failing_method")
    .args_json(())
    .await?
    .assert_failure("failing_method", "expected error substring");

// Assert a specific log line
worker
    .call("account_id", "method")
    .args_json(())
    .await?
    .assert_success("method")
    .assert_log("method", "Expected log message");

// Block timestamp for the transaction
let ts = success.get_block_timestamp(&worker).await;
```

## Cargo features

| Feature | Description |
|--------|-------------|
| `show_logs` | Print contract logs for view/call results when present |
| `show_default_output` | Print transaction boundaries and outcomes by default |

Enable when debugging tests:

```bash
cargo test --features show_logs,show_default_output
```

## License

MIT
