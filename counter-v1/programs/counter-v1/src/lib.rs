// Imports
use anchor_lang::prelude::*;

// Identifiant du program qui est unique
declare_id!("DAk69KNVA23rSeZHfMLULc3qybvgGGawkxmUnsqKNeUc");

// Instructions
#[program]
pub mod counter {
    use super::*;

    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.authority = ctx.accounts.signer.key();

        Ok(())
    }
}

const ANCHOR_DISCRIMINATOR: usize = 8;

// Context
#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    // counter à créer
    #[account(init, payer = signer, space = ANCHOR_DISCRIMINATOR + Counter::INIT_SPACE, seeds = [b"counter", signer.key().as_ref()], bump)]
    // 8 pour anchor discrimanteur et 8 pour le u64
    pub counter: Account<'info, Counter>,
    // signer
    #[account(mut)]
    pub signer: Signer<'info>,
    // system program
    pub system_program: Program<'info, System>,
}

// Accounts
#[account]
#[derive(InitSpace)]
pub struct Counter {
    count: u64,
    authority: Pubkey,
}
