use std::env;
use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::{SeedDerivable, Signer};

mod send_sol;
mod create_token_mint;
mod create_token_account;
mod mint_tokens;
mod create_token_metadata;

const MINT_SIZE: u64 = 82;

fn generate_keypair() -> Keypair {
    let keypair = Keypair::new();
    keypair
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

fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();

    let user = load_keypairs()?;
    let devnet_url = String::from("https://api.devnet.solana.com");
    let client = RpcClient::new_with_commitment(devnet_url, CommitmentConfig::confirmed());

    //send_sol::run(&client, &user)?;
    //create_token_mint::run(&client, &user)?;
    //create_token_account::run(&client, &user)?;
    //mint_tokens::run(&client, &user)?;
    create_token_metadata::run(&client, &user)?;
    Ok(())
}
