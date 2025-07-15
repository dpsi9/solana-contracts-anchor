use anchor_lang::prelude::*;

use crate::{error::VaultError, state::Vault};

pub fn process_remove_owner(ctx: Context<RemoveOwner>, owner_to_remove: Pubkey) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    // Check if owner exists or not
    require!(
        !vault.owners.contains(&owner_to_remove),
        VaultError::OwnerNotFound
    );

    // Only remove if owners are more than the required threshold
    require!(
        vault.owners.len() > vault.threshold as usize,
        VaultError::CannotRemoveOwner
    );

    // Now remove owner
    vault.owners.retain(|&x| x != owner_to_remove);

    msg!("Owner removed: {}", owner_to_remove);

    Ok(())
}

#[derive(Accounts)]
pub struct RemoveOwner<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault.authority.as_ref(), vault.token_mint.as_ref()],
        bump = vault.bump,
        constraint = vault.owners.contains(&owner.key()) @ VaultError::Unauthorized
    )]
    pub vault: Account<'info, Vault>,
}
