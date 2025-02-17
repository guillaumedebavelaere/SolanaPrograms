use anchor_lang::prelude::*;

declare_id!("HWog1qHWAJd8GjMfuCWv8JhWAeaDT1oaQxh6LiEhjm7y");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
