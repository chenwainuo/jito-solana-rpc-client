use solana_account_decoder::UiAccount;
use solana_transaction_status::UiTransactionReturnData;

use crate::{error::JitoTransactionError, BundleExecutionError};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RpcBundleSimulationSummary {
    /// error and offending transaction signature
    Failed {
        error: BundleExecutionError,
        tx_signature: String,
    },
    Succeeded,
}

// TODO: consolidate with [RpcSimulateTransactionResult]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcSimulateBundleTransactionResult {
    pub err: Option<JitoTransactionError>,
    pub logs: Option<Vec<String>>,
    pub pre_execution_accounts: Option<Vec<UiAccount>>,
    pub post_execution_accounts: Option<Vec<UiAccount>>,
    pub units_consumed: Option<u64>,
    pub return_data: Option<UiTransactionReturnData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpcSimulateBundleResult {
    pub summary: RpcBundleSimulationSummary,
    pub transaction_results: Vec<RpcSimulateBundleTransactionResult>,
}
