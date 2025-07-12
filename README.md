# 🚀 Solana Smart Contracts Collection

A comprehensive collection of Solana smart contracts built with the Anchor framework for learning and interview preparation. This repository contains multiple contract implementations showcasing different DeFi and Web3 concepts commonly asked about in blockchain developer interviews.

## 📋 Table of Contents

- [Overview](#overview)
- [Contracts](#contracts)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)
- [Project Structure](#project-structure)
- [Contract Details](#contract-details)
- [Development](#development)
- [Contributing](#contributing)

## 🎯 Overview

This repository contains production-ready Solana smart contracts implemented using the Anchor framework. Each contract is designed to demonstrate core blockchain concepts and best practices in Solana development.

**Built with:**
- ⚡ **Anchor Framework v0.31.1** - Solana's preferred development framework
- 🦀 **Rust** - Systems programming language for high-performance contracts
- 🧪 **TypeScript** - For testing and client interactions
- 🔧 **Solana CLI** - For deployment and interaction

## 📦 Contracts

### ✅ Implemented Contracts

1. **🔐 Escrow Contract** - Trustless token exchange between parties
2. **🏦 Staking Contract** - Token staking with rewards mechanism

### 🚧 In Development

3. **🏪 Vault Contract** - Secure token storage and management
4. **🛒 Marketplace Contract** - NFT marketplace for trading digital assets
5. **🗳️ Governance Contract** - DAO voting and proposal system

## 🛠 Prerequisites

Before running this project, ensure you have the following installed:

- **Rust** (latest stable version)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Solana CLI** (v1.18.0 or higher)
  ```bash
  sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
  ```

- **Anchor CLI** (v0.31.1)
  ```bash
  npm install -g @coral-xyz/anchor-cli
  ```

- **Node.js** (v16 or higher) and **Yarn**
  ```bash
  npm install -g yarn
  ```

## 🚀 Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/dpsi9/solana-contracts-anchor.git
   cd solana-contracts-anchor
   ```

2. **Install dependencies**
   ```bash
   yarn install
   ```

3. **Build all contracts**
   ```bash
   anchor build
   ```

4. **Generate TypeScript types**
   ```bash
   anchor gen
   ```

## 🎮 Usage

### Building Contracts

```bash
# Build all contracts
anchor build

# Build specific contract
anchor build --program-name escrow_anchor
anchor build --program-name staking
```

### Deploying to Localnet

1. **Start local validator**
   ```bash
   solana-test-validator
   ```

2. **Deploy contracts**
   ```bash
   anchor deploy
   ```

### Running Tests

```bash
# Run all tests
anchor test

# Run specific test file
anchor test tests/escrow-anchor.ts
```

## 🏗 Project Structure

```
├── Anchor.toml                 # Anchor configuration
├── Cargo.toml                  # Workspace configuration
├── package.json               # Node.js dependencies
├── tsconfig.json              # TypeScript configuration
├── programs/                  # Smart contracts
│   ├── escrow-anchor/         # ✅ Escrow contract
│   │   ├── src/
│   │   │   ├── lib.rs         # Main program entry
│   │   │   ├── state.rs       # Account structures
│   │   │   ├── error.rs       # Custom errors
│   │   │   └── instructions/  # Instruction handlers
│   │   └── Cargo.toml
│   ├── staking/               # ✅ Staking contract
│   │   ├── src/
│   │   │   ├── lib.rs         # Main program entry
│   │   │   ├── state.rs       # Account structures
│   │   │   ├── error.rs       # Custom errors
│   │   │   ├── utility.rs     # Helper functions
│   │   │   └── instructions/  # Instruction handlers
│   │   └── Cargo.toml
│   ├── vault/                 # 🚧 Vault contract (template)
│   ├── marketplace/           # 🚧 Marketplace contract (template)
│   └── governance/            # 🚧 Governance contract (template)
├── tests/                     # Integration tests
├── target/                    # Build artifacts
│   ├── deploy/               # Deployed program binaries
│   ├── idl/                  # Interface Definition Language files
│   └── types/                # Generated TypeScript types
└── migrations/               # Deployment scripts
```

## 📚 Contract Details

### 🔐 Escrow Contract

**Program ID:** `D1DEx9xFn1Y3dRZbvD7M126nUhMMEFEtVkJZ3oihemDt`

A trustless escrow system that allows two parties to exchange tokens safely without requiring a trusted third party.

#### Features:
- 🔒 **Make Escrow** - Create an escrow offer with specified tokens
- 🤝 **Take Escrow** - Accept and complete the token exchange
- 💰 **Refund** - Cancel escrow and retrieve deposited tokens
- 🛡️ **Security** - Built-in safety checks and validations

#### Instructions:
```rust
pub fn make(ctx: Context<Make>, amount_offered: u64, amount_expected: u64) -> Result<()>
pub fn take(ctx: Context<Take>, amount: u64) -> Result<()>
pub fn refund(ctx: Context<Refund>) -> Result<()>
```

#### Use Cases:
- Token swaps between different SPL tokens
- OTC (Over-the-counter) trading
- Conditional payments
- Cross-chain atomic swaps

---

### 🏦 Staking Contract

**Program ID:** `StaKe11111111111111111111111111111111111111`

A comprehensive staking system that allows users to stake tokens and earn rewards over time with configurable parameters.

#### Features:
- 🏊‍♂️ **Initialize Pool** - Set up staking pools with custom reward rates
- 📈 **Stake Tokens** - Deposit tokens to earn rewards
- 📉 **Unstake Tokens** - Withdraw staked tokens (with time locks)
- 🎁 **Claim Rewards** - Harvest earned staking rewards
- ⚙️ **Update Pool** - Modify pool parameters (admin only)

#### Instructions:
```rust
pub fn initialize_pool(ctx: Context<InitializePool>, reward_rate: u64, minimum_stake_duration: i64) -> Result<()>
pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()>
pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()>
pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()>
pub fn update_pool(ctx: Context<UpdatePool>, new_reward_rate: Option<u64>, new_min_duration: Option<i64>) -> Result<()>
```

#### Key Features:
- ⏰ **Time-based Rewards** - Calculate rewards based on staking duration
- 🔒 **Minimum Stake Duration** - Prevent flash loans and ensure commitment
- 👑 **Admin Controls** - Pool parameter updates for governance
- 📊 **Accurate Accounting** - Precise reward calculations with no rounding errors

#### Use Cases:
- DeFi yield farming
- Governance token staking
- Liquidity mining programs
- Token distribution mechanisms

## 🧪 Testing

The project includes comprehensive test suites for each contract:

```bash
# Run all tests
yarn test

# Run with specific test timeout
anchor test --timeout 60000

# Run tests with detailed output
ANCHOR_LOG=true anchor test
```

### Test Coverage:
- ✅ Contract deployment
- ✅ Instruction execution
- ✅ Error handling
- ✅ State validation
- ✅ Edge cases

## 🔧 Development

### Adding New Contracts

1. Create a new program directory in `programs/`
2. Set up the `Cargo.toml` with proper dependencies
3. Implement the contract in `src/lib.rs`
4. Add the program to `Anchor.toml`
5. Create corresponding tests

### Code Standards

- **Rust**: Follow standard Rust conventions and clippy suggestions
- **Security**: Implement proper access controls and input validation
- **Documentation**: Add comprehensive comments and documentation
- **Testing**: Maintain high test coverage for all functionality

### Useful Commands

```bash
# Format code
cargo fmt

# Run clippy for linting
cargo clippy

# Check for updates
anchor --version

# Clean build artifacts
anchor clean

# Generate new keypair
solana-keygen new
```

## 🎓 Learning Resources

### Concepts Demonstrated:

1. **Program Derived Addresses (PDAs)** - For secure, deterministic account generation
2. **Cross-Program Invocations (CPIs)** - For interacting with other programs
3. **Account Validation** - Using Anchor's constraint system
4. **Error Handling** - Custom error types and proper error propagation
5. **State Management** - Efficient account structure design
6. **Security Patterns** - Access control and input validation

### Interview Topics Covered:

- 🔐 **Escrow Mechanisms** - Trustless exchanges
- 🏦 **Staking Systems** - Reward calculations and time locks
- 🏪 **Token Vaults** - Secure asset storage
- 🛒 **Marketplace Logic** - Buy/sell mechanisms
- 🗳️ **Governance Systems** - DAO voting patterns

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the ISC License - see the LICENSE file for details.

## 🔗 Additional Resources

- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Solana Program Library](https://spl.solana.com/)

---

**Note:** This repository is designed for educational purposes and interview preparation. While the contracts follow security best practices, they should be thoroughly audited before any production use.

Built with ❤️ for the Solana ecosystem
