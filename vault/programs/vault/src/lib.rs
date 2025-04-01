use anchor_lang::prelude::*;

declare_id!("7UcDy4kQetR3MVK5utV9kfW85WMvafxrPgYkQ469hQA1");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
