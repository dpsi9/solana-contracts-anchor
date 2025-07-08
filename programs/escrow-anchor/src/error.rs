use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The token account does not match the required mint")]
    InvalidTokenMint,
    #[msg("The token account is not owned by the user")]
    InvalidTokenOwner,
    #[msg("Invalid amount")]
    InvalidAmount,
}
