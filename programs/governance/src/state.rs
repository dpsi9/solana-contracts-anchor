use anchor_lang::prelude::*;

/// States - Realm, Propsal, Vote Record, Delegated Votes

/// Realm account represents a governance space (like a DAO)
/// It stores governance settings and tracks proposals.
#[account]
pub struct Realm {
    pub authority: Pubkey,                   // Admin authority of the Realm
    pub name: String,                        // Name of the realm
    pub voting_mint: Pubkey,                 // SPL token used for voting
    pub min_tokens_to_create_proposals: u64, // Threashold to create proposal
    pub voting_duration: i64,                // Time voting is open in seconds
    pub execution_delay: i64,                // Delay before execution after passing
    pub quorum_threshold: u64,               // Minimum % of total votes required
    pub approval_threshold: u64,             // % of yes votes required for approval
    pub proposal_count: u64,                 // Auto-incrementing counter for proposals
    pub bump: u8,                            // PDA bump
}

/// Proposals account defines a governance proposal.
#[account]
pub struct Proposal {
    pub realm: Pubkey,               // Name of realm this proposal belongs to
    pub proposer: Pubkey,            // Who proposed this proposal
    pub title: String,               // Proposal title
    pub ipfs_hash: [u8; 46],         // CID pointing data(Description) stored on IPFS
    pub proposal_type: ProposalType, // Type of proposal
    pub execution_instruction: Vec<ProposalInstruction>, // Instruction information to execute after passing
    pub created_at: i64,                                 // Time of proposal creation
    pub voting_starts_at: i64,                           // Time of voting start
    pub voting_ends_at: i64,                             // Time of voting end
    pub execution_at: i64,                               // Time of execution of instruction
    pub yes_votes: u64,                                  // No. of yes votes
    pub no_votes: u64,                                   // No. of no votes
    pub abstain_votes: u64,                              // No. of abstain votes
    pub status: ProposalStatus,                          // Proposal's current status
    pub bump: u8,                                        // Proposal PDA bump
}

/// VoteRecord account represents every user's voting details
/// Prevents double voting, ties the voter key to the proposal key
#[account]
pub struct VoteRecord {
    pub voter: Pubkey,       // Account participated in voting
    pub proposal: Pubkey,    // Proposal to vote
    pub vote_type: VoteType, // Vote type - yes, no, abstain
    pub weight: u64,         // Weight, power of vote, directly proportional to the tokens held
    pub timestamp: i64,      // Time of voting
    pub bump: u8,            // PDA bump
}

/// DelegatedVotes account represent delegate votes information
/// Stores information about delegator and delegatee
#[account]
pub struct DelegatedVotes {
    pub voter: Pubkey,     // Person who is delegating their vote
    pub delegatee: Pubkey, // Person who will vote on behalf
    pub amount: u64,       // how many tokens were delegated
    pub bump: u8,          // pda bump
}

/// Type of proposals supported by the Realm.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalType {
    /// Simple discussion no on-chain effect eg. should we expand our community.
    Text,
    /// Executes on-chain login like CPI calls, eg. transfer X tokens to Y address.
    Executable,
    /// Allocates or transfers treasury funds, eg. Fund grant recipients with 1000 tokens.
    Treasury,
    /// Update DAO parameters like voting duration, quorum threshold, etc.
    Parameters,
}

/// Status of proposal during its lifecycle.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Voting,
    Passed,
    Failed,
    Executed,
}

/// Type of vote cast on proposal.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum VoteType {
    Yes,
    No,
    Abstain,
}

/// Represents a single on-chain instruction to be executed if proposal passes
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ProposalInstruction {
    pub program_id: Pubkey, // Target program (eg. token program, staking program ,etc)
    pub accounts: Vec<ProposalAccountMeta>, // Accounts involved in this instruction
    pub data: Vec<u8>,      // Encoded instruction data
}

/// Metadata for accounts involved in proposal instruction.
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ProposalAccountMeta {
    pub pubkey: Pubkey,     // Involved account address.
    pub is_signer: bool,    // Does it need to sign the transaction?
    pub is_writeable: bool, // Is this account's state going to change?
}
