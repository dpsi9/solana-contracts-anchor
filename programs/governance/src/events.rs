use anchor_lang::prelude::*;

#[event]
pub struct RealmInitiated {
    pub realm: Pubkey,
    pub authority: Pubkey,
}
