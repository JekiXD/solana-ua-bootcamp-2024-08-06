use std::{default, str::FromStr};

use mpl_token_metadata::{instructions::{CreateV1Builder, CreateV1InstructionArgs}, types::{PrintSupply, TokenStandard}};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};


pub fn run(client: &RpcClient, user: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let blockhash = client.get_latest_blockhash()?;

    let token_mint_address = solana_program_mpl::pubkey::Pubkey::from_str("12S8ChkSRGMZgtJxLTGcuESE4WE2V2UN5bqwTyo9pNvB")?;
    let user_pub = solana_program_mpl::pubkey::Pubkey::from_str(&user.pubkey().to_string())?;

    let (metadata_pda, _metadata_bump) = solana_program_mpl::pubkey::Pubkey::find_program_address(
        &[
            String::from("metadata").as_bytes(),
            &mpl_token_metadata::ID.to_bytes(),
            &token_mint_address.to_bytes()
        ], 
        &mpl_token_metadata::ID
    );

    let instruction = CreateV1Builder::new()
        .metadata(metadata_pda)
        .mint(token_mint_address, false)
        .authority(user_pub)
        .payer(user_pub)
        .update_authority(user_pub, false)
        //metadata
        .name(String::from("JekiV2 Solana Bootcamp"))
        .symbol(String::from("JK-V2-UAB"))
        .uri(String::from("https://arweave.net/1234"))
        .seller_fee_basis_points(0)
        .token_standard(TokenStandard::Fungible)
        .is_mutable(true)
        .decimals(2)
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[
            mpl_intsr_to_sdk_instr(instruction)
        ], 
        Some(&user.pubkey()), 
        &[user], 
        blockhash
    );

    let signature = client.send_and_confirm_transaction(&tx)?;
    println!("Metadata created: {0}", metadata_pda);
    println!("Transacton signature: {0}", signature);

    Ok(())
}

fn mpl_intsr_to_sdk_instr(instr: solana_program_mpl::instruction::Instruction) -> solana_sdk::instruction::Instruction {
    let program_id = solana_sdk::pubkey::Pubkey::from_str(&instr.program_id.to_string()).unwrap();
    let data = instr.data.clone();
    let accounts = instr.accounts.iter().map(|acc| {
        solana_sdk::instruction::AccountMeta {
            pubkey: solana_sdk::pubkey::Pubkey::from_str(&acc.pubkey.to_string()).unwrap(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable
        }
    }).collect();

    solana_sdk::instruction::Instruction::new_with_bytes(program_id, &data, accounts)
}