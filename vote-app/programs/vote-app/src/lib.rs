use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("635yU9yp7a6hf1s4vhTGrpRp2qrP1xXr9F3iDutshS7V");

#[program]
mod hello_anchor {
    use super::*;
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        choices: Vec<String>,
        deadline: u64,
    ) -> Result<()> {
        // vérification que choices est un vec de max 5 élements
        require!(
            (choices.len() as u64) <= MAX_CHOICES,
            ProposalError::MaxLengthChoices
        );

        // init attributes from Proposal
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

    pub fn cast_vote(ctx: Context<CastVote>, user_choice: u8) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;

        // verification au niveau de la deadline
        require!(
            Clock::get()?.unix_timestamp < proposal.deadline as i64,
            ProposalError::DeadlinePassed
        );

        // init voter
        let voter = &mut ctx.accounts.voter;
        voter.proposal = proposal.key();
        voter.user = ctx.accounts.signer.key();
        voter.choice_option = user_choice;

        // verification choices length
        /*if user_choice < 0 {
            return Err("user choice index negative");
        }
        if (proposal.choices.len() as u8) < user_choice {
            return Err("user choice index too big");
        }*/
        require!(
            (proposal.choices.len() as u8) < user_choice,
            ProposalError::InvalidOption
        );

        // +1
        proposal.choices[user_choice as usize].count += 1;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String, description: String, choices: Vec<String>)]
pub struct CreateProposal<'info> {
    #[account(init, payer = signer, space = 
    8 + // anchor discrimanator
    (4 + title.len()) + // title lenght
    (4 + description.len()) + // description length
    8 + // deadline u64
    4 + // vec length
    choices.iter().map(|choice| 4 + choice.len() + 8).sum::<usize>()) // choice label size + count u64
    ]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(init, 
    payer = signer, 
    space = 8 + Voter::INIT_SPACE, 
    seeds = [proposal.key().as_ref(), signer.key().as_ref()], 
    bump)]
    pub voter: Account<'info, Voter>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

const MAX_CHOICES: u64 = 5;

#[account]
pub struct Proposal {
    title: String,
    description: String,
    choices: Vec<Choice>,
    deadline: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Choice {
    label: String,
    count: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Voter {
    proposal: Pubkey,
    user: Pubkey,
    choice_option: u8,
}

#[error_code]
pub enum ProposalError {
    #[msg("Choices vec is max 5 elements")]
    MaxLengthChoices,
    #[msg("You can't vote anymore, deadline passed")]
    DeadlinePassed,
    #[msg("option index negative or too big")]
    InvalidOption,
}

// frontend
// Title : What's the best crypto token
// Description ""
// options : "SOL", "BTC", "ETH"
