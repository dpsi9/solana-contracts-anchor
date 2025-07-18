use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::{
    error::GovernanceError,
    events::ProposalFinalized,
    state::{Proposal, ProposalStatus, Realm},
};

pub fn process_finalize_proposal(ctx: Context<FinalizeProposal>) -> Result<()> {
    let realm = &mut ctx.accounts.realm;
    let proposal = &mut ctx.accounts.proposal;
    let mint = &mut ctx.accounts.voting_mint;

    let now = Clock::get()?.unix_timestamp;
    // Check if voting has ended or not
    require!(
        now > proposal.voting_ends_at,
        GovernanceError::VotingStillActive
    );

    let total_votes = proposal.yes_votes + proposal.no_votes + proposal.abstain_votes;
    let participation = (total_votes as u128 * 10000) / (mint.supply as u128);
    let approval = if total_votes > 0 {
        (proposal.yes_votes as u128) / (total_votes as u128)
    } else {
        0
    };

    proposal.status = if participation >= realm.quorum_threshold as u128
        && approval >= realm.approval_threshold as u128
    {
        ProposalStatus::Passed
    } else {
        ProposalStatus::Failed
    };

    emit!(ProposalFinalized {
        proposal: proposal.key(),
        status: proposal.status.clone(),
        participation: participation as u64,
        approval: approval as u64
    });

    Ok(())
}

#[derive(Accounts)]
pub struct FinalizeProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"realm", realm.authority.as_ref(), realm.name.as_bytes()],
        bump
    )]
    pub realm: Account<'info, Realm>,
    #[account(
        mut,
        seeds = [b"proposal", realm.key().as_ref(), proposal.proposer.as_ref()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
    pub voting_mint: InterfaceAccount<'info, Mint>,
}
