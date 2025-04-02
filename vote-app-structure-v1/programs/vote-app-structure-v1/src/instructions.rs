use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;
use crate::{
    errors::ProposalError,
    constants::*,
    state::*,
};

pub fn create_proposal(
    ctx: Context<CreateProposal>,
    title: String,
    description: String,
    choices: Vec<String>,
    deadline: u64,
) -> Result<()> {
    require!(
        (choices.len() as u64) <= MAX_CHOICES,
        ProposalError::MaxLengthChoices
    );
    let proposal = &mut ctx.accounts.proposal;
    proposal.title = title;
    proposal.description = description;
    proposal.deadline = deadline;
    let mut choices_vec = Vec::new();
    for choice in choices {
        let option = Choice {
            label: choice,
            count: 0,
        };
        choices_vec.push(option);
    }
    proposal.choices = choices_vec;
    Ok(())
}

#[derive(Accounts)]
#[instruction(title: String, description: String, choices: Vec<String>)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        payer = signer,
        space = 8
            + (4 + title.len())
            + (4 + description.len())
            + 8
            + 4
            + choices.iter().map(|choice| 4 + choice.len() + 8).sum::<usize>()
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn cast_vote(ctx: Context<CastVote>, user_choice: u8) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    require!(
        Clock::get()?.unix_timestamp < proposal.deadline as i64,
        ProposalError::DeadlinePassed
    );
    let voter = &mut ctx.accounts.voter;
    voter.proposal = proposal.key();
    voter.user = ctx.accounts.signer.key();
    voter.choice_option = user_choice;
    require!(
        (proposal.choices.len() as u8) > user_choice,
        ProposalError::InvalidOption
    );
    proposal.choices[user_choice as usize].count += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(
        init,
        payer = signer,
        space = 8 + Voter::INIT_SPACE,
        seeds = [proposal.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub voter: Account<'info, Voter>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}