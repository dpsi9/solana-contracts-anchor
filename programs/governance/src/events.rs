use anchor_lang::prelude::*;

use crate::state::{ProposalStatus, VoteType};

#[event]
pub struct RealmInitiated {
    pub realm: Pubkey,
    pub authority: Pubkey,
}

#[event]
pub struct ProposalCreated {
    pub proposal: Pubkey,
    pub proposer: Pubkey,
    pub title: String,
}

#[event]
pub struct VoteCast {
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub vote_type: VoteType,
    pub weight: u64,
}

#[event]
pub struct ProposalFinalized {
    pub proposal: Pubkey,
    pub status: ProposalStatus,
    pub participation: u64,
    pub approval: u64,
}

#[event]
pub struct ProposalExecuted {
    pub proposal: Pubkey,
}
