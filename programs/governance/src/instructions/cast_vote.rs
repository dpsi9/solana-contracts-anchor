use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;

use crate::error::GovernanceError;
use crate::events::VoteCast;
use crate::state::{DelegatedVotes, Proposal, ProposalStatus, Realm, VoteRecord, VoteType};

pub fn process_cast_vote(ctx: Context<CastVote>, vote_type: VoteType) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let vote_record = &mut ctx.accounts.vote_record;
    let user_token_account = &ctx.accounts.user_token_account;
    let delegated_votes = ctx.accounts.delegated_votes.as_ref(); // as ref let's you directly match on delegated votes

    let now = Clock::get()?.unix_timestamp;
    // Check if voting period is active or not
    require!(
        now >= proposal.voting_starts_at && now <= proposal.voting_ends_at,
        GovernanceError::VotingNotActive
    );
    // Make sure the proposal is in voting state
    require!(
        proposal.status == ProposalStatus::Voting,
        GovernanceError::ProposalNotInVotingState
    );

    let direct_weight = user_token_account.amount;
    let delegated_weight = delegated_votes.map(|dv| dv.amount).unwrap_or(0);
    let total_weight = direct_weight + delegated_weight;

    require!(total_weight > 0, GovernanceError::NoVotingPower);

    // Initialize the vote record
    vote_record.voter = ctx.accounts.voter.key();
    vote_record.proposal = proposal.key();
    vote_record.vote_type = vote_type.clone();
    vote_record.weight = total_weight;
    vote_record.timestamp = now;
    vote_record.bump = ctx.bumps.vote_record;

    // Perform voting
    match vote_type {
        VoteType::Yes => proposal.yes_votes += total_weight,
        VoteType::No => proposal.no_votes += total_weight,
        VoteType::Abstain => proposal.abstain_votes += total_weight,
    }

    emit!(VoteCast {
        proposal: proposal.key(),
        voter: vote_record.voter,
        vote_type,
        weight: total_weight
    });
    Ok(())
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(mut)]
    pub realm: Account<'info, Realm>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    #[account(
        init,
        payer = voter,
        space = 8 + 100,
        seeds = [b"vote", proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>,
    #[account(
        constraint = user_token_account.owner == voter.key(),
        constraint = user_token_account.mint == realm.voting_mint
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"delegation", voter.key().as_ref()],
        bump
    )]
    pub delegated_votes: Option<Account<'info, DelegatedVotes>>,

    pub system_program: Program<'info, System>,
}
