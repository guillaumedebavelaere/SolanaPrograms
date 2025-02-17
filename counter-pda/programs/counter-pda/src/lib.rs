use anchor_lang::prelude::*;

declare_id!("HFDmW1soYc5Z644hq713rt2Xyff8tnPwcXoxdCT979du");

#[program]
pub mod counter_pda {
    use super::*;

    pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
        msg!("Creating a Counter!!");

        let counter = &mut ctx.accounts.counter;
        counter.authority = ctx.accounts.authority.key();
        counter.count = 0;

        msg!("Current count is {}", counter.count);
        msg!("The Admin PubKey is: {} ", counter.authority);

        Ok(())
    }

    pub fn update_counter(ctx: Context<UpdateCounter>) -> Result<()> {
        msg!("Adding 1 to the counter!!");
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("Current count is {}", counter.count);
        msg!("{} remaining to reach 1000 ", 1000 - counter.count);

        Ok(())
    }

    pub fn close_counter(_ctx: Context<CloseCounter>) -> Result<()> {
        msg!("Close the counter account");
        Ok(())
    }

}

#[derive(Accounts)]
pub struct CreateCounter<'info> {
    #[account(mut)]
    authority: Signer<'info>,
    #[account(
        init,
        seeds = [b"counter", authority.key().as_ref()],
        bump,
        payer = authority,
        space = 100
    )]
    counter: Account<'info, Counter>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCounter<'info> {
    authority: Signer<'info>,
    #[account(mut, has_one = authority)]
    counter: Account<'info, Counter>,
}

#[derive(Accounts)]
pub struct CloseCounter<'info> {
    authority: Signer<'info>,
    #[account(
        mut, 
        close = authority,
        has_one = authority
    )]
    counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    authority: Pubkey,
    count: u64,
}