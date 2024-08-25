use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};


pub fn run(client: &RpcClient, user1: &Keypair, user2: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let blockhash = client.get_latest_blockhash()?;
    let token_mint_address = Pubkey::from_str("EmPSMn1jqA9JEWPhji6TJd23t7qUCkkNAZY5URvknjXY")?;
    let multisig = Pubkey::from_str("Goug4c34QRwQxzJkeubsZbTwoAjhGrtKbRNk7ovfdvD9")?;

    let (token_account_addresss, _bump) = Pubkey::find_program_address(
        &[
            &user2.pubkey().to_bytes(),
            &spl_token::ID.to_bytes(),
            &token_mint_address.to_bytes()
        ], 
        &spl_associated_token_account::ID
    );

    let amount = 100000000;
    let mint_to_instruction = spl_token::instruction::mint_to(
        &spl_token::ID, 
        &token_mint_address, 
        &token_account_addresss, 
        &multisig, 
        &[
            &user1.pubkey(),
            &user2.pubkey()
        ], 
        amount
    )?;

    let tx = Transaction::new_signed_with_payer(
        &[
            mint_to_instruction
        ], 
        Some(&user1.pubkey()), 
        &[user1, &user2], 
        blockhash
    );

    let signature = client.send_and_confirm_transaction(&tx)?;
    println!("Minted {0} tokens to: {1}", amount, user2.pubkey().to_string());
    println!("Transacton signature: {0}", signature);

    Ok(())
}