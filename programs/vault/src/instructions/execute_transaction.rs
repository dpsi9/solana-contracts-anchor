use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

use crate::{
    error::VaultError,
    state::{Transaction, Vault},
};

pub fn process_execute_transaction(
    ctx: Context<ExecuteTransaction>,
    transaction_id: [u8; 8],
) -> Result<()> {
    let transaction = &mut ctx.accounts.transaction;
    let vault = &ctx.accounts.vault;

    require!(!transaction.executed, VaultError::TransactionExecuted); // check if already executed
    require!(
        transaction.approved_by.len() >= vault.threshold as usize,
        VaultError::InsufficientApprovals
    );

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"vault",
        vault.authority.as_ref(),
        vault.token_mint.as_ref(),
        &[vault.bump],
    ]];

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.token_vault.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer_seeds,
    );
    token_interface::transfer_checked(
        cpi_context,
        transaction.amount,
        ctx.accounts.token_mint.decimals,
    )?;

    transaction.executed = true;
    transaction.executed_at = Clock::get()?.unix_timestamp;

    msg!(
        "Transaction {:#?} executed: {} tokens send to {}",
        transaction_id,
        transaction.amount,
        transaction.recipient
    );
    Ok(())
}

#[derive(Accounts)]
#[instruction(transaction_id: [u8;8])]
pub struct ExecuteTransaction<'info> {
    #[account(mut)]
    pub executer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault.authority.as_ref(), vault.token_mint.as_ref()],
        bump = vault.bump,
        constraint = vault.owners.contains(&executer.key()) @ VaultError::Unauthorized,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"transaction", vault.key().as_ref(), &transaction_id],
        bump = transaction.bump,
    )]
    pub transaction: Account<'info, Transaction>,

    #[account(
        mut,
        constraint = token_vault.owner == vault.key() @ VaultError::InvalidTokenAccount,
        constraint = token_vault.key() == vault.token_vault @ VaultError::InvalidTokenAccount,
        constraint = token_vault.mint == vault.token_mint @ VaultError::InvalidTokenAccount,
        constraint = token_vault.mint == token_mint.key(),
    )]
    pub token_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = recipient_token_account.owner == transaction.recipient,
        constraint = recipient_token_account.mint == vault.token_mint
    )]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}
