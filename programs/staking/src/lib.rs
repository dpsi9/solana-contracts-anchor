use anchor_lang::prelude::*;
mod error;
mod instructions;
mod state;
mod utility;
use instructions::*;
declare_id!("StaKe11111111111111111111111111111111111111");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        reward_rate: u64,
        minimum_stake_duration: i64,
    ) -> Result<()> {
        process_initialize_pool(ctx, reward_rate, minimum_stake_duration)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        process_stake(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        process_unstake(ctx, amount)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        process_claim_rewards(ctx)
    }

    pub fn update_pool(
        ctx: Context<UpdatePool>,
        new_reward_rate: Option<u64>,
        new_min_duration: Option<i64>,
    ) -> Result<()> {
        process_update_pool(ctx, new_reward_rate, new_min_duration)
    }
}
