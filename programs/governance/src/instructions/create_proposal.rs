use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;

use crate::error::GovernanceError;
use crate::events::ProposalCreated;
use crate::state::{Proposal, ProposalInstruction, ProposalStatus, ProposalType, Realm};

pub fn process_create_proposal(
    ctx: Context<CreateProposal>,
    title: String,
    ipfs_hash: [u8; 46],
    proposal_type: ProposalType,
    execution_instruction: Option<Vec<ProposalInstruction>>,
) -> Result<()> {
    // Title must be provided for proposal creation
    require!(!title.is_empty(), GovernanceError::EmptyTitle);

    let realm = &mut ctx.accounts.realm;
    let proposal = &mut ctx.accounts.proposal;
    let user_token_account = &ctx.accounts.user_token_account;

    // Check user has enough tokens to create the proposal
    require!(
        user_token_account.amount >= realm.min_tokens_to_create_proposals,
        GovernanceError::InsufficientTokensToPropose
    );

    let now = Clock::get()?.unix_timestamp;

    proposal.realm = realm.key();
    proposal.proposer = ctx.accounts.proposer.key();
    proposal.title = title;
    proposal.ipfs_hash = ipfs_hash;
    proposal.proposal_type = proposal_type;
    proposal.execution_instruction = execution_instruction.unwrap_or_default();
    proposal.created_at = now;
    proposal.voting_starts_at = now;
    proposal.voting_ends_at = now + realm.voting_duration;
    proposal.execution_at = now + realm.voting_duration + realm.execution_delay;
    proposal.yes_votes = 0;
    proposal.no_votes = 0;
    proposal.abstain_votes = 0;
    proposal.status = ProposalStatus::Voting;
    proposal.bump = ctx.bumps.proposal;

    realm.proposal_count += 1;

    emit!(ProposalCreated {
        proposal: proposal.key(),
        proposer: proposal.proposer,
        title: proposal.title.clone()
    });

    Ok(())
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"realm", realm.authority.as_ref(), realm.name.as_bytes()],
        bump = realm.bump
    )]
    pub realm: Account<'info, Realm>,

    #[account(
        init,
        payer = proposer,
        space = 8 + 5000,
        seeds = [b"proposal", realm.key().as_ref(), proposer.key().as_ref()], // here use created_at or any id, get it from frontend because from this seed propser will be able to create only one propsal
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(
        constraint = user_token_account.owner == proposer.key(),
        constraint = user_token_account.mint == realm.voting_mint
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
