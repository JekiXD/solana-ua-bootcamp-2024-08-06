use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::Transaction;

pub fn run(client: &RpcClient, user: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let blockhash = client.get_latest_blockhash()?;

    let encoded = std::fs::read("transaction.dat")?;
    let mut tx: Transaction = bincode::deserialize(&encoded)?;

    tx.partial_sign(
        &[user], 
        blockhash
    );

    let signature = client.send_and_confirm_transaction(&tx)?;

    println!("Tokens are sent!");
    println!("Signature: {0}", signature);

    Ok(())
}