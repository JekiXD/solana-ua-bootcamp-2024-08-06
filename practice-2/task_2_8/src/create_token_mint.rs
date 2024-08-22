use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::{self, Keypair}, signer::Signer, system_instruction, transaction::Transaction};
use spl_token::instruction::initialize_mint2;

use crate::{generate_keypair, MINT_SIZE};

pub fn run(client: &RpcClient, user: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let blockhash = client.get_latest_blockhash()?;
    let rent = client.get_minimum_balance_for_rent_exemption(MINT_SIZE as usize)?;
    let mint_key = generate_keypair();

    let create_mint_instruction = system_instruction::create_account(
        &user.pubkey(), 
        &mint_key.pubkey(), 
        rent, 
        MINT_SIZE, 
        &spl_token::ID
    );

    let init_mint_instruction = initialize_mint2(
        &spl_token::ID, 
        &mint_key.pubkey(), 
        &user.pubkey(), 
        None, 
        2
    )?;

    let tx = Transaction::new_signed_with_payer(
        &[
            create_mint_instruction,
            init_mint_instruction
        ], 
        Some(&user.pubkey()), 
        &[user, &mint_key], 
        blockhash
    );

    let signature = client.send_and_confirm_transaction(&tx)?;
    println!("Transacton signature: {0}", signature);

    Ok(())
}