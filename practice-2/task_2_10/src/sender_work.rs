use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

pub fn run(client: &RpcClient, user: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let blockhash = client.get_latest_blockhash()?;
    let user2 = Pubkey::from_str("DrFMYRHBRMcYfcGLzHZ9AggJEqW5SdvQPebJWB9aLfSB")?;
    let token_mint_address = Pubkey::from_str("12S8ChkSRGMZgtJxLTGcuESE4WE2V2UN5bqwTyo9pNvB")?;

    let (token_account_addresss, _bump) = Pubkey::find_program_address(
        &[
            &user2.to_bytes(),
            &spl_token::ID.to_bytes(),
            &token_mint_address.to_bytes()
        ], 
        &spl_associated_token_account::ID
    );

    let amount = 10000;
    let mint_to_instruction = spl_token::instruction::mint_to(
        &spl_token::ID, 
        &token_mint_address, 
        &token_account_addresss, 
        &user.pubkey(),
        &[],
        amount
    )?;

    let mut tx = Transaction::new_with_payer(
        &[mint_to_instruction], 
        Some(&user2)
    );

    tx.partial_sign(
        &[user], 
        blockhash
    );

    let encoded = bincode::serialize(&tx)?;

    let mut transaction_file = File::create("transaction.dat")?;
    transaction_file.write_all(&encoded)?;

    println!("Transaction is partially signed and serialized!");

    Ok(())
}