use std::env;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::{SeedDerivable, Signer};
use solana_sdk::transaction::Transaction;

mod sender_work;
mod reciever_work;
mod create_nonce;

fn load_keypairs(secret_key: &str) -> Result<Keypair, Box<dyn std::error::Error>> {
    if let Ok(v) = serde_json::from_str::<Vec<u8>>(secret_key) {
        match Keypair::from_seed(v.as_slice()) {
            Ok(keypair) => {
                return Ok(keypair);
            },
            Err(err) => {
                return Err(std::format!("Failed to recover KeyPair from secret key: {0}", err).into());
            }
        }
    }

    Err("Invalid secret key".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let secret1 = env::var("SECRET_KEY")?;
    let secret2 = env::var("SECRET_KEY2")?;
    let user1 = load_keypairs(&secret1)?;
    let user2 = load_keypairs(&secret2)?;
    let devnet_url = String::from("https://api.devnet.solana.com");
    let client = RpcClient::new_with_commitment(devnet_url, CommitmentConfig::confirmed());

    // create_nonce::run(&client, &user1)?;
    // sender_work::run(&client, &user1)?;
    reciever_work::run(&client, &user2)?;

    Ok(())
}
