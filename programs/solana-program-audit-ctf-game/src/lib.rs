use anchor_lang::prelude::*;

declare_id!("J4xGwfqaVwBFUvetT4zybTNLqNyYqzC2eNFwVtaphzjV");

#[program]
pub mod solana_program_audit_ctf_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
