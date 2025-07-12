use crate::state::{StakePool, UserStakeAccount};
use anchor_lang::prelude::*;
pub fn update_pool_rewards(pool: &mut StakePool, current_time: i64) -> Result<()> {
    if pool.total_staked > 0 {
        let time_elapsed = current_time - pool.last_updated;
        let reward_per_token_increment = (time_elapsed as u64)
            .checked_mul(pool.reward_rate)
            .unwrap()
            .checked_div(pool.total_staked)
            .unwrap_or(0);
        pool.reward_per_token_stored = pool
            .reward_per_token_stored
            .checked_add(reward_per_token_increment)
            .unwrap();
    }
    pool.last_updated = current_time;
    Ok(())
}

pub fn calculate_pending_rewards(user_stake: &UserStakeAccount, pool: &StakePool) -> Result<u64> {
    let reward_per_token_diff = pool
        .reward_per_token_stored
        .checked_sub(user_stake.reward_per_token_paid)
        .unwrap();

    let new_rewards = user_stake
        .amount_staked
        .checked_mul(reward_per_token_diff)
        .unwrap()
        .checked_div(1_000_000_000)
        .unwrap();

    Ok(new_rewards)
}
