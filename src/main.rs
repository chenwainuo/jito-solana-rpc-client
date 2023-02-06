use std::env;

use jito_solana_rpc_client::{JitoRpcClient, RpcSimulateBundleConfig, VersionedBundle};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::transaction::Transaction;
use solana_sdk::{pubkey, system_instruction};
use solana_sdk::{pubkey::Pubkey, transaction::VersionedTransaction};

const TEST_USER: Pubkey = pubkey!("2AQdpHJ2JpcEgPiATUXjQxA8QmafFegfQwSLWSprPicm"); // Coinbase 2 wallet, for plenty of SOL
const TIP_ACCOUNT: Pubkey = pubkey!("96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5");

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let url = args[1].clone();
    println!("url: {}", url);

    let client = RpcClient::new(url);
    let jito_client = JitoRpcClient(client);

    // A dumb bundle with 1 self transfer tx, then the tip tx
    let transactions = vec![
        Transaction::new_with_payer(
            &[system_instruction::transfer(
                &TEST_USER, &TEST_USER, 1_000_000,
            )],
            Some(&TEST_USER),
        ),
        Transaction::new_with_payer(
            &[system_instruction::transfer(
                &TEST_USER,
                &TIP_ACCOUNT,
                1_000_000,
            )],
            Some(&TEST_USER),
        ),
    ];
    let versioned_bundle = VersionedBundle {
        transactions: transactions
            .into_iter()
            .map(VersionedTransaction::from)
            .collect(),
    };
    let result = jito_client
        .simulate_bundle_with_config(
            &versioned_bundle,
            RpcSimulateBundleConfig {
                pre_execution_accounts_configs: vec![None; versioned_bundle.transactions.len()],
                post_execution_accounts_configs: vec![None; versioned_bundle.transactions.len()],
                skip_sig_verify: true,
                replace_recent_blockhash: true,
                ..RpcSimulateBundleConfig::default()
            },
        )
        .await
        .unwrap();
    println!("{:#?}", result);
}
