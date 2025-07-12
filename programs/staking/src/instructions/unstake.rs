use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};
use crate::state::{StakePool, UserStakeAccount};
use crate::error::StakingError;
use crate::utility::{calculate_pending_rewards, update_pool_rewards};

pub fn process_unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    require!(amount > 0, StakingError::InvalidAmount);

    let current_time = Clock::get()?.unix_timestamp;
    let user_stake = &mut ctx.accounts.user_stake_account;

    require!(user_stake.user == ctx.accounts.user.key(), StakingError::Unauthorized);

    let stake_duration = current_time - user_stake.stake_start_time;

    require!(stake_duration >= ctx.accounts.stake_pool.min_stake_duration, StakingError::MinimumStakeDurationNotMet);

    require!(user_stake.amount_staked >= amount, StakingError::InsufficientStakeBalance);

    {
        let pool = &mut ctx.accounts.stake_pool;
        update_pool_rewards(pool, current_time)?;
    }

    // update user's pending rewards
    user_stake.pending_rewards = calculate_pending_rewards(user_stake, &ctx.accounts.stake_pool)?;
    user_stake.reward_per_token_paid = ctx.accounts.stake_pool.reward_per_token_stored;

    let stake_mint_key = ctx.accounts.stake_pool.stake_mint.key();
    let reward_mint_key = ctx.accounts.stake_pool.reward_mint.key();
    let signer_seeds: &[&[&[u8]]] = &[
        &[
            b"stake_pool",
            stake_mint_key.as_ref(),
            reward_mint_key.as_ref(),
            &[ctx.accounts.stake_pool.bump]
        ]
    ];

    // transfer tokens from pool to user
    token_interface::transfer_checked(
        CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), 
        TransferChecked { 
            from: ctx.accounts.pool_stake_vault.to_account_info(),
            mint: ctx.accounts.stake_mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.stake_pool.to_account_info() 
        }, signer_seeds),
        amount, ctx.accounts.stake_mint.decimals)?;

    //Update state
    user_stake.amount_staked = user_stake.amount_staked.checked_sub(amount).unwrap();
    
    let pool = &mut ctx.accounts.stake_pool;
    pool.total_staked = pool.total_staked.checked_sub(amount).unwrap();

    msg!("Unstaked tokens {}. Remaining staked {}", amount, user_stake.amount_staked);
    Ok(())
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user_stake", stake_pool.key().as_ref(), user.key().as_ref()],
        bump = user_stake_account.bump
    )]
    pub user_stake_account: Account<'info, UserStakeAccount>,
    #[account(
        mut, 
        constraint = user_token_account.owner == user.key(),
        constraint = user_token_account.mint == stake_pool.stake_mint,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"stake_pool", stake_pool.stake_mint.as_ref(), stake_pool.reward_mint.as_ref()],
        bump = stake_pool.bump
    )]
    pub stake_pool: Account<'info, StakePool>,
    #[account(
        mut,
        seeds = [b"stake_vault", stake_pool.key().as_ref(),stake_pool.stake_mint.as_ref()],
        bump
    )]
    pub pool_stake_vault: InterfaceAccount<'info, TokenAccount>,
    pub stake_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>
}
