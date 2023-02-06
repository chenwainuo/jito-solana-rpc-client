use bincode::serialize;
use config::{RpcBundleRequest, SimulationSlotConfig};
use response::RpcSimulateBundleResult;
use serde_json::json;
use solana_client::{
    client_error::{ClientErrorKind, Result as ClientResult},
    nonblocking::rpc_client::RpcClient,
    rpc_request::RpcRequest,
    rpc_response::RpcResult,
};

pub use config::RpcSimulateBundleConfig;
pub use error::BundleExecutionError;
pub use solana_sdk::transaction::VersionedTransaction;
use solana_transaction_status::UiTransactionEncoding;

mod config;
mod error;
mod response;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, PartialEq, Default, Eq, Clone, Serialize, Deserialize)]
pub struct VersionedBundle {
    pub transactions: Vec<VersionedTransaction>,
}

pub struct JitoRpcClient(pub RpcClient);

impl JitoRpcClient {
    // TODO: batch_simulate_bundle and batch_simulate_bundle_with_config

    pub async fn simulate_bundle(
        &self,
        bundle: &VersionedBundle,
    ) -> RpcResult<RpcSimulateBundleResult> {
        self.simulate_bundle_with_config(
            bundle,
            RpcSimulateBundleConfig {
                simulation_bank: Some(SimulationSlotConfig::Commitment(self.0.commitment())),
                pre_execution_accounts_configs: vec![None; bundle.transactions.len()],
                post_execution_accounts_configs: vec![None; bundle.transactions.len()],
                ..RpcSimulateBundleConfig::default()
            },
        )
        .await
    }

    pub async fn simulate_bundle_with_config(
        &self,
        bundle: &VersionedBundle,
        config: RpcSimulateBundleConfig,
    ) -> RpcResult<RpcSimulateBundleResult> {
        let transaction_encoding = if let Some(enc) = config.transaction_encoding {
            enc
        } else {
            // private method, only required for ancient clusters, hardcode instead
            // self.default_cluster_transaction_encoding().await?
            UiTransactionEncoding::Base64
        };
        let simulation_bank = Some(config.simulation_bank.unwrap_or_default());

        let encoded_transactions = bundle
            .transactions
            .iter()
            .map(|tx| serialize_and_encode::<VersionedTransaction>(tx, transaction_encoding))
            .collect::<ClientResult<Vec<String>>>()?;
        let rpc_bundle_request = RpcBundleRequest {
            encoded_transactions,
        };

        let config = RpcSimulateBundleConfig {
            transaction_encoding: Some(transaction_encoding),
            simulation_bank,
            ..config
        };

        self.0
            .send(
                RpcRequest::Custom {
                    method: "simulateBundle",
                },
                json!([rpc_bundle_request, config]),
            )
            .await
    }
}

fn serialize_and_encode<T>(input: &T, encoding: UiTransactionEncoding) -> ClientResult<String>
where
    T: serde::ser::Serialize,
{
    let serialized = serialize(input)
        .map_err(|e| ClientErrorKind::Custom(format!("Serialization failed: {}", e)))?;
    let encoded = match encoding {
        UiTransactionEncoding::Base58 => bs58::encode(serialized).into_string(),
        #[allow(deprecated)]
        UiTransactionEncoding::Base64 => base64::encode(serialized),
        _ => {
            return Err(ClientErrorKind::Custom(format!(
                "unsupported encoding: {}. Supported encodings: base58, base64",
                encoding
            ))
            .into())
        }
    };
    Ok(encoded)
}
