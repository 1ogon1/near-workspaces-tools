use near_workspaces::{Account, Contract, Worker, network::Sandbox, types::NearToken};

pub async fn create_account(
    worker: &Worker<Sandbox>,
    account_prefix: &str,
    initial_balance: NearToken,
) -> Account {
    worker
        .root_account()
        .unwrap()
        .create_subaccount(account_prefix)
        .initial_balance(initial_balance)
        .transact()
        .await
        .unwrap()
        .result
}

pub async fn deploy_contract(
    worker: &Worker<Sandbox>,
    account_prefix: &str,
    initial_balance: NearToken,
    wasm: &[u8],
) -> Contract {
    create_account(worker, account_prefix, initial_balance)
        .await
        .deploy(wasm)
        .await
        .unwrap()
        .result
}
