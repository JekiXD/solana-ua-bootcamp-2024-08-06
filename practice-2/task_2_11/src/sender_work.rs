use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use solana_client::nonce_utils;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;

pub fn run(client: &RpcClient, user: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let user2 = Pubkey::from_str("DrFMYRHBRMcYfcGLzHZ9AggJEqW5SdvQPebJWB9aLfSB")?;
    let token_mint_address = Pubkey::from_str("12S8ChkSRGMZgtJxLTGcuESE4WE2V2UN5bqwTyo9pNvB")?;
    let nonce_pubkey = Pubkey::from_str("6dhqDy5E3h4jCes4HPvKXh2RYBsPicET8XdyieNLWg7V")?;

    let nonce_acc = nonce_utils::get_account(&client, &nonce_pubkey)?;
    let nonce_data = nonce_utils::data_from_account(&nonce_acc)?;
    let blockhash = nonce_data.blockhash();

    let advance_nonce_instruction = system_instruction::advance_nonce_account(
        &nonce_pubkey, 
        &user.pubkey()
    );

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
        &[
            advance_nonce_instruction,
            mint_to_instruction
        ], 
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
    println!("Time now: {0}", chrono::Local::now().format("%Y-%m-%d/%H:%M:%S"));

    Ok(())
}