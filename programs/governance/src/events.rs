use anchor_lang::prelude::*;

use crate::state::VoteType;

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
