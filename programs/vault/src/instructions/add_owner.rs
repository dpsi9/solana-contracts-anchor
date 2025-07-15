use anchor_lang::prelude::*;

use crate::{constants::MAX_OWNERS, error::VaultError, state::Vault};

pub fn process_add_owner(ctx: Context<AddOwner>, new_owner: Pubkey) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    require!(
        !vault.owners.contains(&new_owner),
        VaultError::OwnerAlreadyExists
    );
    require!(vault.owners.len() < MAX_OWNERS, VaultError::TooManyOwners);

    vault.owners.push(new_owner);

    msg!("Owner added: {}", new_owner);
    Ok(())
}

#[derive(Accounts)]
pub struct AddOwner<'info> {
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
