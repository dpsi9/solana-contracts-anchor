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

    pub fn propose_transaction(
        ctx: Context<ProposeTransaction>,
        transaction_id: [u8; 8],
        recipient: Pubkey,
        amount: u64,
        description: String,
    ) -> Result<()> {
        process_propose_transaction(ctx, transaction_id, recipient, amount, description)
    }

    pub fn approve_transaction(
        ctx: Context<ApproveTransaction>,
        transaction_id: [u8; 8],
    ) -> Result<()> {
        process_approve_transaction(ctx, transaction_id)
    }

    pub fn execute_transaction(
        ctx: Context<ExecuteTransaction>,
        transaction_id: [u8; 8],
    ) -> Result<()> {
        process_execute_transaction(ctx, transaction_id)
    }
}
