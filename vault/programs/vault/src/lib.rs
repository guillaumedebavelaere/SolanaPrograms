use anchor_lang::prelude::*;

declare_id!("7UcDy4kQetR3MVK5utV9kfW85WMvafxrPgYkQ469hQA1");

#[program]
pub mod vault {
    use super::*;

    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.amount = 0;

        msg!("Vault created");
        Ok(())
    }

    pub fn deposit(ctx: Context<DepositVault>, amount: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateVault<'info> {
    // signer => utilisateur qui signe la transaction
    #[account(mut)]
    pub signer: Signer<'info>,
    // vault => vault à créé
    #[account(
        init,
        payer = signer,
        space = 8 + 8,
        seeds = [b"vault"],
        bump
    )]
    pub vault: Account<'info, Vault>,
    // system_program => alloue l'espace pour l'account à créer
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositVault<'info> {
    // signer => utilisateur qui signe la transaction
    #[account(mut)]
    pub signer: Signer<'info>,
    // vault => vault à créé
    #[account(
        mut,
        seeds = [b"vault"],
        bump
    )]
    pub vault: Account<'info, Vault>,
    // system_program => transfert
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    amount: u64,
}
