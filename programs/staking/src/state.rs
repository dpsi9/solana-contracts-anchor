use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakePool {
    pub authority: Pubkey,
    pub stake_mint: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_rate: u64,
    pub reward_vault: Pubkey,
    pub min_stake_duration: i64,
    pub total_staked: u64,
    pub last_update_time: i64,
    pub reward_per_token_stored: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct UserStakeAccount {
    pub user: Pubkey,
    pub stake_pool: Pubkey,
    pub amount_staked: u64,
    pub pending_rewards: u64,
    pub rewards_per_token_paid: u64,
    pub stake_start_time: i64,
    pub bump: u8,
}
