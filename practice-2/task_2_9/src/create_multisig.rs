use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::{system_instruction, system_program};

const MULTISIG_SIZE: u64 = 355;

pub fn run(client: &RpcClient, user1: &Keypair, user2: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let rent = client.get_minimum_balance_for_rent_exemption(MULTISIG_SIZE as usize)?;
    let multisig = Keypair::new();
    let blockhash = client.get_latest_blockhash()?;

    let create_multisig_instruction = system_instruction::create_account(
        &user1.pubkey(), 
        &multisig.pubkey(), 
        rent, 
        MULTISIG_SIZE, 
        &spl_token::id()
    );

    let init_multisig_instruction = spl_token::instruction::initialize_multisig2(
        &spl_token::id(), 
        &multisig.pubkey(), 
        &[
            &user1.pubkey(),
            &user2.pubkey()
        ], 
        2
    )?;

    let tx = Transaction::new_signed_with_payer(
        &[
            create_multisig_instruction,
            init_multisig_instruction
        ], 
        Some(&user1.pubkey()), 
        &[user1, &multisig], 
        blockhash
    );

    let signature = client.send_and_confirm_transaction(&tx)?;

    println!("Created and initialized multisig account");
    println!("Multisig pubkey: {0}", multisig.pubkey());
    println!("Multisig secretkey: {0:?}", multisig.secret().as_bytes());
    println!("Transaction signature: {0}", signature);

    Ok(())
}