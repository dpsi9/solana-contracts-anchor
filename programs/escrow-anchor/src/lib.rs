use anchor_lang::prelude::*;
mod instructions;
use instructions::*;
mod error;
mod state;
declare_id!("D1DEx9xFn1Y3dRZbvD7M126nUhMMEFEtVkJZ3oihemDt");

#[program]
pub mod escrow_anchor {
    use super::*;

    pub fn make(ctx: Context<Make>, amount_offered: u64, amount_expected: u64) -> Result<()> {
        process_make(ctx, amount_offered, amount_expected)
    }

    pub fn take(ctx: Context<Take>, amount: u64) -> Result<()> {
        process_take(ctx, amount)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        process_refund(ctx)
    }
}
