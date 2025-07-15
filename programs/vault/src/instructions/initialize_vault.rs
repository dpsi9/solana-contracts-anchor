use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::constants::MAX_OWNERS;
use crate::error::VaultError;
use crate::state::Vault;

pub fn process_initialize_vault(
    ctx: Context<InitializeVault>,
    threshold: u8,
    owners: Vec<Pubkey>,
) -> Result<()> {
    require!(threshold > 0, VaultError::InvalidThreshold);
    require!(owners.len() > 0, VaultError::NoOwners);
    require!(
        threshold <= owners.len() as u8,
        VaultError::ThresholdTooHigh
    );
    require!(owners.len() <= MAX_OWNERS, VaultError::TooManyOwners);

    let vault = &mut ctx.accounts.vault;

    vault.authority = ctx.accounts.authority.key();
    vault.token_mint = ctx.accounts.token_mint.key();
    vault.token_vault = ctx.accounts.token_vault.key();
    vault.threshold = threshold;
    vault.owners = owners.clone();
    vault.transaction_count = 0;
    vault.bump = ctx.bumps.vault;

    msg!(
        "Vault initialized with {} owners, threshold: {}",
        owners.len(),
        threshold
    );

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = authority,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", authority.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = vault,
        token::token_program = token_program,
    )]
    pub token_vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
