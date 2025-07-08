use anchor_lang::prelude::*;
use anchor_spl::token_2022::TransferChecked;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface};

use crate::error::ErrorCode;
use crate::state::EscrowAccount;

/*
transfer tokens from taker_token_account to maker_token_account and transfer tokens from escrow_token_account to taker_token_account
*/
pub fn process_take(ctx: Context<Take>, amount: u64) -> Result<()> {
    let escrow_account = &mut ctx.accounts.escrow_account;
    require!(amount == escrow_account.amount_b, ErrorCode::InvalidAmount); // amount to be exchanged should be equal to the requested amount

    // transfer tokens from taker_token_b_account to maker_token_b_account
    token_interface::transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.taker_token_b_account.to_account_info(),
                mint: ctx.accounts.mint_b.to_account_info(),
                to: ctx.accounts.maker_token_b_account.to_account_info(),
                authority: ctx.accounts.taker.to_account_info(),
            },
        ),
        escrow_account.amount_b,
        ctx.accounts.mint_b.decimals,
    )?;

    // transfer tokens from escrow_token_account_a to taker_token_a_account
    let maker_key = escrow_account.maker.key();
    let mint_a_key = escrow_account.mint_a.key();
    let mint_b_key = escrow_account.mint_b.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"escrow",
        maker_key.as_ref(),
        mint_a_key.as_ref(),
        mint_b_key.as_ref(),
        &[escrow_account.bump],
    ]];
    token_interface::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.escrow_token_account_a.to_account_info(),
                mint: ctx.accounts.mint_a.to_account_info(),
                to: ctx.accounts.taker_token_a_account.to_account_info(),
                authority: escrow_account.to_account_info(),
            },
            signer_seeds,
        ),
        escrow_account.amount_a,
        ctx.accounts.mint_a.decimals,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct Take<'info> {
    pub taker: Signer<'info>,
    pub maker: AccountInfo<'info>,
    #[account(
        mut,
        constraint = maker_token_b_account.owner == maker.key()  @ ErrorCode::InvalidTokenOwner,
        constraint = maker_token_b_account.mint == mint_b.key() @ ErrorCode::InvalidTokenMint,
    )]
    pub maker_token_b_account: InterfaceAccount<'info, TokenAccount>,
    pub mint_a: InterfaceAccount<'info, Mint>, // token offered
    pub mint_b: InterfaceAccount<'info, Mint>, // token expected
    #[account(
        mut,
        seeds = [b"escrow", maker.key().as_ref(), mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump = escrow_account.bump
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(
        mut,
        constraint = escrow_token_account_a.owner == escrow_account.key() @ ErrorCode::InvalidTokenOwner,
        constraint = escrow_token_account_a.mint == mint_a.key() @ ErrorCode::InvalidTokenMint,
    )]
    pub escrow_token_account_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = taker_token_a_account.owner == taker.key()  @ ErrorCode::InvalidTokenOwner,
        constraint = taker_token_a_account.mint == mint_a.key() @ ErrorCode::InvalidTokenMint,
    )]
    pub taker_token_a_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = taker_token_b_account.owner == taker.key() @ ErrorCode::InvalidTokenOwner,
        constraint = taker_token_b_account.mint == mint_b.key()  @ ErrorCode::InvalidTokenMint,
    )]
    pub taker_token_b_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
