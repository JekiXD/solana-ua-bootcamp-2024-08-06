use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};


pub fn run(client: &RpcClient, user: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let blockhash = client.get_latest_blockhash()?;

    let token_mint_address = Pubkey::from_str("12S8ChkSRGMZgtJxLTGcuESE4WE2V2UN5bqwTyo9pNvB")?;
    let reciever = Pubkey::from_str("DrFMYRHBRMcYfcGLzHZ9AggJEqW5SdvQPebJWB9aLfSB")?;

    let (token_account_addresss, _bump) = Pubkey::find_program_address(
        &[
            &reciever.to_bytes(),
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

    let tx = Transaction::new_signed_with_payer(
        &[
            mint_to_instruction
        ], 
        Some(&user.pubkey()), 
        &[user], 
        blockhash
    );

    let signature = client.send_and_confirm_transaction(&tx)?;
    println!("Minted {0} tokens to: {1}", amount, reciever.to_string());
    println!("Transacton signature: {0}", signature);

    Ok(())
}