use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

use crate::state::{StakePool, UserStakeAccount};
use crate::error::StakingError;
use crate::utility::{calculate_pending_rewards, update_pool_rewards};

pub fn process_stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
    require!(amount > 0, StakingError::InvalidAmount);

    let current_time = Clock::get()?.unix_timestamp;
    //update pool rewards
    {
        let pool = &mut ctx.accounts.stake_pool;
        update_pool_rewards(pool, current_time)?;
    }
    let user_stake = &mut ctx.accounts.user_stake_account;

    if user_stake.user == Pubkey::default() {
        user_stake.user = ctx.accounts.user.key();
        user_stake.stake_pool = ctx.accounts.stake_pool.key();
        user_stake.amount_staked = 0;
        user_stake.pending_rewards = 0;
        user_stake.stake_start_time = current_time;
        user_stake.reward_per_token_paid = ctx.accounts.stake_pool.reward_per_token_stored;
        user_stake.bump = ctx.bumps.user_stake_account;
    } else {
        user_stake.pending_rewards = calculate_pending_rewards(user_stake, &ctx.accounts.stake_pool)?;
        user_stake.reward_per_token_paid = ctx.accounts.stake_pool.reward_per_token_stored;
    }

    token_interface::transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            TransferChecked{
                from: ctx.accounts.user_token_account.to_account_info(),
                mint: ctx.accounts.stake_mint.to_account_info(),
                to: ctx.accounts.pool_stake_vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            })
        ,amount, ctx.accounts.stake_mint.decimals)?;

    //update state
    user_stake.amount_staked = user_stake.amount_staked.checked_add(amount).unwrap();
    let pool = &mut ctx.accounts.stake_pool;
    pool.total_staked = pool.total_staked.checked_add(amount).unwrap();

    msg!("Staked {} token. Total staked {}", amount, user_stake.amount_staked);
    
    Ok(())
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"stake_pool", stake_pool.stake_mint.as_ref(), stake_pool.reward_mint.as_ref()],
        bump = stake_pool.bump
    )]
    pub stake_pool: Account<'info, StakePool>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserStakeAccount::INIT_SPACE,
        seeds = [b"user_stake", stake_pool.key().as_ref(), user.key().as_ref()],
        bump,
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
        seeds = [b"stake_vault", stake_pool.key().as_ref(),stake_pool.stake_mint.as_ref()],
        bump
    )]
    pub pool_stake_vault: InterfaceAccount<'info, TokenAccount>,
    pub stake_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info,  System>,
}
