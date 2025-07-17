use anchor_lang::prelude::*;

use crate::error::GovernanceError;
use crate::events::RealmInitiated;
use crate::state::Realm;

pub fn process_initialize_realm(
    ctx: Context<InitializeRealm>,
    name: String,
    voting_mint: Pubkey,
    min_tokens_to_create_proposal: u64,
    voting_duration: i64,
    execution_delay: i64,
    quorum_threshold: u64,
    approval_threshold: u64,
) -> Result<()> {
    require!(!name.is_empty(), GovernanceError::EmptyName); // Name shouldn't be empty
    require!(voting_duration > 0, GovernanceError::InvalidVotingDuration); // voting duration must be greater than 0
    require!(quorum_threshold <= 10000, GovernanceError::InvalidQuorum);
    require!(
        approval_threshold <= 10000,
        GovernanceError::InvalidApprovalThreshold
    );

    let realm = &mut ctx.accounts.realm;

    realm.authority = ctx.accounts.authority.key();
    realm.name = name;
    realm.voting_mint = voting_mint;
    realm.min_tokens_to_create_proposals = min_tokens_to_create_proposal;
    realm.voting_duration = voting_duration;
    realm.execution_delay = execution_delay;
    realm.quorum_threshold = quorum_threshold;
    realm.approval_threshold = approval_threshold;
    realm.proposal_count = 0;
    realm.bump = ctx.bumps.realm;

    emit!(RealmInitiated {
        realm: realm.key(),
        authority: realm.authority
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeRealm<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 400,
        seeds = [b"realm", authority.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub realm: Account<'info, Realm>,
    pub system_program: Program<'info, System>,
}
