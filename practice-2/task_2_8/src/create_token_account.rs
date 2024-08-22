use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::{self, Keypair}, signer::Signer, system_instruction, transaction::Transaction};
use spl_associated_token_account::{get_associated_token_address, instruction::create_associated_token_account};


pub fn run(client: &RpcClient, user: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let blockhash = client.get_latest_blockhash()?;
    let token_mint_address = Pubkey::from_str("12S8ChkSRGMZgtJxLTGcuESE4WE2V2UN5bqwTyo9pNvB")?;
    let reciever = Pubkey::from_str("DrFMYRHBRMcYfcGLzHZ9AggJEqW5SdvQPebJWB9aLfSB")?;

    // let (token_account_addresss, bump) = Pubkey::find_program_address(
    //     &[
    //         &reciever.to_bytes(),
    //         &spl_token::ID.to_bytes(),
    //         &token_mint_address.to_bytes()
    //     ], 
    //     &spl_associated_token_account::ID
    // );

    let instruction = create_associated_token_account(
        &user.pubkey(), 
        &reciever, 
        &token_mint_address, 
        &spl_token::ID
    );

    let tx = Transaction::new_signed_with_payer(
        &[
            instruction
        ], 
        Some(&user.pubkey()), 
        &[user], 
        blockhash
    );

    let signature = client.send_and_confirm_transaction(&tx)?;
    println!("Token account created for: {0}", reciever.to_string());
    println!("Transacton signature: {0}", signature);

    Ok(())
}