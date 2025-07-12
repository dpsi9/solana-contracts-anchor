use crate::error::StakingError;
use crate::state::{StakePool, UserStakeAccount};
use crate::utility::{calculate_pending_rewards, update_pool_rewards};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

pub fn process_claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let user_stake = &mut ctx.accounts.user_stake_account;
    // Validate user owns this stake account
    require!(
        user_stake.user == ctx.accounts.user.key(),
        StakingError::Unauthorized
    );

    //update pool rewards
    {
        let pool = &mut ctx.accounts.stake_pool;
        update_pool_rewards(pool, current_time)?;
    }

    // calculate total rewards to claim
    let total_rewards = calculate_pending_rewards(user_stake, &ctx.accounts.stake_pool)?
        .checked_add(user_stake.pending_rewards)
        .unwrap();

    require!(total_rewards > 0, StakingError::NoRewardsToClaim);

    let stake_mint_key = ctx.accounts.stake_pool.stake_mint.key();
    let reward_mint_key = ctx.accounts.stake_pool.reward_mint.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"stake_pool",
        stake_mint_key.as_ref(),
        reward_mint_key.as_ref(),
        &[ctx.accounts.stake_pool.bump],
    ]];
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.pool_reward_vault.to_account_info(),
            mint: ctx.accounts.reward_mint.to_account_info(),
            to: ctx.accounts.user_reward_account.to_account_info(),
            authority: ctx.accounts.stake_pool.to_account_info(),
        },
        signer_seeds,
    );
    token_interface::transfer_checked(
        cpi_context,
        total_rewards,
        ctx.accounts.reward_mint.decimals,
    )?;
    user_stake.pending_rewards = 0;
    user_stake.reward_per_token_paid = ctx.accounts.stake_pool.reward_per_token_stored;

    msg!("Claimed {} reward tokens", total_rewards);

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"stake_pool", stake_pool.stake_mint.as_ref(), stake_pool.reward_mint.as_ref()],
        bump = stake_pool.bump
    )]
    pub stake_pool: Account<'info, StakePool>,
    #[account(
        mut,
        seeds = [b"user_stake", stake_pool.key().as_ref(), user.key().as_ref()],
        bump = user_stake_account.bump
    )]
    pub user_stake_account: Account<'info, UserStakeAccount>,
    #[account(
        mut,
        constraint = user_reward_account.owner == user.key(),
        constraint = user_reward_account.mint == stake_pool.reward_mint,
    )]
    pub user_reward_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"reward_vault", stake_pool.key().as_ref(), stake_pool.reward_mint.as_ref()],
        bump
    )]
    pub pool_reward_vault: InterfaceAccount<'info, TokenAccount>,
    pub reward_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}
