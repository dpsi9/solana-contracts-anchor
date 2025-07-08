use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EscrowAccount {
    pub maker: Pubkey,  // who is creating the escrow
    pub mint_a: Pubkey, // type of mint offered
    pub mint_b: Pubkey, // type of mint expected in return
    pub amount_a: u64,  // amount of tokens offered
    pub amount_b: u64,  // amount of tokens expected
    pub bump: u8,       //bump of the current escrow
}
