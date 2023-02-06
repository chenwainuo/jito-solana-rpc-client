use solana_account_decoder::UiAccountEncoding;
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    slot_history::Slot,
};
use solana_transaction_status::UiTransactionEncoding;

#[derive(Debug, PartialEq, Default, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcBundleRequest {
    pub encoded_transactions: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SimulationSlotConfig {
    /// Simulate on top of bank with the provided commitment.
    Commitment(CommitmentConfig),

    /// Simulate on the provided slot's bank.
    Slot(Slot),

    /// Simulates on top of the RPC's highest slot's bank i.e. the working bank.
    Tip,
}

impl Default for SimulationSlotConfig {
    fn default() -> Self {
        Self::Commitment(CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcSimulateTransactionAccountsConfig {
    pub encoding: Option<UiAccountEncoding>,
    pub addresses: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcSimulateBundleConfig {
    /// Gives the state of accounts pre/post transaction execution.
    /// The length of each of these must be equal to the number transactions.   
    pub pre_execution_accounts_configs: Vec<Option<RpcSimulateTransactionAccountsConfig>>,
    pub post_execution_accounts_configs: Vec<Option<RpcSimulateTransactionAccountsConfig>>,

    /// Specifies the encoding scheme of the contained transactions.
    pub transaction_encoding: Option<UiTransactionEncoding>,

    /// Specifies the bank to run simulation against.
    #[serde(flatten)]
    pub simulation_bank: Option<SimulationSlotConfig>,

    /// Opt to skip sig-verify for faster performance.
    #[serde(default)]
    pub skip_sig_verify: bool,

    /// Replace recent blockhash to simulate old transactions without resigning.
    #[serde(default)]
    pub replace_recent_blockhash: bool,
}
