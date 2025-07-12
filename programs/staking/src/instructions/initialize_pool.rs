use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::state::StakePool;

pub fn process_initialize_pool(ctx: Context<InitializePool>, reward_rate: u64, minimum_stake_duration: i64) -> Result<()> {
    let pool = &mut ctx.accounts.stake_pool;

    pool.authority = ctx.accounts.authority.key();
    pool.stake_mint = ctx.accounts.stake_mint.key();
    pool.reward_mint = ctx.accounts.reward_mint.key();
    pool.total_staked = 0;
    pool.reward_per_token_stored = 0;
    pool.reward_rate = reward_rate;
    pool.min_stake_duration = minimum_stake_duration;
    pool.last_updated = Clock::get()?.unix_timestamp;
    pool.bump = ctx.bumps.stake_pool;

    msg!("Staking Pool initialized! reward rate: {} per second", reward_rate);
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
        token::mint = stake_mint,
        token::authority = stake_pool,
        token::token_program = token_program,
        seeds = [b"stake_vault", stake_pool.key().as_ref(),stake_mint.key().as_ref()],
        bump
    )]
    pub pool_stake_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init, 
        payer = authority,
        token::mint = reward_mint,
        token::authority = stake_pool,
        token::token_program = token_program,
        seeds = [b"reward_vault", stake_pool.key().as_ref(), reward_mint.key().as_ref()],
        bump
    )]
    pub pool_reward_vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}
