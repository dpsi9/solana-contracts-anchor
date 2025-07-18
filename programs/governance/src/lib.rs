use anchor_lang::prelude::*;

pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

use crate::state::{ProposalInstruction, ProposalType, VoteType};
pub use instructions::*;

declare_id!("Gover11111111111111111111111111111111111111");

#[program]
pub mod governance {
    use super::*;

    pub fn initialize_realm(
        ctx: Context<InitializeRealm>,
        name: String,
        voting_mint: Pubkey,
        min_tokens_to_create_proposal: u64,
        voting_duration: i64,
        execution_delay: i64,
        quorum_threshold: u64,
        approval_threshold: u64,
    ) -> Result<()> {
        process_initialize_realm(
            ctx,
            name,
            voting_mint,
            min_tokens_to_create_proposal,
            voting_duration,
            execution_delay,
            quorum_threshold,
            approval_threshold,
        )
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        ipfs_hash: [u8; 46],
        proposal_type: ProposalType,
        execution_instruction: Option<Vec<ProposalInstruction>>,
    ) -> Result<()> {
        process_create_proposal(ctx, title, ipfs_hash, proposal_type, execution_instruction)
    }

    pub fn cast_vote(ctx: Context<CastVote>, vote_type: VoteType) -> Result<()> {
        process_cast_vote(ctx, vote_type)
    }
}
