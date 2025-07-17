use anchor_lang::prelude::*;

#[error_code]
pub enum GovernanceError {
    #[msg("Realm name cannot be empty")]
    EmptyName,
    #[msg("Title cannot be empty")]
    EmptyTitle,
    #[msg("Invalid voting duration")]
    InvalidVotingDuration,
    #[msg("Invalid quorum threshold")]
    InvalidQuorum,
    #[msg("Invalid approval threshold")]
    InvalidApprovalThreshold,
    #[msg("Insufficient tokens to create proposals")]
    InsufficientTokensToPropose,
    #[msg("Voting not active")]
    VotingNotActive,
    #[msg("Proposal not in voting state")]
    ProposalNotInVotingState,
    #[msg("Already voted")]
    AlreadyVoted,
    #[msg("No voting power")]
    NoVotingPower,
    #[msg("Voting not ended yet")]
    VotingStillActive,
    #[msg("Proposal has not passed")]
    ProposalNotPassed,
    #[msg("Execution delay not met")]
    ExecutionDelayNotMet,
}
