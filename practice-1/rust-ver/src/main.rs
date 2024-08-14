use std::env;

use serde_json::Value;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::{SeedDerivable, Signer}};
use dotenv::dotenv;


fn generate_keypair() {
    let keypair = Keypair::new();
    println!("Public key: {0}", keypair.pubkey());
    println!("Private key: {0:?}", keypair.secret().as_bytes());
}

fn load_keypairs() -> Result<Keypair, Box<dyn std::error::Error>> {
    let secret_key = env::var("SECRET_KEY");
    if let Err(err) = secret_key {
        return Err(std::format!("No secret key found in the environment: {0}", err).into());
    }

    let secret_key = secret_key.unwrap();

    if let Ok(v) = serde_json::from_str::<Vec<u8>>(&secret_key) {
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

fn check_balance(keypair: &Keypair) -> Result<(), Box<dyn std::error::Error>>{
    let rpc_url = String::from("https://api.devnet.solana.com");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let balance = client.get_balance(&keypair.pubkey())?;
    let balance_in_sol = balance as f64 / LAMPORTS_PER_SOL as f64;
    println!("Current balance: {0} lamports", balance);
    println!("Current balance: {0} sol", balance_in_sol);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();

    //generate_keypair();
    let keypair = load_keypairs()?;
    println!("Recovered public key: {0}", keypair.pubkey());
    check_balance(&keypair)?;

    Ok(())
}
