use anchor_lang::prelude::*;

use crate::{error::StakingError, state::StakePool, utility::update_pool_rewards};

pub fn process_update_pool(ctx: Context<UpdatePool>, new_reward_rate: Option<u64>, new_min_duration: Option<i64>) -> Result<()> {
    let pool = &mut ctx.accounts.stake_pool;
    let current_time = Clock::get()?.unix_timestamp;

    // update rewards before changing 
    update_pool_rewards(pool, current_time)?;

    // update reward rate if provided
    if let Some(rate) = new_reward_rate {
        pool.reward_rate = rate;
        msg!("Updated reward rate to: {}", rate);
    }
    
    // update min_duration if provided
    if let Some(duration) = new_min_duration {
        pool.min_stake_duration = duration;
        msg!("Updated minimum stake duration to: {}", duration);
    }
    Ok(())
}

#[derive(Accounts)]
pub struct UpdatePool<'info> {
    #[account(
        mut, 
        constraint = authority.key() == stake_pool.authority @ StakingError::Unauthorized
    )]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"stake_pool", stake_pool.stake_mint.as_ref(), stake_pool.reward_mint.as_ref()],
        bump = stake_pool.bump
    )]
    pub stake_pool: Account<'info, StakePool>,
}