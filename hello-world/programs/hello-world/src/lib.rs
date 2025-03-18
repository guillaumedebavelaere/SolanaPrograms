// Imports
use anchor_lang::prelude::*;

// Identifiant du program qui est unique
declare_id!("1J3GS4PhUsLkH2o6XqMYp3fRRo44iG3cFNLMrGZGS8w");

// Instructions
#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, _age: u64) -> Result<()> {
        msg!("Hello : {}", name); // affichage message dans les logs explorer
        Ok(())
    }
}

// Context
#[derive(Accounts)]
pub struct Initialize {}

// Accounts
