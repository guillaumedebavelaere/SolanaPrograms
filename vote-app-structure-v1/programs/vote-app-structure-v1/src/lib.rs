use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("3ABTQrNqyZyXdkmtxHfHhKyrzQoRaDFvL5dWN1XPzRrG");

#[program]
pub mod vote_app_structure_v1 {
    use super::*;

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        choices: Vec<String>,
        deadline: u64,
    ) -> Result<()> {
        instructions::create_proposal(ctx, title, description, choices, deadline)
    }

    pub fn cast_vote(ctx: Context<CastVote>, user_choice: u8) -> Result<()> {
        instructions::cast_vote(ctx, user_choice)
    }   
}
