use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use spl_associated_token_account::instruction::create_associated_token_account;

pub fn run(client: &RpcClient, user1: &Keypair, user2: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let blockhash = client.get_latest_blockhash()?;
    let token_mint_address = Pubkey::from_str("EmPSMn1jqA9JEWPhji6TJd23t7qUCkkNAZY5URvknjXY")?;

    let instruction = create_associated_token_account(
        &user1.pubkey(), 
        &user2.pubkey(), 
        &token_mint_address, 
        &spl_token::ID
    );

    let tx = Transaction::new_signed_with_payer(
        &[
            instruction
        ], 
        Some(&user1.pubkey()), 
        &[user1], 
        blockhash
    );

    let signature = client.send_and_confirm_transaction(&tx)?;

    println!("Created and initialized token account");
    println!("Transaction signature: {0}", signature);

    Ok(())
}