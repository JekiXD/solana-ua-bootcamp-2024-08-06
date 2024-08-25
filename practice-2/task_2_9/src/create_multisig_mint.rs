use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::{system_instruction, system_program};

const MINT_SIZE: u64 = 82;

pub fn run(client: &RpcClient, user1: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let rent = client.get_minimum_balance_for_rent_exemption(MINT_SIZE as usize)?;
    let multisig = Pubkey::from_str("Goug4c34QRwQxzJkeubsZbTwoAjhGrtKbRNk7ovfdvD9")?;
    let blockhash = client.get_latest_blockhash()?;
    let mint_account = Keypair::new();

    let create_mint_instruction = system_instruction::create_account(
        &user1.pubkey(), 
        &mint_account.pubkey(), 
        rent, 
        MINT_SIZE, 
        &spl_token::id()
    );

    let init_mint_instruction = spl_token::instruction::initialize_mint2(
        &spl_token::id(), 
        &mint_account.pubkey(), 
        &multisig, 
        None, 10
    )?;

    let tx = Transaction::new_signed_with_payer(
        &[
            create_mint_instruction,
            init_mint_instruction
        ], 
        Some(&user1.pubkey()), 
        &[user1, &mint_account], 
        blockhash
    );

    let signature = client.send_and_confirm_transaction(&tx)?;

    println!("Created and initialized mint account: {0}", mint_account.pubkey());
    println!("Transaction signature: {0}", signature);

    Ok(())
}