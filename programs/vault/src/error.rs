use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Invalid Threshold")]
    InvalidThreshold,
    #[msg("No owners provided")]
    NoOwners,
    #[msg("Threshold too high")]
    ThresholdTooHigh,
    #[msg("Too many owners")]
    TooManyOwners,
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Description too long")]
    DescriptionTooLong,
    #[msg("Already approved")]
    AlreadyApproved,
    #[msg("Transaction already executed")]
    TransactionExecuted,
    #[msg("Insufficient approvals")]
    InsufficientApprovals,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Owner already exists")]
    OwnerAlreadyExists,
    #[msg("Owner not found")]
    OwnerNotFound,
    #[msg("Cannot remove owner")]
    CannotRemoveOwner,
}
