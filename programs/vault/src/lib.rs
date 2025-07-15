use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub use instructions::*;
declare_id!("VauLt11111111111111111111111111111111111111");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        threshold: u8,
        owners: Vec<Pubkey>,
    ) -> Result<()> {
        process_initialize_vault(ctx, threshold, owners)
    }
}
