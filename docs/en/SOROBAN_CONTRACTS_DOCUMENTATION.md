# Technical Documentation: Soroban Contracts Workspace

## Table of Contents

1. [Introduction](#introduction)
2. [Architecture and Project Structure](#architecture-and-project-structure)
3. [Workspace Configuration](#workspace-configuration)
4. [Libraries and Dependencies](#libraries-and-dependencies)
5. [Development Guide](#development-guide)
6. [Compilation and Testing](#compilation-and-testing)
7. [Deployment and Testing on Local Network](#deployment-and-testing-on-local-network)
8. [Code Examples](#code-examples)
9. [Best Practices](#best-practices)
10. [Troubleshooting](#troubleshooting)
11. [References and Resources](#references-and-resources)

---

## Introduction

This document provides a comprehensive technical guide for the Stellar Soroban smart contracts workspace located at `/home/carlos/dispensario_digital_sc/contracts`. The workspace is configured as an autonomous Cargo project that enables efficient development, compilation, and testing of multiple Soroban smart contracts.

### What is Soroban?

Soroban is Stellar's smart contract platform, designed to be secure, scalable, and efficient. Contracts are written in Rust and compiled to WebAssembly (WASM), enabling deterministic and secure execution on the Stellar network.

### Workspace Objectives

- **Modularity**: Each contract is an independent project within the workspace
- **Reusability**: Shared dependencies across contracts
- **Optimization**: Configuration optimized for minimal WASM binary size
- **Efficient Development**: Tools and configuration ready for development

---

## Architecture and Project Structure

### Directory Structure

```
contracts/
├── Cargo.toml                 # Workspace configuration
├── Cargo.lock                 # Dependency lock file (generated)
├── rust-toolchain.toml        # Rust toolchain configuration
├── target/                    # Build directory (generated)
│   └── wasm32v1-none/
│       └── release/
│           └── *.wasm         # Compiled binaries
└── hello-world/               # Example contract
    ├── Cargo.toml             # Contract configuration
    └── src/
        ├── lib.rs             # Contract source code
        └── test.rs            # Unit tests
```

### Main Components

#### 1. Cargo.toml (Workspace Root)

Main workspace configuration file that defines:
- Workspace members (contracts)
- Shared dependencies
- Optimized compilation profiles

#### 2. rust-toolchain.toml

Specifies the exact Rust version and targets needed for WASM compilation.

#### 3. Individual Contracts

Each contract is an independent Cargo project with its own `Cargo.toml` and source code.

---

## Workspace Configuration

### Workspace Cargo.toml

```toml
[workspace]
members = ["hello-world"]
resolver = "2"
exclude = ["target"]
```

**Configuration Explanation:**

- **`members`**: List of directories containing contracts. Add new contracts here.
- **`resolver = "2"`**: Uses Cargo resolver v2 for better dependency resolution.
- **`exclude`**: Directories to exclude from workspace (like `target/` build directory).

### Workspace Dependencies

Shared dependencies are defined in `[workspace.dependencies]`:

```toml
[workspace.dependencies]
soroban-sdk = "23.1.0"
stellar-access = { git = "https://github.com/OpenZeppelin/stellar-contracts", tag = "v0.5.1", package = "stellar-access" }
stellar-macros = { git = "https://github.com/OpenZeppelin/stellar-contracts", tag = "v0.5.1", package = "stellar-macros" }
stellar-tokens = { git = "https://github.com/OpenZeppelin/stellar-contracts", tag = "v0.5.1", package = "stellar-tokens" }
```

**Advantages:**
- Consistent versions across all contracts
- Centralized updates
- Less code duplication

### Release Compilation Profile

```toml
[profile.release]
opt-level = "z"              # Optimization for minimum size
debug = false                # No debug information
lto = true                   # Link-Time Optimization
debug-assertions = false     # No debug assertions
codegen-units = 1           # Single code unit for better optimization
panic = "abort"             # Abort on panic (reduces size)
overflow-checks = true      # Check overflow (security)
strip = true                # Remove debug symbols
```

**Why These Optimizations?**

In WASM contracts, binary size directly affects deployment and execution costs. These configurations minimize size while maintaining security.

### Rust Toolchain Configuration

```toml
[toolchain]
channel = "1.89.0"
targets = ["wasm32v1-none"]
```

**Specifications:**
- **Rust 1.89.0**: Specific version required for Soroban SDK compatibility
- **wasm32v1-none**: Target for WASM compilation compatible with Soroban v1

---

## Libraries and Dependencies

### 1. soroban-sdk (v23.1.0)

**Description:**
The official Soroban SDK provides all necessary APIs for interacting with the Soroban execution environment.

**Main Components:**

#### Env (Execution Environment)
- **Persistent Storage**: Persistent data storage
- **Temporary Storage**: Temporary storage (lasts only the transaction)
- **Events**: Event system for logging
- **Crypto**: Cryptographic functions
- **Ledger Info**: Ledger information (time, sequence, etc.)

#### Data Types
- **Address**: Contract and account addresses
- **BytesN**: Fixed-size byte arrays
- **Symbol**: Symbols (optimized short strings)
- **String**: Dynamic strings
- **Vec**: Dynamic vectors
- **Map**: Key-value maps
- **I128, I256, U128, U256**: Arbitrary precision integers

#### Macros
- **`#[contract]`**: Marks a struct as a contract
- **`#[contractimpl]`**: Implements contract functions
- **`#[contracterror]`**: Defines custom errors

**Usage Example:**

```rust
use soroban_sdk::{contract, contractimpl, Env, Address, Symbol};

#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    pub fn store(env: Env, key: Symbol, value: Address) {
        env.storage().persistent().set(&key, &value);
    }
    
    pub fn retrieve(env: Env, key: Symbol) -> Address {
        env.storage().persistent().get(&key).unwrap()
    }
}
```

**Special Features:**
- **Testutils**: Feature for testing that provides `Env::default()` and mock functions
- **Determinism**: All operations are deterministic
- **Gas Accounting**: All operations have associated gas costs

### 2. stellar-access (v0.5.1)

**Description:**
OpenZeppelin library for access control in Stellar contracts. Provides common authorization patterns.

**Components:**

#### Ownable
Ownership pattern where a contract has a single owner.

```rust
use stellar_access::Ownable;

#[contract]
pub struct OwnableContract {
    ownable: Ownable,
}

#[contractimpl]
impl OwnableContract {
    pub fn only_owner(env: Env) {
        ownable.require_owner(&env);
        // Only owner can execute this
    }
}
```

#### Roles
Role system for granular access control.

**Use Cases:**
- Administrators
- Operators
- Custom roles

**Advantages:**
- Reuse of tested code
- Audited security
- Industry standard patterns

### 3. stellar-macros (v0.5.1)

**Description:**
Procedural macros that reduce boilerplate in Stellar contracts.

**Available Macros:**
- Macros for events
- Macros for storage
- Macros for validation

**Example:**

```rust
use stellar_macros::*;

// Simplifies event and storage definitions
```

**Benefits:**
- Less repetitive code
- Lower error probability
- More readable code

### 4. stellar-tokens (v0.5.1)

**Description:**
Standard token implementations for Stellar, similar to ERC-20 and ERC-721 from Ethereum.

**Token Types:**

#### Fungible Tokens (ERC-20-like)
- Token transfers
- Approvals and allowances
- Minting and burning
- Allowlists

#### Non-Fungible Tokens (ERC-721-like)
- Unique asset tokenization
- Token enumeration
- Metadata

**Usage Example:**

```rust
use stellar_tokens::FungibleToken;

#[contract]
pub struct MyToken {
    token: FungibleToken,
}

#[contractimpl]
impl MyToken {
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        self.token.transfer(&env, &from, &to, amount);
    }
}
```

**Features:**
- Complete and tested implementation
- Standards compatible
- Gas optimized

---

## Development Guide

### Creating a New Contract

#### Step 1: Create Structure

```bash
cd /home/carlos/dispensario_digital_sc/contracts
mkdir my-new-contract
cd my-new-contract
mkdir src
```

#### Step 2: Create Cargo.toml

```toml
[package]
name = "my-new-contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
soroban-sdk = { workspace = true }

[dev-dependencies]
soroban-sdk = { workspace = true, features = ["testutils"] }

[package.metadata.stellar]
cargo_inherit = true
```

**Explanation:**
- **`crate-type = ["cdylib"]`**: Required to compile as dynamic library (WASM)
- **`cargo_inherit = true`**: Inherits workspace configuration

#### Step 3: Create lib.rs

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    pub fn greet(env: Env) -> String {
        String::from_str(&env, "Hello from Soroban!")
    }
}

#[cfg(test)]
mod test;
```

**Important Points:**
- **`#![no_std]`**: Required - Soroban doesn't use Rust stdlib
- **`#[contract]`**: Marks the struct as a contract
- **`#[contractimpl]`**: Implements public contract functions

#### Step 4: Add to Workspace

Edit `contracts/Cargo.toml`:

```toml
[workspace]
members = ["hello-world", "my-new-contract"]
```

### Contract Structure

#### Basic Components

1. **Imports**: Import types and functions from SDK
2. **Contract Struct**: Struct marked with `#[contract]`
3. **Implementation**: Functions marked with `#[contractimpl]`
4. **Tests**: Test module (optional but recommended)

#### Complete Example

```rust
#![no_std]
use soroban_sdk::{
    contract, contractimpl,
    symbol_short,
    Env, Symbol, String, Vec,
    Address,
    storage::Storage,
};

#[contract]
pub struct ExampleContract {
    // Storage can be defined here if used
}

#[contractimpl]
impl ExampleContract {
    // Simple public function
    pub fn get_message(env: Env) -> String {
        String::from_str(&env, "Message from contract")
    }
    
    // Function with parameters
    pub fn greet(env: Env, name: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), name]
    }
    
    // Function with storage
    pub fn save_value(env: Env, key: Symbol, value: i128) {
        env.storage().persistent().set(&key, &value);
    }
    
    pub fn get_value(env: Env, key: Symbol) -> Option<i128> {
        env.storage().persistent().get(&key)
    }
    
    // Function with authentication
    pub fn admin_only(env: Env, admin: Address) {
        // Verify invoker is admin
        admin.require_auth();
        // Admin logic
    }
}
```

### Storage in Soroban

#### Storage Types

1. **Persistent Storage**: Persists between transactions
2. **Temporary Storage**: Only lasts the current transaction
3. **Instance Storage**: Contract instance-level storage

#### Storage Examples

```rust
// Persistent Storage
env.storage().persistent().set(&key, &value);
let value: Option<Type> = env.storage().persistent().get(&key);
env.storage().persistent().remove(&key);

// Temporary Storage
env.storage().temporary().set(&key, &value);
let value: Option<Type> = env.storage().temporary().get(&key);

// Instance Storage
env.storage().instance().set(&key, &value);
let value: Option<Type> = env.storage().instance().get(&key);
```

**Considerations:**
- Persistent storage has higher gas cost
- Temporary storage is cheaper but lost
- Instance storage is for contract configuration data

### Events and Logging

```rust
use soroban_sdk::{contract, contractimpl, Env, symbol_short, log};

#[contract]
pub struct ContractWithEvents;

#[contractimpl]
impl ContractWithEvents {
    pub fn emit_event(env: Env, message: Symbol) {
        log!(&env, "Event emitted: {}", message);
        
        // Structured events
        env.events().publish(
            (symbol_short!("transfer"), symbol_short!("from")),
            address_from,
        );
    }
}
```

### Error Handling

```rust
use soroban_sdk::{contracterror, contract, contractimpl, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    DivisionByZero = 1,
    InvalidValue = 2,
}

#[contract]
pub struct ContractWithErrors;

#[contractimpl]
impl ContractWithErrors {
    pub fn divide(env: Env, a: i128, b: i128) -> Result<i128, Error> {
        if b == 0 {
            return Err(Error::DivisionByZero);
        }
        Ok(a / b)
    }
}
```

---

## Compilation and Testing

### Compiling a Contract

#### Individual Compilation

```bash
cd /home/carlos/dispensario_digital_sc/contracts
cargo build --target wasm32v1-none --release --package contract-name
```

**Result:**
- WASM binary at `target/wasm32v1-none/release/contract-name.wasm`

#### Compile All Contracts

```bash
cd /home/carlos/dispensario_digital_sc/contracts
cargo build --target wasm32v1-none --release
```

### Running Tests

#### Tests for Specific Contract

```bash
cd /home/carlos/dispensario_digital_sc/contracts
cargo test --package contract-name
```

#### All Tests

```bash
cargo test
```

#### Tests with Detailed Output

```bash
cargo test --package contract-name -- --nocapture
```

### Test Structure

```rust
use soroban_sdk::{symbol_short, vec, Env, Address};

use crate::{MyContract, MyContractClient};

#[test]
fn test_basic_function() {
    // Create test environment
    let env = Env::default();
    
    // Register contract
    let contract_id = env.register(MyContract, ());
    
    // Create client to interact
    let client = MyContractClient::new(&env, &contract_id);
    
    // Execute function and verify result
    let result = client.function_to_test();
    assert_eq!(result, expected_value);
}

#[test]
fn test_with_storage() {
    let env = Env::default();
    let contract_id = env.register(MyContract, ());
    let client = MyContractClient::new(&env, &contract_id);
    
    // Save value
    client.save_value(&symbol_short!("key"), &100);
    
    // Retrieve and verify
    let value = client.get_value(&symbol_short!("key"));
    assert_eq!(value, Some(100));
}

#[test]
#[should_panic(expected = "Expected error")]
fn test_error_handling() {
    let env = Env::default();
    let contract_id = env.register(MyContract, ());
    let client = MyContractClient::new(&env, &contract_id);
    
    // This should fail
    client.function_that_fails();
}
```

### Verifying WASM Binary

```bash
# Check binary size
ls -lh target/wasm32v1-none/release/*.wasm

# View WASM information
wasm-objdump -h target/wasm32v1-none/release/contract.wasm
```

---

## Deployment and Testing on Local Network

### Local Network Setup

To test contracts on a local Stellar network, you need to run Stellar Quickstart, which is a Docker container that provides a complete and functional Stellar network on your local machine.

#### Prerequisites

- **Docker**: Must be installed and running
- **Stellar CLI**: Stellar command-line tool

#### Starting the Local Network

**Option 1: Using Stellar Quickstart (Recommended)**

```bash
# Start Stellar Quickstart container
docker run --rm -it \
  -p 8000:8000 \
  -p 8001:8001 \
  --name stellar \
  stellar/quickstart:testing \
  --standalone \
  --enable-soroban-rpc

# Or in detached mode (background)
docker run -d \
  -p 8000:8000 \
  -p 8001:8001 \
  --name stellar \
  stellar/quickstart:testing \
  --standalone \
  --enable-soroban-rpc
```

**Ports:**
- **8000**: Horizon API (HTTP)
- **8001**: Soroban RPC (HTTP)

**Verify it's running:**

```bash
# Check container
docker ps | grep stellar

# Verify it responds
curl http://localhost:8000
curl http://localhost:8001
```

**Option 2: Using Stellar CLI**

If you have `stellar-cli` configured, you can use:

```bash
# Configure local network
stellar network use local

# This will automatically start a Docker container if not running
```

#### Stopping the Local Network

```bash
# Stop container
docker stop stellar

# Remove container (optional)
docker rm stellar
```

### Configure Stellar CLI for Local Network

#### Configure Network

```bash
# Use local network
stellar network use local

# Verify current configuration
stellar network show
```

#### Create Identities (Accounts)

```bash
# Generate a new identity
stellar keys generate alice

# View public key
stellar keys address alice

# View secret key (handle with care!)
stellar keys secret alice
```

#### Fund Accounts on Local Network

On local network, you can fund accounts using friendbot:

```bash
# Fund an account using friendbot
curl "http://localhost:8000/friendbot?addr=GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"

# Or using CLI (if configured)
stellar keys fund alice
```

**Note:** On local network, friendbot is available at `http://localhost:8000/friendbot`. You only need to pass the public address of the account as the `addr` parameter.

### Deploy a Contract on Local Network

#### Step 1: Compile the Contract

```bash
cd /home/carlos/dispensario_digital_sc/contracts
cargo build --target wasm32v1-none --release --package hello-world
```

#### Step 2: Deploy the Contract

```bash
# Make sure you're using local network
stellar network use local

# Use an identity with funds
stellar keys use alice

# Deploy the contract
stellar contract deploy \
  --wasm target/wasm32v1-none/release/hello-world.wasm \
  --alias hello-world
```

**Expected output:**
```
Contract deployed with ID: CBB65ZLBQBZL5IYHDHEEPCVUUMFOQUZSQKAJFV36R7TZETCLWGFTRLOQ
```

#### Step 3: Verify Deployment

```bash
# View contract information
stellar contract read --id hello-world

# Or using the ID directly
stellar contract read --id CBB65ZLBQBZL5IYHDHEEPCVUUMFOQUZSQKAJFV36R7TZETCLWGFTRLOQ
```

### Invoke Contract Functions

#### Invoke a Simple Function

```bash
# Invoke hello function with a parameter
stellar contract invoke \
  --id hello-world \
  -- hello \
  --to World
```

**Example with multiple parameters:**

```bash
stellar contract invoke \
  --id hello-world \
  -- function_with_params \
  --param1 value1 \
  --param2 value2
```

#### Invoke with Authentication

If the function requires authentication:

```bash
# Use a specific identity
stellar keys use alice

# Invoke the function
stellar contract invoke \
  --id hello-world \
  -- authenticated_function \
  --admin alice
```

### Read Contract Storage

#### Read Persistent Storage

```bash
# Read a value from persistent storage
stellar contract read \
  --id hello-world \
  --durability persistent \
  --key COUNTER
```

#### Read Temporary Storage

```bash
# Read a value from temporary storage
stellar contract read \
  --id hello-world \
  --durability temporary \
  --key TEMP_KEY
```

#### Read Instance Storage

```bash
# Read a value from instance storage
stellar contract read \
  --id hello-world \
  --durability instance \
  --key ADMIN
```

### Complete Local Network Testing Workflow

#### Example: Complete Contract

```bash
# 1. Start local network (in another terminal)
docker run -d -p 8000:8000 -p 8001:8001 --name stellar \
  stellar/quickstart:testing --standalone --enable-soroban-rpc

# 2. Configure local network
stellar network use local

# 3. Create and fund account
stellar keys generate alice
curl "http://localhost:8000/friendbot?addr=$(stellar keys address alice)"
stellar keys use alice

# 4. Compile contract
cd /home/carlos/dispensario_digital_sc/contracts
cargo build --target wasm32v1-none --release --package hello-world

# 5. Deploy contract
stellar contract deploy \
  --wasm target/wasm32v1-none/release/hello-world.wasm \
  --alias hello-world

# 6. Invoke function
stellar contract invoke --id hello-world -- hello --to Developer

# 7. Read storage (if applicable)
stellar contract read --id hello-world --durability persistent --key COUNTER

# 8. Cleanup (optional)
docker stop stellar
docker rm stellar
```

### Verify Network Status

#### View Ledger Information

```bash
# View current ledger information
curl http://localhost:8000/ledgers?order=desc&limit=1

# View account information
curl http://localhost:8000/accounts/GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

#### View Transactions

```bash
# View recent transactions
curl http://localhost:8000/transactions?order=desc&limit=10
```

### Troubleshooting Local Network

#### Error: "Network connection failed"

**Cause:** Docker container is not running.

**Solution:**
```bash
# Verify container is running
docker ps | grep stellar

# If not, start it
docker start stellar

# Or create a new one
docker run -d -p 8000:8000 -p 8001:8001 --name stellar \
  stellar/quickstart:testing --standalone --enable-soroban-rpc
```

#### Error: "Account not found"

**Cause:** Account has no funds or doesn't exist.

**Solution:**
```bash
# Fund account using friendbot
curl "http://localhost:8000/friendbot?addr=$(stellar keys address alice)"

# Verify balance
stellar keys balance alice
```

#### Error: "Insufficient balance"

**Cause:** Account doesn't have enough XLM to pay fees.

**Solution:**
```bash
# Fund account more
curl "http://localhost:8000/friendbot?addr=$(stellar keys address alice)"
```

#### Error: "Contract not found"

**Cause:** Contract is not deployed or ID is incorrect.

**Solution:**
```bash
# Verify contract is deployed
stellar contract read --id hello-world

# If it doesn't exist, deploy again
stellar contract deploy --wasm path/to/contract.wasm --alias hello-world
```

### Advantages of Using Local Network

1. **Free**: No transaction costs
2. **Fast**: Instant transactions
3. **Full Control**: You can reset state whenever you want
4. **Private**: Everything stays on your machine
5. **Ideal for Development**: Risk-free testing

### Comparison: Local Network vs Testnet

| Feature | Local Network | Testnet |
|---------|---------------|---------|
| Cost | Free | Free (but requires funds) |
| Speed | Instant | ~5 seconds |
| Persistence | Lost when stopped | Persistent |
| Friendbot | Manual (curl) | Automatic (`stellar keys fund`) |
| Use Case | Development/Testing | More realistic testing |

---

## Code Examples

### Example 1: Simple Contract with Storage

```rust
#![no_std]
use soroban_sdk::{
    contract, contractimpl,
    symbol_short,
    Env, Symbol, i128,
};

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> i128 {
        let key = symbol_short!("counter");
        let mut value: i128 = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(0);
        value += 1;
        env.storage().persistent().set(&key, &value);
        value
    }
    
    pub fn get(env: Env) -> i128 {
        let key = symbol_short!("counter");
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(0)
    }
    
    pub fn reset(env: Env) {
        let key = symbol_short!("counter");
        env.storage().persistent().remove(&key);
    }
}
```

### Example 2: Contract with Authentication

```rust
#![no_std]
use soroban_sdk::{
    contract, contractimpl,
    symbol_short,
    Env, Address, Symbol,
};

#[contract]
pub struct SecureContract {
    // Admin stored in instance storage
}

#[contractimpl]
impl SecureContract {
    pub fn initialize(env: Env, admin: Address) {
        let key = symbol_short!("admin");
        env.storage().instance().set(&key, &admin);
    }
    
    pub fn admin_only(env: Env) {
        let key = symbol_short!("admin");
        let admin: Address = env.storage().instance().get(&key).unwrap();
        admin.require_auth();
        // Only admin can execute this
    }
    
    pub fn public_function(env: Env) -> Symbol {
        symbol_short!("public")
    }
}
```

### Example 3: Contract with Events

```rust
#![no_std]
use soroban_sdk::{
    contract, contractimpl,
    symbol_short,
    Env, Address, Symbol,
    log,
};

#[contract]
pub struct ContractWithEvents;

#[contractimpl]
impl ContractWithEvents {
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        
        // Transfer logic...
        
        // Emit event
        env.events().publish(
            (symbol_short!("transfer"), symbol_short!("from")),
            from.clone(),
        );
        env.events().publish(
            (symbol_short!("transfer"), symbol_short!("to")),
            to,
        );
        env.events().publish(
            (symbol_short!("transfer"), symbol_short!("amount")),
            amount,
        );
        
        // Simple log
        log!(&env, "Transfer completed: {} units", amount);
    }
}
```

### Example 4: Contract with Error Handling

```rust
#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror,
    Env, Address, i128,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    InsufficientBalance = 1,
    InvalidAmount = 2,
    Unauthorized = 3,
}

#[contract]
pub struct Bank;

#[contractimpl]
impl Bank {
    pub fn withdraw(env: Env, user: Address, amount: i128) -> Result<i128, Error> {
        user.require_auth();
        
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        
        let balance_key = symbol_short!("balance");
        let current_balance: i128 = env
            .storage()
            .persistent()
            .get(&balance_key)
            .unwrap_or(0);
        
        if current_balance < amount {
            return Err(Error::InsufficientBalance);
        }
        
        let new_balance = current_balance - amount;
        env.storage().persistent().set(&balance_key, &new_balance);
        
        Ok(new_balance)
    }
}
```

---

## Best Practices

### 1. Security

#### Input Validation
```rust
pub fn secure_function(env: Env, value: i128) {
    // ALWAYS validate inputs
    if value < 0 {
        panic!("Negative value not allowed");
    }
    // ...
}
```

#### Authentication
```rust
pub fn private_function(env: Env, admin: Address) {
    // Verify authentication
    admin.require_auth();
    // ...
}
```

#### Reentrancy Prevention
- Use checks-effects-interactions pattern
- Consider locks when necessary

### 2. Gas Optimization

#### Efficient Storage Usage
- Prefer temporary storage when possible
- Group related data
- Use compact types (Symbol vs String)

#### Minimize Calculations
- Cache calculated values
- Avoid unnecessary loops
- Use native operations when possible

### 3. Clean Code

#### Descriptive Names
```rust
// Bad
pub fn fn1(env: Env, a: i128, b: i128) -> i128

// Good
pub fn calculate_total_balance(env: Env, account: Address, period: i128) -> i128
```

#### Useful Comments
```rust
/// Calculates the total balance of an account for a specific period.
/// 
/// # Arguments
/// * `account` - Account address
/// * `period` - Time period in days
/// 
/// # Returns
/// Total balance as i128
pub fn calculate_total_balance(env: Env, account: Address, period: i128) -> i128 {
    // ...
}
```

#### Modularization
- Separate complex logic into private functions
- Use modules to organize large code

### 4. Testing

#### Complete Coverage
- Tests for happy paths
- Tests for error cases
- Edge case tests
- Integration tests

#### Readable Tests
```rust
#[test]
fn test_successful_withdrawal_when_sufficient_balance() {
    // Arrange
    let env = Env::default();
    let contract_id = env.register(Bank, ());
    let client = BankClient::new(&env, &contract_id);
    
    // Act
    let result = client.withdraw(&user, &100);
    
    // Assert
    assert_eq!(result, Ok(900));
}
```

### 5. Versioning

- Use semantic versions in Cargo.toml
- Document breaking changes
- Maintain changelog

---

## Troubleshooting

### Error: "target wasm32v1-none not found"

**Solution:**
```bash
rustup target add wasm32v1-none
```

### Error: "failed to load manifest for workspace member"

**Cause:** Workspace is trying to include directories that aren't contracts.

**Solution:** Verify that `exclude = ["target"]` is in `Cargo.toml` and only valid contracts are listed in `members`.

### Error: "use of undeclared crate or module `std`"

**Cause:** Missing `#![no_std]` at the beginning of the file.

**Solution:** Add `#![no_std]` as the first line of the file.

### Compilation Error: "cannot find type X in this scope"

**Cause:** Missing import of the type from SDK.

**Solution:** Verify imports:
```rust
use soroban_sdk::{Env, Address, Symbol, /* other needed types */};
```

### WASM Binary Too Large

**Solutions:**
1. Verify compilation in `--release` mode
2. Check that `release` profile has `opt-level = "z"`
3. Remove unused code
4. Use more compact types (Symbol instead of String when possible)

### Tests Don't Compile

**Common Cause:** Missing `testutils` feature in dev-dependencies.

**Solution:**
```toml
[dev-dependencies]
soroban-sdk = { workspace = true, features = ["testutils"] }
```

### Error: "method `register_contract` is deprecated"

**Solution:** Use `env.register(Contract, ())` instead of `env.register_contract(None, Contract)`.

---

## References and Resources

### Official Documentation

- **Soroban Documentation**: https://soroban.stellar.org/docs
- **Soroban SDK Reference**: https://docs.rs/soroban-sdk/
- **Stellar Documentation**: https://developers.stellar.org/

### Repositories

- **Soroban SDK**: https://github.com/stellar/rs-soroban-sdk
- **OpenZeppelin Stellar Contracts**: https://github.com/OpenZeppelin/stellar-contracts
- **Stellar Examples**: https://github.com/stellar/soroban-examples

### Tools

- **Stellar CLI**: https://github.com/stellar/stellar-cli
- **Soroban Tools**: https://github.com/stellar/soroban-tools

### Community

- **Stellar Discord**: https://discord.gg/stellar
- **Stellar Stack Exchange**: https://stellar.stackexchange.com/

### Tutorials

- **Soroban Book**: https://soroban.stellar.org/docs/category/getting-started
- **Smart Contract Development Guide**: https://soroban.stellar.org/docs/category/smart-contracts

### Specifications

- **WASM Specification**: https://webassembly.org/spec/
- **Stellar Protocol**: https://developers.stellar.org/docs/encyclopedia/protocol

---

## Conclusion

This workspace provides a solid foundation for smart contract development on Soroban. With the proper tools and configurations, you can develop, test, and deploy contracts efficiently and securely.

For questions or issues, consult the official documentation or the Stellar community.

---

**Last Updated:** November 2024  
**SDK Version:** 23.1.0  
**Rust Version:** 1.89.0

