use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::{signature::Signer, transaction::Transaction};

use crate::Miner;
use gemm_api::consts::TREASURY_ADDRESS;

impl Miner {
    pub async fn initialize(&self) {
        // Return early if program is already initialized
        if self.rpc_client.get_account(&TREASURY_ADDRESS).await.is_ok() {
            return;
        }

        // Submit initialize tx
        let blockhash = self.rpc_client.get_latest_blockhash().await.unwrap();
        let ix = gemm_api::instruction::initialize(self.signer().pubkey());
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.signer().pubkey()),
            &[&self.signer()],
            blockhash,
        );

        // Send transaction with skip_preflight
        let signature = self.rpc_client.send_transaction_with_config(
            &tx,
            RpcSendTransactionConfig {
                skip_preflight: true,
                ..RpcSendTransactionConfig::default()
            },
        ).await.unwrap();

        // Confirm the transaction
        let res = self.rpc_client.confirm_transaction(&signature).await;
        println!("{:?}", res);
    }
}

