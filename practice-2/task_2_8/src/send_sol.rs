use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction, transaction::Transaction};


pub fn run(client: &RpcClient, user: &Keypair) -> Result<(), Box<dyn std::error::Error>> {

    let reciever = Pubkey::from_str("DrFMYRHBRMcYfcGLzHZ9AggJEqW5SdvQPebJWB9aLfSB")?;
    println!("Sending 100 sol!");
    println!("From: {0}", user.pubkey().to_string());
    println!("To: {0}", reciever.to_string());

    let blockhash = client.get_latest_blockhash()?;
    let instruction = system_instruction::transfer(&user.pubkey(), &reciever, 100);

    let memo_message = "Some memo message";
    let memo_program = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")?;

    let memo_nstruction = Instruction::new_with_bytes(
        memo_program, 
        memo_message.as_bytes(), 
        vec![
            AccountMeta::new(user.pubkey(), true)
        ]
    );

    let tx = Transaction::new_signed_with_payer(
        &[
            instruction,
            memo_nstruction
        ], 
        Some(&user.pubkey()), 
        &[user], 
        blockhash
    );


    let signature = client.send_and_confirm_transaction(&tx)?;
    println!("Transaction confirmed!");
    println!("Signature: {0}", signature);
    println!("Sent memo: {0}", memo_message);

    Ok(())
}