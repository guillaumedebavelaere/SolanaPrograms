use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("7UcDy4kQetR3MVK5utV9kfW85WMvafxrPgYkQ469hQA1");

#[program]
pub mod vault {
    use anchor_lang::system_program;

    use super::*;

    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.amount = 0;

        msg!("Vault created");
        Ok(())
    }

    pub fn deposit(ctx: Context<DepositVault>, amount: u64) -> Result<()> {

        // transfert par CPI
        // création du context 
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.vault.to_account_info()
            }
        );

        // puis appel du programme SystemProgram transfert
        system_program::transfer(cpi_context, amount)?;


        // MAJ de l'attribut amount
        ctx.accounts.vault.amount += amount;
        msg!("Deposit on vault");

        Ok(())
    }

    pub fn withdraw(ctx: Context<WithdrawVault>, amount: u64) -> Result<()> {
        let fee: u64 = 10000;
        ctx.accounts.vault.amount -= amount;
        
        // Equilibre entre le + et -

        **ctx.accounts
            .vault
            .to_account_info()
            .try_borrow_mut_lamports()? -= amount;

        **ctx
            .accounts
            .signer
            .to_account_info()
            .try_borrow_mut_lamports()? += amount - fee;

        **ctx
            .accounts
            .admin
            .to_account_info()
            .try_borrow_mut_lamports()? += fee;
        
        msg!("Withdraw on vault");
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

#[derive(Accounts)]
pub struct WithdrawVault<'info> {
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
    #[account(mut)]
    pub admin: UncheckedAccount<'info>,
    // system_program => transfert
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    amount: u64,
}
