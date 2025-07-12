use anchor_lang::prelude::*;
mod error;
mod instructions;
mod state;
mod utility;
declare_id!("StaKe11111111111111111111111111111111111111");

#[program]
pub mod staking {
    use crate::instructions::{process_initialize_pool, InitializePool};

    use super::*;

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        reward_rate: u64,
        minimum_stake_duration: i64,
    ) -> Result<()> {
        process_initialize_pool(ctx, reward_rate, minimum_stake_duration)
    }
}
