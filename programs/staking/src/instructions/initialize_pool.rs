use crate::state::{StakePool, UserStakeAccount};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

pub fn process_initialize_pool(
    ctx: Context<InitializePool>,
    reward_rate: u64,
    min_stake_duration: i64,
) -> Result<()> {
    *ctx.accounts.stake_pool = StakePool {
        authority: ctx.accounts.authority.key(),
        stake_mint: ctx.accounts.stake_mint.key(),
        reward_mint: ctx.accounts.reward_mint.key(),
        reward_vault: ctx.accounts.reward_vault.key(),
        reward_rate: reward_rate,
        min_stake_duration: min_stake_duration,
        total_staked: 0,
        last_update_time: Clock::get()?.unix_timestamp,
        reward_per_token_stored: 0,
        bump: ctx.bumps.stake_pool,
    };
    Ok(())
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub stake_mint: InterfaceAccount<'info, Mint>,
    pub reward_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = authority,
        space = 8 + StakePool::INIT_SPACE,
        seeds = [b"stake_pool", stake_mint.key().as_ref(), reward_mint.key().as_ref()],
        bump
    )]
    pub stake_pool: Account<'info, StakePool>,
    #[account(
        init,
        payer = authority,
        token::mint = reward_mint,
        token::authority = stake_pool,
        token::token_program = token_program,
    )]
    pub reward_vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}
