use anchor_lang::prelude::*;

declare_id!("6dHVUfZ2ppTbxsRsmoAwyD1wCz8tp9uKH9GCyjrFuSvU");

#[program]
pub mod todolist {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        // Initialiser les attributs de l'utilisateur

        // user_pubkey la public key du signer
        // todocount = 0
        // nickname = paramètre passé

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    // signer => utilisateur qui signe la transaction
    #[account(mut)]
    pub signer: Signer<'info>,
    // user => utilisateur à créé
    #[account(
        init,
        payer = signer,
        space = 8 + User::INIT_SPACE,
        seeds = [b"user", signer.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,
    // system_program => alloue l'espace pour l'account à créer
    pub system_program: Program<'info, System>,
}


#[account]
#[derive(InitSpace)]
pub struct User {
    user_pubkey: Pubkey,
    #[max_len(30)]
    nickname: String,
    todo_count: u32,
}

#[account]
#[derive(InitSpace)]
pub struct Todo {
    todo_id: u32,
    status: TodoStatus,
    #[max_len(50)]
    description: String,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TodoStatus {
    Todo,
    Done
    // In Progress
}
