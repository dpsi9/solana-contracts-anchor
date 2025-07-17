use anchor_lang::prelude::*;

pub mod state;
pub mod error;
pub mod events;

pub mod instructions;
pub use instructions::*;

declare_id!("Gover11111111111111111111111111111111111111");

#[program]
pub mod governance {
    use super::*;
    
}

