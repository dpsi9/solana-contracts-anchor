use anchor_lang::prelude::*;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
};

use crate::{
    error::GovernanceError,
    events::ProposalExecuted,
    state::{Proposal, ProposalStatus, Realm},
};

pub fn process_execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let now = Clock::get()?.unix_timestamp;

    require!(
        proposal.status == ProposalStatus::Passed,
        GovernanceError::ProposalNotPassed
    );

    require!(
        now > proposal.execution_at,
        GovernanceError::ExecutionDelayNotMet
    );

    for ix in &proposal.execution_instruction {
        let accounts: Vec<AccountMeta> = ix
            .accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: acc.pubkey,
                is_signer: acc.is_signer,
                is_writable: acc.is_writeable,
            })
            .collect();

        let ix = Instruction {
            program_id: ix.program_id,
            accounts,
            data: ix.data.clone(),
        };

        let seeds: &[&[&[u8]]] = &[&[
            b"proposal",
            proposal.realm.as_ref(),
            proposal.proposer.as_ref(),
            &[proposal.bump],
        ]];
        invoke_signed(&ix, ctx.remaining_accounts, seeds)?;
    }

    proposal.status = ProposalStatus::Executed;
    emit!(ProposalExecuted {
        proposal: proposal.key()
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"realm", realm.authority.as_ref(), realm.name.as_bytes()],
        bump
    )]
    pub realm: Account<'info, Realm>,
    #[account(
        mut,
        seeds = [b"proposal", realm.key().as_ref(), proposal.proposer.as_ref()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
}
