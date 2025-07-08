use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

use crate::{error::ErrorCode, state::EscrowAccount};

/*
refund the tokens from escrow_token account to user token account
*/
pub fn process_refund(ctx: Context<Refund>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow_account;
    // check if the refuder is maker or not
    require!(
        ctx.accounts.maker.key() == escrow.maker,
        ErrorCode::InvalidTokenOwner,
    );

    // transfer from escrow_token_account to user_token_account
    let maker_key = escrow.maker.key();
    let mint_a_key = escrow.mint_a.key();
    let mint_b_key = escrow.mint_b.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"escrow",
        maker_key.as_ref(),
        mint_a_key.as_ref(),
        mint_b_key.as_ref(),
        &[escrow.bump],
    ]];
    token_interface::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.escrow_token_a_account.to_account_info(),
                mint: ctx.accounts.mint_a.to_account_info(),
                to: ctx.accounts.maker_token_a_account.to_account_info(),
                authority: escrow.to_account_info(),
            },
            signer_seeds,
        ),
        escrow.amount_a,
        ctx.accounts.mint_a.decimals,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct Refund<'info> {
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        constraint = maker_token_a_account.owner == maker.key() @ ErrorCode::InvalidTokenOwner,
        constraint = maker_token_a_account.mint == mint_a.key() @ ErrorCode::InvalidTokenMint
    )]
    pub maker_token_a_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump = escrow_account.bump
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(
        mut,
        constraint = escrow_token_a_account.owner == escrow_account.key() @ ErrorCode::InvalidTokenOwner,
        constraint = escrow_token_a_account.mint == mint_a.key() @ ErrorCode::InvalidTokenMint
    )]
    pub escrow_token_a_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
