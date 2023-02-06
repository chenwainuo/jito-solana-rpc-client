Provides `JitoRpcClient` struct over RpcClient to perform jito-solana specific JSON RPC calls without having to switch crates

jito-solana `simulate_bundle_result_with_config`

https://github.com/jito-foundation/jito-solana/blob/d43ba1d14e9069b535833fda5372af0b883feeaa/client/src/nonblocking/rpc_client.rs#L1482-L1514

This is to avoid conflicts which would require replacing solana-client/sdk all the way down
