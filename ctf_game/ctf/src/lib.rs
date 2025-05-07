//! # Game Leveling Program
//!
//! This Solana program implements a simple game-leveling system using two custom account types:
//! `GameConfig` and `User`. The program allows an admin to configure how many credits are needed
//! per level and to mint credits to users. Users can then spend these credits to level up.
//!
//! ## Instruction Overview:
//!
//! - `CreateGameConfig { credits_per_level }`  
//!   Creates a new `GameConfig` account that defines how many credits are required per level.  
//!   The account is a PDA derived from the admin and the `GAME_CONFIG_SEED`.
//!
//! - `CreateUser {}`  
//!   Creates a `User` account associated with a specific game config and authority.  
//!   The account is a PDA derived from the game config, authority, and `USER_SEED`.
//!
//! - `MintCreditsToUser { credits }`  
//!   Allows the admin (associated with the game config) to mint a given number of credits to a user.
//!
//! - `UserLevelUp { credits_to_burn }`  
//!   Allows a user to burn credits to level up. The required amount of credits increases linearly
//!   with each level based on the `credits_per_level` defined in the game config.  
//!   The user can level up multiple times in a single transaction as long as they have enough credits.
//!
//! ## Accounts
//! - `GameConfig`: Stores game parameters like `credits_per_level`.
//! - `User`: Stores user state including authority, selected game config, current credits, and level.
//!
//! ## Address Derivation
//! - `GameConfig`: `PDA(admin, GAME_CONFIG_SEED)`
//! - `User`: `PDA(game_config, user_authority, USER_SEED)`
//!
//! ## Design Notes:
//! - PDAs ensure deterministic account addresses for game configs and users.
//! - Borsh is used for instruction serialization and account data handling.
//! - Custom trait `AccountData` enables type-safe deserialization of account types.


use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    declare_id,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

pub mod state;
pub mod instructions;
pub mod constants;
mod processor;

use instructions::ProgramInstruction;
use processor::*;

declare_id!("GAME8ZGUzNChyRXHMxR4fVTvhpNDa6dJyK8oVmydp4RZ");

entrypoint!(process_instruction);

/// Top level instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match ProgramInstruction::try_from_slice(instruction_data)? {
        ProgramInstruction::CreateGameConfig { credits_per_level } => create_game_config(credits_per_level, accounts),
        ProgramInstruction::CreateUser { } => create_user(accounts),
        ProgramInstruction::MintCreditsToUser { credits } => mint_credits_to_user(credits, accounts),
        ProgramInstruction::UserLevelUp { credits_to_burn } => user_level_up(credits_to_burn, accounts)
    }
}