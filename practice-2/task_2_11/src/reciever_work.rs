use std::str::FromStr;
use solana_client::nonce_utils;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::Transaction;

pub fn run(client: &RpcClient, user: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let nonce_pubkey = Pubkey::from_str("6dhqDy5E3h4jCes4HPvKXh2RYBsPicET8XdyieNLWg7V")?;
    
    let nonce_acc = nonce_utils::get_account(&client, &nonce_pubkey)?;
    let nonce_data = nonce_utils::data_from_account(&nonce_acc)?;
    let blockhash = nonce_data.blockhash();

    let encoded = std::fs::read("transaction.dat")?;
    let mut tx: Transaction = bincode::deserialize(&encoded)?;

    tx.partial_sign(
        &[user], 
        blockhash
    );

    let signature = client.send_and_confirm_transaction(&tx)?;

    println!("Tokens are sent!");
    println!("Signature: {0}", signature);
    println!("Time now: {0}", chrono::Local::now().format("%Y-%m-%d/%H:%M:%S"));

    Ok(())
}