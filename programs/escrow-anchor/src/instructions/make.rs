use crate::error::ErrorCode;
use crate::state::EscrowAccount;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

/*
    create an escrow account and transfers user's tokens into the escrow token account
*/
pub fn process_make(ctx: Context<Make>, amount_offered: u64, amount_expected: u64) -> Result<()> {
    let escrow_account = &mut ctx.accounts.escrow_account;
    escrow_account.maker = ctx.accounts.maker.key();
    escrow_account.mint_a = ctx.accounts.mint_a.key();
    escrow_account.mint_b = ctx.accounts.mint_b.key();
    escrow_account.amount_a = amount_offered;
    escrow_account.amount_b = amount_expected;
    escrow_account.bump = ctx.bumps.escrow_account;

    require!(
        amount_offered <= ctx.accounts.maker_token_a_account.amount,
        ErrorCode::InvalidAmount
    ); // check if the tokens offered are availabe in maker's account

    // transfer tokens from maker_token_account_a to escrow_token_account_a
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.maker_token_a_account.to_account_info(),
            mint: ctx.accounts.mint_a.to_account_info(),
            to: ctx.accounts.escrow_token_account_a.to_account_info(),
            authority: ctx.accounts.maker.to_account_info(),
        },
    );
    token_interface::transfer_checked(cpi_ctx, amount_offered, ctx.accounts.mint_a.decimals)?;

    msg!(
        "Escrow created, offereing {} tokens for tokens {}",
        amount_offered,
        amount_expected
    );

    Ok(())
}

#[derive(Accounts)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>, // token offered
    pub mint_b: InterfaceAccount<'info, Mint>, // token expected
    #[account(
        init,
        payer = maker,
        space = 8 + EscrowAccount::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(
        mut,
        constraint = maker_token_a_account.owner == maker.key() @ ErrorCode::InvalidTokenMint,
        constraint = maker_token_a_account.mint == mint_a.key() @ ErrorCode::InvalidTokenOwner,
    )]
    pub maker_token_a_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        token::mint = mint_a,
        token::authority = escrow_account,
        token::token_program = token_program,
    )]
    pub escrow_token_account_a: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
