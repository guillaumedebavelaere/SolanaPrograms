use anchor_lang::prelude::*;

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
        choices: Vec<Choice>,
        deadline: u64,
    ) -> Result<()> {
        // vérification que choices est un vec de max 5 élements
        require!(
            (choices.len() as u64) <= MAX_CHOICES,
            ProposalError::MaxLengthChoices
        );

        // init attributes from Proposal

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
    choices.iter().map(|choice| 4 + choice.len()).sum::<usize>())
    ]
    pub proposal: Account<'info, Proposal>,
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

pub struct Voter {
    proposal: Pubkey,
    user: Pubkey,
    choice_option: u8,
}

#[error_code]
pub enum ProposalError {
    #[msg("Choices vec is max 5 elements")]
    MaxLengthChoices,
}

// frontend
// Title : What's the best crypto token
// Description ""
// options : "SOL", "BTC", "ETH"
