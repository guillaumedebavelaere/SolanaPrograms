use anchor_lang::prelude::*;

#[error_code]
pub enum ProposalError {
    #[msg("Choices vec is max 5 elements")]
    MaxLengthChoices,
    #[msg("You can't vote anymore, deadline passed")]
    DeadlinePassed,
    #[msg("option index negative or too big")]
    InvalidOption,
}