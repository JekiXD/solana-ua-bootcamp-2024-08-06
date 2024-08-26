use solana_client::rpc_client::RpcClient;
use solana_sdk::{nonce, pubkey::Pubkey, signature::{self, Keypair}, signer::Signer, system_instruction, transaction::Transaction};

const NONCE_ACCOUNT_LENGTH: u64 = 80;

pub fn run(client: &RpcClient, user: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let blockhash = client.get_latest_blockhash()?;
    let rent = client.get_minimum_balance_for_rent_exemption(nonce::State::size())?;
    let nonce_acc = Keypair::new();

    let create_nonce_instruction = system_instruction::create_nonce_account(
        &user.pubkey(), 
        &nonce_acc.pubkey(), 
        &user.pubkey(), 
        rent
    );

    let tx = Transaction::new_signed_with_payer(
        &create_nonce_instruction, 
        Some(&user.pubkey()), 
        &[user, &nonce_acc], 
        blockhash
    );

    let signature = client.send_and_confirm_transaction(&tx)?;
    println!("Nonce created: {0}", nonce_acc.pubkey().to_string());
    println!("Transacton signature: {0}", signature);

    Ok(())
}