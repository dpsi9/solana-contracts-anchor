use anchor_lang::prelude::*;

use crate::constants::MAX_DESCRIPTION_LENGTH;
use crate::error::VaultError;
use crate::state::{Transaction, Vault};

pub fn process_propose_transaction(
    ctx: Context<ProposeTransaction>,
    transaction_id: [u8; 8],
    recipient: Pubkey,
    amount: u64,
    description: String,
) -> Result<()> {
    require!(amount > 0, VaultError::InvalidAmount);
    require!(
        description.len() <= MAX_DESCRIPTION_LENGTH,
        VaultError::DescriptionTooLong
    );

    let vault = &mut ctx.accounts.vault;
    let transaction = &mut ctx.accounts.transaction;

    transaction.vault = vault.key();
    transaction.proposer = ctx.accounts.owner.key();
    transaction.recipient = recipient;
    transaction.amount = amount;
    transaction.description = description;
    transaction.approved_by = Vec::new();
    transaction.executed = false;
    transaction.created_at = Clock::get()?.unix_timestamp;
    transaction.transaction_id = transaction_id;
    transaction.bump = ctx.bumps.transaction;

    // auto approve by proposer
    transaction.approved_by.push(ctx.accounts.owner.key());

    vault.transaction_count += 1;

    msg!(
        "Transaction {:?} proposed by {}, amount: {}",
        transaction_id,
        ctx.accounts.owner.key(),
        amount
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(transaction_id: [u8; 8])]
pub struct ProposeTransaction<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault.authority.as_ref(), vault.token_mint.as_ref()],
        bump = vault.bump,
        constraint = vault.owners.contains(&owner.key()) @ VaultError::Unauthorized
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        init,
        payer = owner,
        space = 8 + Transaction::INIT_SPACE,
        seeds = [b"transaction", vault.key().as_ref(), &transaction_id],
        bump
    )]
    pub transaction: Account<'info, Transaction>,
    pub system_program: Program<'info, System>,
}
