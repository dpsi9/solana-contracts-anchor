use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;

use crate::{events::DelegationUpdated, state::DelegatedVotes};

pub fn process_delegate_votes(ctx: Context<DelegateVotes>, delegatee: Pubkey) -> Result<()> {
    let delegation = &mut ctx.accounts.delegated_votes;
    delegation.voter = ctx.accounts.voter.key();
    delegation.delegatee = delegatee;
    delegation.amount = ctx.accounts.user_token_account.amount;
    delegation.bump = ctx.bumps.delegated_votes;

    emit!(DelegationUpdated {
        voter: delegation.voter,
        delegatee
    });
    Ok(())
}

#[derive(Accounts)]
pub struct DelegateVotes<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(
        init,
        payer = voter,
        space = 8 + 100,
        seeds = [b"delegation", voter.key().as_ref()],
        bump
    )]
    pub delegated_votes: Account<'info, DelegatedVotes>,
    #[account(
        constraint = user_token_account.owner == voter.key()
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}
