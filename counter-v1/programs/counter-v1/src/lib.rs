// Imports
use anchor_lang::prelude::*;

// Identifiant du program qui est unique
declare_id!("7gW3FxyrXEMwxh9Pm8fPLHNXfrLxvbzin5gLRMXQEh9k");

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

    // increment counter + 1
    pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }

    // set counter value from paramater
    pub fn set_counter(ctx: Context<SetCounter>, count: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = count;
        Ok(())
    }

    // close account counter (delete)
    pub fn close_counter(ctx: Context<CloseCounter>) -> Result<()> {
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

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    //#[account(mut, seeds = [b"counter", signer.key().as_ref()], bump)]
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    // signer
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetCounter<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseCounter<'info> {
    #[account(mut, has_one = authority, close = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

// Accounts
#[account]
#[derive(InitSpace)]
pub struct Counter {
    count: u64,
    authority: Pubkey,
}
