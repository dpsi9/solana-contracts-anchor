use anchor_lang::prelude::*;

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
