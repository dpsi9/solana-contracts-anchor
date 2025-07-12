use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("The provided amount is invalid")]
    InvalidAmount,
    #[msg("Unauthorized to perform this action")]
    Unauthorized,
    #[msg("Insufficient stake balance")]
    InsufficientStakeBalance,
    #[msg("Minimum stake duration not met")]
    MinimumStakeDurationNotMet,
    #[msg("No rewards to claim")]
    NoRewardsToClaim,
}
