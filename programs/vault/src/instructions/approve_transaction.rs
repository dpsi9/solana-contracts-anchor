use anchor_lang::prelude::*;

use crate::{
    error::VaultError,
    state::{Transaction, Vault},
};

pub fn process_approve_transaction(
    ctx: Context<ApproveTransaction>,
    _transaction_id: [u8; 8],
) -> Result<()> {
    let transaction = &mut ctx.accounts.transaction;
    let owner = ctx.accounts.owner.key();
    // Check if already approved
    require!(
        !transaction.approved_by.contains(&owner),
        VaultError::AlreadyApproved
    );

    // Check if transaction is not executed
    require!(!transaction.executed, VaultError::TransactionExecuted);

    transaction.approved_by.push(owner);

    msg!(
        "Transaction approved by {}, approvals: {}/{}",
        owner,
        transaction.approved_by.len(),
        ctx.accounts.vault.threshold
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(transaction_id: [u8; 8])]
pub struct ApproveTransaction<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        seeds = [b"vault", vault.authority.as_ref(), vault.token_mint.as_ref()],
        bump = vault.bump,
        constraint = vault.owners.contains(&owner.key()) @ VaultError::Unauthorized,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"transaction", vault.key().as_ref(), &transaction_id],
        bump = transaction.bump,
    )]
    pub transaction: Account<'info, Transaction>,
}
