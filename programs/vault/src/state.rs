use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub threshold: u8,
    #[max_len(10)]
    pub owners: Vec<Pubkey>,
    pub transaction_count: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Transaction {
    pub vault: Pubkey,
    pub proposer: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    #[max_len(200)]
    pub description: String,
    #[max_len(10)]
    pub approved_by: Vec<Pubkey>,
    pub executed: bool,
    pub created_at: u64,
    pub executed_at: u64,
    pub bump: u8,
}
