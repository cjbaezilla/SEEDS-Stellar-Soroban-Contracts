# Technical Documentation: Cannabis Seed Traceability System

## Table of Contents

1. [Introduction](#introduction)
2. [System Architecture](#system-architecture)
3. [Seed Registry Contract](#seed-registry-contract)
4. [Seed NFT Contract](#seed-nft-contract)
5. [Contract Integration](#contract-integration)
6. [Libraries and Dependencies](#libraries-and-dependencies)
7. [Role and Permission System](#role-and-permission-system)
8. [Seed Lifecycle](#seed-lifecycle)
9. [Testing and Validation](#testing-and-validation)
10. [Workflows](#workflows)
11. [Events and Auditing](#events-and-auditing)
12. [Security Considerations](#security-considerations)
13. [Usage Examples](#usage-examples)

---

## Introduction

This smart contract system is designed to provide complete traceability of cannabis seeds from their creation to the end consumer. The system is built on Stellar's Soroban platform and uses OpenZeppelin libraries to ensure security and standards compliance.

### System Objectives

- **Complete Traceability**: Track each seed from initial registration to final consumption
- **Transparency**: Provide verifiable information about origin, processing, and distribution
- **Regulatory Compliance**: Facilitate compliance with regulations for dispensaries and cannabis clubs
- **Access Control**: Implement a robust role and permission system
- **Immutability**: Ensure historical data cannot be modified

### Use Cases

- Legal dispensaries that need to demonstrate product provenance
- Cannabis clubs requiring traceability for regulatory compliance
- Regulators needing to audit the supply chain
- Consumers wanting to verify product quality and origin

---

## System Architecture

The system consists of two main smart contracts that work together:

```
┌─────────────────────────────────────────────────────────────┐
│              Traceability System                            │
└─────────────────────────────────────────────────────────────┘
                              │
                ┌─────────────┴─────────────┐
                │                           │
    ┌───────────▼──────────┐    ┌──────────▼──────────┐
    │  Seed Registry       │    │   Seed NFT          │
    │  (Registry)          │    │   (Tokenization)   │
    │                      │    │                     │
    │  - Stores data       │    │  - Represents each  │
    │  - Creates NFTs      │    │    seed as NFT      │
    │  - Queries           │    │  - Manages states   │
    │  - Batch operations  │    │  - History          │
    └──────────────────────┘    └─────────────────────┘
```

### Data Flow

1. **Initial Registration**: The Registry contract stores seed information and automatically creates an NFT
2. **Tokenization**: Each seed is represented as a unique NFT in the Seed NFT contract
3. **State Updates**: Throughout the lifecycle, the NFT is updated with new states and metadata
4. **Transfers**: NFTs can only be transferred between authorized addresses (whitelist)
5. **Querying**: Both contracts allow querying historical and current information

---

## Seed Registry Contract

### Overview

The `seed-registry` contract is the central entry point for all seeds. It acts as a master registry that stores complete information about each seed and coordinates the creation of corresponding NFTs.

### Contract Structure

```
seed-registry/
├── src/
│   ├── lib.rs          # Main contract and public functions
│   ├── registry.rs     # Internal registry logic
│   ├── seed_data.rs    # SeedData structure
│   ├── roles.rs        # Role definitions
│   └── test.rs         # Unit tests
```

### Data Structure: SeedData

Each registered seed contains the following information:

```rust
pub struct SeedData {
    pub id: String,                    // Unique seed ID
    pub created_at: u64,               // Creation timestamp
    pub creator: Address,               // Address that created the registration
    pub variety: String,                // Variety/genetics (e.g., "Indica", "Sativa")
    pub batch: String,                  // Batch number
    pub origin_country: String,         // Country of origin
    pub seed_bank: String,              // Seed bank
    pub expected_thc: Option<u32>,      // Expected THC percentage
    pub expected_cbd: Option<u32>,      // Expected CBD percentage
    pub organic_certified: bool,        // Organic certification
    pub nft_id: u128,                   // Associated NFT ID
    pub nft_contract: Address,          // NFT contract address
}
```

### Main Functions

#### `initialize(env: Env, admin: Address, nft_contract: Address)`

Initializes the registry contract. Can only be executed once.

**Parameters:**
- `admin`: Address that will be assigned as administrator
- `nft_contract`: Address of the NFT contract to be used for creating tokens

**Effects:**
- Sets seed counter to 0
- Stores reference to NFT contract
- Assigns ADMIN role to administrator
- Initializes pause state as `false`

**Example:**
```rust
let admin = Address::generate(&env);
let nft_contract = Address::generate(&env);
client.initialize(&admin, &nft_contract);
```

#### `register_seed(...) -> u128`

Registers a new seed in the system and automatically creates its corresponding NFT.

**Parameters:**
- `seed_id`: Unique seed identifier
- `variety`: Variety/genetics
- `batch`: Batch number
- `origin_country`: Country of origin
- `seed_bank`: Seed bank
- `expected_thc`: Expected THC percentage (optional)
- `expected_cbd`: Expected CBD percentage (optional)
- `organic_certified`: Whether it has organic certification

**Requirements:**
- Contract must not be paused
- Invoker must have CULTIVATOR role
- Seed must not exist previously

**Returns:**
- `u128`: ID of the created NFT

**Events Emitted:**
- `SeedReg` with fields: `seed_id`, `nft_id`, `creator`, `created`

**Example:**
```rust
let nft_id = client.register_seed(
    &seed_id,
    &String::from_str(&env, "Indica"),
    &String::from_str(&env, "BATCH-2024-001"),
    &String::from_str(&env, "Colombia"),
    &String::from_str(&env, "Bank-001"),
    Some(20u32),  // 20% expected THC
    Some(2u32),   // 2% expected CBD
    true          // Organic certified
);
```

#### `register_seeds_batch(...) -> Vec<u128>`

Registers multiple seeds in a single transaction. Useful for registering complete batches.

**Parameters:**
- Vectors of equal length with information for each seed

**Limits:**
- Maximum 100 seeds per batch

**Advantages:**
- Reduces gas costs
- Allows atomic registration of complete batches
- If a seed already exists, it is skipped without failing the entire operation

**Example:**
```rust
let seed_ids = vec![&env, id1, id2, id3];
let varieties = vec![&env, var1, var2, var3];
// ... other vectors
let nft_ids = client.register_seeds_batch(
    &seed_ids, &varieties, &batches, 
    &origin_countries, &seed_banks,
    &expected_thcs, &expected_cbds, 
    &organic_flags
);
```

#### Query Functions

##### `get_seed(env: Env, seed_id: String) -> Option<SeedData>`

Gets all information for a specific seed.

##### `get_seed_count(env: Env) -> u64`

Returns the total number of registered seeds.

##### `get_all_seed_ids(env: Env) -> Vec<String>`

Returns a list of all registered seed IDs.

##### `query_seeds_by_variety(env: Env, variety: String) -> Vec<String>`

Searches for all seeds of a specific variety.

**Example:**
```rust
let indica_seeds = client.query_seeds_by_variety(
    &String::from_str(&env, "Indica")
);
```

##### `query_seeds_by_batch(env: Env, batch: String) -> Vec<String>`

Searches for all seeds in a specific batch.

##### `query_seeds_by_creator(env: Env, creator: Address) -> Vec<String>`

Searches for all seeds created by a specific address.

### Storage

The contract uses two types of storage:

#### Instance Storage
- `SEED_CNT`: Total seed counter (u64)
- `NFT_CNTRCT`: NFT contract address (Address)
- `PAUSED`: Pause state (bool)

#### Persistent Storage
- `(SEED_DATA, seed_id)`: Complete data for each seed (SeedData)
- `(SEED_IDS)`: List of all seed IDs (Vec<String>)
- `(ROLE_KEY, address)`: Role mapping by address (bool)

---

## Seed NFT Contract

### Overview

The `seed-nft` contract implements a non-fungible token (NFT) system based on OpenZeppelin Stellar's ERC-721-like standard. Each NFT represents a unique seed and stores its current state, complete metadata, and transition history.

### Contract Structure

```
seed-nft/
├── src/
│   ├── lib.rs          # Main contract
│   ├── nft.rs          # NFT helper functions
│   ├── lifecycle.rs    # Lifecycle states
│   ├── metadata.rs     # Metadata structure
│   ├── history.rs      # History management
│   ├── roles.rs        # Role definitions
│   └── test.rs         # Unit tests
```

### Lifecycle States

The system defines 8 main states representing the complete cycle of a seed:

```rust
pub enum LifecycleState {
    Seed = 0,              // Initial state: seed registered
    Germinated = 1,        // Seed germinated
    PlantVegetative = 2,   // Plant in vegetative phase
    PlantFlowering = 3,    // Plant in flowering phase
    PlantHarvested = 4,    // Plant harvested
    Processed = 5,         // Product processed
    Distributed = 6,       // In dispensary/distribution
    Consumed = 7,          // Consumed by end user
}
```

### Transition Rules

State transitions are strictly validated:

1. **Sequentiality**: Only transitions to the next state are allowed
2. **No Rollback**: Cannot return to a previous state
3. **No Skipping**: Cannot skip intermediate states
4. **State Permissions**: Each transition requires a specific role

**Valid Transition Matrix:**
```
Seed → Germinated → PlantVegetative → PlantFlowering 
→ PlantHarvested → Processed → Distributed → Consumed
```

### Metadata Structure

Each NFT stores complete metadata, including fields compatible with the OpenSea standard:

```rust
pub struct Attribute {
    pub trait_type: String,              // Attribute type (e.g., "Variety", "THC")
    pub value: String,                   // Attribute value (e.g., "Indica", "20%")
}

pub struct OpenSeaMetadata {
    pub name: Option<String>,            // NFT name
    pub description: Option<String>,      // NFT description
    pub image: Option<String>,            // Image URL
    pub external_url: Option<String>,    // External URL
    pub attributes: Option<Vec<Attribute>>, // List of attributes
}

pub struct SeedMetadata {
    pub state: LifecycleState,           // Current state
    pub location: Option<String>,         // GPS location or description
    pub temperature: Option<i32>,       // Temperature in Celsius
    pub humidity: Option<u32>,           // Relative humidity (%)
    pub lab_analysis: Option<String>,    // Laboratory analysis (hash or reference)
    pub processor: Option<Address>,      // Processor address
    pub distributor: Option<Address>,    // Distributor address
    pub consumer: Option<Address>,        // End consumer address
    pub updated_at: u64,                 // Last update timestamp
    // OpenSea compatible fields
    pub name: String,                    // NFT name (required by OpenSea)
    pub description: String,             // NFT description (required by OpenSea)
    pub image: String,                   // Image URL (required by OpenSea)
    pub external_url: Option<String>,    // External URL (optional)
    pub attributes: Vec<Attribute>,       // List of attributes/traits (optional)
}
```

### Main Functions

#### `initialize(env: Env, admin: Address, name: String, symbol: String)`

Initializes the NFT contract. Configures the base token and assigns administrator role.

**Parameters:**
- `admin`: Administrator address
- `name`: Token name (e.g., "Cannabis Seed NFT")
- `symbol`: Token symbol (e.g., "CSNFT")

#### `mint(env: Env, to: Address, token_id: u128, name: String, description: String, image: String, external_url: Option<String>, attributes: Vec<Attribute>)`

Creates a new NFT. This function is automatically called by the Registry contract.

**Parameters:**
- `to`: Address that will receive the NFT
- `token_id`: Unique token ID
- `name`: NFT name (required by OpenSea)
- `description`: NFT description (required by OpenSea)
- `image`: NFT image URL (required by OpenSea)
- `external_url`: Optional external URL (can be `None`)
- `attributes`: List of NFT attributes/traits (can be an empty vector)

**Effects:**
- Creates NFT with initial `Seed` state
- Initializes basic metadata including OpenSea fields
- Emits `Mint` event

**Example:**
```rust
let name = String::from_str(&env, "Cannabis Indica Seed #001");
let description = String::from_str(&env, "Certified Indica variety seed");
let image = String::from_str(&env, "https://example.com/images/seed-001.png");
let external_url = Some(String::from_str(&env, "https://example.com/seeds/001"));
let mut attributes = Vec::new(&env);
attributes.push_back(&Attribute {
    trait_type: String::from_str(&env, "Variety"),
    value: String::from_str(&env, "Indica"),
});
attributes.push_back(&Attribute {
    trait_type: String::from_str(&env, "Expected THC"),
    value: String::from_str(&env, "20%"),
});

client.mint(&owner, &token_id, &name, &description, &image, &external_url, &attributes);
```

#### `update_state(...)`

Updates the lifecycle state of the NFT. This is one of the most important functions in the system.

**Parameters:**
- `caller`: Address performing the update (must authenticate)
- `token_id`: ID of NFT to update
- `new_state`: New state (u32)
- `location`: Optional location
- `temperature`: Optional temperature
- `humidity`: Optional humidity
- `notes`: Optional additional notes

**Validations:**
1. Contract must not be paused
2. Caller must authenticate
3. Caller must have appropriate role for new state:
   - `Germinated`, `PlantVegetative`, `PlantFlowering`, `PlantHarvested`: Requires `CULTIVATOR`
   - `Processed`: Requires `PROCESSOR`
   - `Distributed`: Requires `DISPENSARY`
   - `Consumed`: Requires `DISPENSARY`
4. State transition must be valid according to rules

**Effects:**
- Updates state in metadata
- Saves transition in history
- Updates specific fields based on state (processor, distributor, consumer)
- Emits `StateTrans` event

**Example:**
```rust
client.update_state(
    &cultivator,
    &token_id,
    &(LifecycleState::Germinated as u32),
    &Some(String::from_str(&env, "40.7128,-74.0060")), // GPS
    &Some(25i32),  // 25°C
    &Some(60u32),  // 60% humidity
    &Some(String::from_str(&env, "Successful germination"))
);
```

#### `update_metadata(...)`

Updates specific metadata fields without changing state. Allows updating both traceability fields and OpenSea fields.

**Parameters:**
- `caller`: Authenticated address with CULTIVATOR role
- `token_id`: NFT ID
- `location`: New location (optional)
- `temperature`: New temperature (optional)
- `humidity`: New humidity (optional)
- `lab_analysis`: New laboratory analysis (optional)
- `opensea_metadata`: Optional structure with OpenSea fields to update (`Option<OpenSeaMetadata>`)

**OpenSeaMetadata Structure:**
```rust
pub struct OpenSeaMetadata {
    pub name: Option<String>,            // New name (optional)
    pub description: Option<String>,      // New description (optional)
    pub image: Option<String>,            // New image URL (optional)
    pub external_url: Option<String>,    // New external URL (optional)
    pub attributes: Option<Vec<Attribute>>, // New attributes (optional)
}
```

**Note:** Only fields provided in `opensea_metadata` are updated. If a field is `None`, it is not modified.

**Example:**
```rust
// Update only traceability fields
client.update_metadata(
    &cultivator,
    &token_id,
    &Some(String::from_str(&env, "New location")),
    &Some(22i32),
    &Some(65u32),
    &Some(String::from_str(&env, "THC: 22%, CBD: 3%")),
    &None, // Don't update OpenSea fields
);

// Update OpenSea fields
let mut opensea = OpenSeaMetadata {
    name: Some(String::from_str(&env, "Updated Seed")),
    description: Some(String::from_str(&env, "Updated description")),
    image: Some(String::from_str(&env, "https://example.com/new-image.png")),
    external_url: None, // Don't change
    attributes: None,    // Don't change
};
client.update_metadata(
    &cultivator,
    &token_id,
    &None, &None, &None, &None, // Don't change traceability fields
    &Some(opensea),
);
```

#### `get_metadata(env: Env, token_id: u128) -> Option<SeedMetadata>`

Gets complete metadata for an NFT.

#### `get_history(env: Env, token_id: u128) -> Vec<StateTransition>`

Gets complete history of state transitions.

**StateTransition Structure:**
```rust
pub struct StateTransition {
    pub from_state: LifecycleState,
    pub to_state: LifecycleState,
    pub timestamp: u64,
    pub updated_by: Address,
    pub notes: Option<String>,
}
```

### Standard NFT Functions

The contract implements all standard NFT functions:

- `name()`: Token name
- `symbol()`: Token symbol
- `balance_of(owner)`: Balance of NFTs for an address
- `owner_of(token_id)`: Owner of a specific NFT
- `approve(caller, operator, token_id)`: Approve transfer
- `get_approved(token_id)`: Get current approval
- `set_approval_for_all(caller, operator, approved)`: Approve all tokens
- `is_approved_for_all(owner, operator)`: Check global approval
- `transfer_from(caller, from, to, token_id)`: Transfer NFT

**Important Note**: `transfer_from` validates that the recipient is on the whitelist before allowing the transfer.

### OpenSea Compatibility

The Seed NFT contract is compatible with the OpenSea metadata standard, allowing tokens to be properly displayed on marketplaces and explorers that support this standard.

#### Required Fields

The following fields are required by the OpenSea standard and must be provided when creating an NFT:

- **`name`**: Descriptive name of the NFT
- **`description`**: Detailed description of the NFT
- **`image`**: URL of the image representing the NFT (must be publicly accessible)

#### Optional Fields

- **`external_url`**: External URL pointing to a web page related to the NFT
- **`attributes`**: List of attributes or traits describing NFT characteristics

#### Attribute Structure

Attributes follow the OpenSea standard format:

```rust
pub struct Attribute {
    pub trait_type: String,  // Attribute type (e.g., "Variety", "THC", "State")
    pub value: String,       // Attribute value (e.g., "Indica", "20%", "Germinated")
}
```

#### OpenSea Metadata Example

When querying NFT metadata, OpenSea fields are available alongside traceability fields:

```rust
let metadata = client.get_metadata(&token_id).unwrap();
// metadata.name - NFT name
// metadata.description - Description
// metadata.image - Image URL
// metadata.external_url - External URL (optional)
// metadata.attributes - List of attributes
```

#### Benefits

1. **Marketplace Display**: NFTs can be properly displayed on platforms supporting the OpenSea standard
2. **Filterable Attributes**: Attributes allow filtering and searching NFTs by specific characteristics
3. **Universal Compatibility**: The standard is widely adopted, facilitating integration with external tools
4. **Rich Metadata**: Allows including visual and descriptive information in addition to traceability data

### Whitelist System

To ensure only authorized addresses can receive NFTs, the system implements a whitelist:

#### `add_to_whitelist(env: Env, caller: Address, account: Address)`

Adds an address to the whitelist. Only ADMIN can execute this function.

#### `remove_from_whitelist(env: Env, caller: Address, account: Address)`

Removes an address from the whitelist.

#### `is_whitelisted(env: Env, account: Address) -> bool`

Checks if an address is on the whitelist.

**Usage:**
```rust
// Only admin can add addresses
client.add_to_whitelist(&admin, &authorized_address);

// Transfers only work to whitelisted addresses
client.transfer_from(&owner, &owner, &authorized_address, &token_id);
```

### Storage

#### Instance Storage
- `TOKEN`: NonFungibleToken instance (NonFungibleToken)
- `PAUSED`: Pause state (bool)

#### Persistent Storage
- `(METADATA, token_id)`: Metadata for each NFT (SeedMetadata)
- `(HISTORY, token_id)`: Transition history (Vec<StateTransition>)
- `(WHITELIST, address)`: Whitelist of addresses (bool)
- `(ROLE_KEY, address)`: Role mapping (bool)

---

## Contract Integration

### Integration Flow

```
1. Initialization
   ├── Seed Registry initialized with reference to NFT contract
   └── Seed NFT initialized with name and symbol

2. Seed Registration
   ├── User calls register_seed() in Seed Registry
   ├── Registry validates permissions and data
   ├── Registry generates unique NFT ID
   ├── Registry calls mint() in Seed NFT contract
   ├── Seed NFT creates token with Seed state
   └── Registry stores reference to NFT

3. State Update
   ├── User calls update_state() in Seed NFT
   ├── NFT validates transition and permissions
   ├── NFT updates metadata and state
   └── NFT saves transition in history

4. Query
   ├── User can query data in Registry
   ├── User can query metadata in NFT
   └── User can query history in NFT
```

### Cross-References

- **Registry → NFT**: Registry stores `nft_id` and `nft_contract` for each seed
- **NFT → Registry**: NFT can validate it was created by Registry (via token_id)

### Synchronization

Contracts maintain consistency through:
1. **Unique IDs**: Registry generates sequential IDs that match NFT token_ids
2. **Events**: Both contracts emit events that can be monitored
3. **Immutability**: Once created, NFT cannot be deleted

---

## Libraries and Dependencies

### Soroban SDK (v23.1.0)

**Description**: Official Stellar SDK for Soroban smart contract development.

**Components Used:**
- `contract`, `contractimpl`: Macros for defining contracts
- `contracterror`: Custom error handling
- `Env`: Contract environment
- `Address`: Contract and account addresses
- `String`, `Vec`, `Symbol`: Data types
- `storage`: Storage system (persistent, instance, temporary)
- `events`: Event system

**Documentation**: [Soroban Documentation](https://soroban.stellar.org/docs)

### OpenZeppelin Stellar Contracts (v0.5.1)

#### stellar-access

**Description**: Access control library for Stellar contracts.

**Usage in Project:**
- Although not used directly, the system implements a similar role pattern
- Roles are stored in persistent storage with structure `(ROLE_KEY, address)`

**Defined Roles:**
- `ADMIN`: Full contract control
- `CULTIVATOR`: Can register seeds and update cultivation states
- `PROCESSOR`: Can update state to Processed
- `DISPENSARY`: Can update Distributed and Consumed states
- `CONSUMER`: Role for future functionalities

#### stellar-tokens

**Description**: Standard token implementations for Stellar, similar to ERC-20 and ERC-721.

**Component Used:**
- `NonFungibleToken`: Base NFT implementation

**Inherited Functionalities:**
- Minting and burning
- Transfers
- Approvals (approve, set_approval_for_all)
- Enumeration (balance_of, owner_of)

**Documentation**: [OpenZeppelin Stellar Contracts](https://docs.openzeppelin.com/stellar-contracts)

#### stellar-macros

**Description**: Procedural macros that reduce boilerplate.

**Usage:**
- Although included in dependencies, mainly used for future extensions
- Can simplify event and storage definitions

### Versions

```
soroban-sdk = "23.1.0"
stellar-access = "v0.5.1"
stellar-tokens = "v0.5.1"
stellar-macros = "v0.5.1"
```

---

## Role and Permission System

### Defined Roles

#### ADMIN
- **Permissions**:
  - Pause/resume contracts
  - Grant/revoke any role
  - Add/remove addresses from whitelist
  - Change associated NFT contract (Registry)
- **Assignment**: Automatic upon contract initialization

#### CULTIVATOR
- **Permissions**:
  - Register new seeds
  - Update states: Germinated, PlantVegetative, PlantFlowering, PlantHarvested
  - Update metadata (location, temperature, humidity, analysis)
- **Usage**: Cultivators, farmers, greenhouse operators

#### PROCESSOR
- **Permissions**:
  - Update state to Processed
  - Update processing-related metadata
- **Usage**: Processors, derivative product manufacturers

#### DISPENSARY
- **Permissions**:
  - Update states: Distributed, Consumed
  - Perform NFT transfers
- **Usage**: Dispensaries, distributors, point of sale

#### CONSUMER
- **Permissions**:
  - Currently limited, reserved for future functionalities
- **Usage**: End consumers

### Role Management

#### Grant Role
```rust
client.grant_role(&admin, &user_address, &ROLE_CULTIVATOR);
```

#### Revoke Role
```rust
client.revoke_role(&admin, &user_address, &ROLE_CULTIVATOR);
```

#### Check Role
```rust
let has_role = client.has_role(&user_address, &ROLE_CULTIVATOR);
```

### Role Storage

Roles are stored in persistent storage with key:
```
(ROLE_KEY, address) → bool
```

Where `ROLE_KEY` is the role symbol (e.g., `CULTIVAT`, `PROCESS`, etc.)

---

## Seed Lifecycle

### State Diagram

```
┌─────┐
│Seed │ (Initial state)
└──┬──┘
   │ [CULTIVATOR]
   ▼
┌──────────┐
│Germinated│
└──┬───────┘
   │ [CULTIVATOR]
   ▼
┌──────────────┐
│PlantVegetative│
└──┬───────────┘
   │ [CULTIVATOR]
   ▼
┌─────────────┐
│PlantFlowering│
└──┬──────────┘
   │ [CULTIVATOR]
   ▼
┌──────────────┐
│PlantHarvested│
└──┬───────────┘
   │ [PROCESSOR]
   ▼
┌──────────┐
│Processed│
└──┬───────┘
   │ [DISPENSARY]
   ▼
┌────────────┐
│Distributed│
└──┬────────┘
   │ [DISPENSARY]
   ▼
┌─────────┐
│Consumed │ (Final state)
└─────────┘
```

### State Descriptions

#### 1. Seed
- **Description**: Initial state when seed is registered
- **Initial Metadata**: Only basic registration information
- **Duration**: Until germination
- **Responsible**: System (automatic upon NFT creation)

#### 2. Germinated
- **Description**: Seed has germinated and begun growing
- **Typical Metadata**: Cultivation location, initial conditions
- **Duration**: Typically 1-2 weeks
- **Responsible**: CULTIVATOR

#### 3. PlantVegetative
- **Description**: Plant in vegetative growth phase
- **Typical Metadata**: Cultivation conditions, applied nutrients
- **Duration**: 3-16 weeks depending on variety
- **Responsible**: CULTIVATOR

#### 4. PlantFlowering
- **Description**: Plant in flowering phase
- **Typical Metadata**: Specific flowering conditions, estimated harvest date
- **Duration**: Typically 8-12 weeks
- **Responsible**: CULTIVATOR

#### 5. PlantHarvested
- **Description**: Plant has been harvested
- **Typical Metadata**: Harvest date, weight, storage conditions
- **Duration**: Until processing
- **Responsible**: CULTIVATOR

#### 6. Processed
- **Description**: Product has been processed (dried, cured, extracted, etc.)
- **Typical Metadata**: Processing type, laboratory analysis
- **Duration**: Until distribution
- **Responsible**: PROCESSOR

#### 7. Distributed
- **Description**: Product is in dispensary or point of sale
- **Typical Metadata**: Dispensary information, arrival date
- **Duration**: Until sale
- **Responsible**: DISPENSARY

#### 8. Consumed
- **Description**: Product has been consumed by end user
- **Typical Metadata**: Consumer information (if allowed)
- **Duration**: Final state
- **Responsible**: DISPENSARY

### Transition Validation

The system validates each state transition:

```rust
pub fn can_transition_to(self, to: LifecycleState) -> bool {
    match (self, to) {
        (Seed, Germinated) => true,
        (Germinated, PlantVegetative) => true,
        (PlantVegetative, PlantFlowering) => true,
        (PlantFlowering, PlantHarvested) => true,
        (PlantHarvested, Processed) => true,
        (Processed, Distributed) => true,
        (Distributed, Consumed) => true,
        _ => false,  // Any other transition is invalid
    }
}
```

---

## Testing and Validation

### Test Structure

Tests are organized in separate files for each contract:

- `seed-registry/src/test.rs`: Registry contract tests
- `seed-nft/src/test.rs`: NFT contract tests

### Registry Contract Tests

#### `test_initialize`
Verifies correct contract initialization:
- Seed counter at 0
- Admin has ADMIN role
- Contract is not paused

#### `test_register_seed`
Tests individual seed registration:
- Permission validation
- Data creation
- Counter increment

#### `test_roles`
Verifies role system:
- Grant roles
- Revoke roles
- Check roles

#### `test_pause`
Tests pause functionality:
- Pause contract
- Resume contract
- Verify state

#### `test_queries`
Validates query functions:
- Get all seeds
- Search by variety
- Search by batch
- Search by creator

### NFT Contract Tests

#### `test_initialize`
Verifies initialization:
- Correct name and symbol
- Admin has ADMIN role
- Correct initial state

#### `test_mint`
Tests NFT creation:
- Successful mint
- Correct balance
- Correct owner
- Initialized metadata

#### `test_state_transitions`
Validates state transitions:
- Valid transition
- Metadata update
- History saving
- Permission validation

#### `test_whitelist`
Tests whitelist system:
- Add to whitelist
- Remove from whitelist
- Verify state

#### `test_roles`
Similar to Registry role test

#### `test_pause`
Similar to Registry pause test

#### `test_metadata_update`
Validates metadata update:
- Individual field updates
- Preservation of other fields
- Permission validation

### Running Tests

```bash
# Registry tests
cd contracts
cargo test --package seed-registry

# NFT tests
cargo test --package seed-nft

# All tests
cargo test
```

### Test Coverage

Tests cover:
- ✅ Initialization
- ✅ Basic CRUD operations
- ✅ Role system
- ✅ Pause functionality
- ✅ Permission validations
- ✅ State transitions
- ✅ Metadata updates
- ✅ Whitelist system
- ⚠️ Contract integration (requires additional setup)
- ⚠️ Batch operations (basic structure)

---

## Workflows

### Workflow 1: Initial Seed Registration

```
1. Admin initializes Seed Registry
   └── Establishes reference to NFT contract

2. Admin grants CULTIVATOR role to cultivator
   └── Cultivator can now register seeds

3. Cultivator registers seed
   ├── Validates has CULTIVATOR role
   ├── Validates seed doesn't exist
   ├── Creates seed data
   ├── Calls mint() in NFT contract
   ├── NFT creates token with Seed state
   ├── Registry stores reference
   └── Emits events

4. Result
   ├── Seed registered in Registry
   ├── NFT created in Seed NFT
   └── Both contracts synchronized
```

### Workflow 2: Complete Lifecycle

```
1. Seed registered → State: Seed

2. Cultivator updates to Germinated
   ├── Validates CULTIVATOR role
   ├── Validates valid transition
   ├── Updates state
   ├── Saves to history
   └── Emits event

3. Cultivator updates to PlantVegetative
   └── Similar process

4. Cultivator updates to PlantFlowering
   └── Similar process

5. Cultivator updates to PlantHarvested
   └── Similar process

6. Processor updates to Processed
   ├── Validates PROCESSOR role
   ├── Updates processor in metadata
   └── Saves laboratory analysis

7. Dispensary updates to Distributed
   ├── Validates DISPENSARY role
   ├── Updates distributor in metadata
   └── Records arrival date

8. Dispensary updates to Consumed
   ├── Validates DISPENSARY role
   ├── Updates consumer in metadata
   └── Final state reached
```

### Workflow 3: NFT Transfer

```
1. Admin adds address to whitelist
   └── Only whitelisted addresses can receive NFTs

2. Owner approves transfer
   ├── approve() or set_approval_for_all()
   └── Authorizes another address

3. Transfer executed
   ├── Validates recipient is whitelisted
   ├── Validates approval
   ├── Transfers NFT
   └── Emits Transfer event

4. Result
   └── NFT now belongs to new address
```

### Workflow 4: Traceability Query

```
1. User has token_id or seed_id

2. Query Registry
   ├── get_seed(seed_id) → Initial information
   └── Gets nft_id and nft_contract

3. Query NFT
   ├── get_metadata(token_id) → Current state and metadata
   └── get_history(token_id) → Complete history

4. Result
   └── Complete traceability from registration to consumption
```

---

## Events and Auditing

### Registry Contract Events

#### `SeedReg` (SeedRegistered)
Emitted when a new seed is registered.

**Fields:**
- `seed_id`: Seed ID
- `nft_id`: Created NFT ID
- `creator`: Address that registered the seed
- `created`: Creation timestamp

**Usage Example:**
```rust
// Monitor all seed registrations
env.events().publish(
    (symbol_short!("SeedReg"), symbol_short!("seed_id")),
    seed_id
);
```

#### `RoleGrant` (RoleGranted)
Emitted when a role is granted.

**Fields:**
- `account`: Address that received the role
- `role`: Granted role

#### `RoleRevok` (RoleRevoked)
Emitted when a role is revoked.

**Fields:**
- `account`: Address that lost the role
- `role`: Revoked role

#### `Paused` / `Unpaused`
Emitted when contract is paused or resumed.

**Fields:**
- `account`: Address that executed the action

### NFT Contract Events

#### `Mint`
Emitted when a new NFT is created.

**Fields:**
- `to`: Address receiving the NFT
- `token_id`: Token ID

#### `StateTrans` (StateTransitioned)
Emitted when lifecycle state changes.

**Fields:**
- `token_id`: NFT ID
- `from_st`: Previous state
- `to_st`: New state
- `updated`: Address that performed the update

#### `MetaUpd` (MetadataUpdated)
Emitted when metadata is updated.

**Fields:**
- `token_id`: NFT ID

#### `Whitelist` (TransferWhitelistUpdated)
Emitted when whitelist changes.

**Fields:**
- `account`: Affected address
- `added`: true if added, false if removed

#### `Transfer`
Emitted when an NFT is transferred (ERC-721 standard).

**Fields:**
- `from`: Source address
- `to`: Destination address
- `token_id`: Token ID

### Auditing

The system provides complete auditing through:

1. **Immutable Events**: All events are stored on the blockchain
2. **Complete History**: Each state transition is saved with timestamp and responsible party
3. **Preserved Metadata**: All historical metadata is maintained
4. **End-to-End Traceability**: From registration to consumption

### Audit Queries

```rust
// Get complete history of an NFT
let history = client.get_history(&token_id);
for transition in history {
    println!("From {:?} to {:?} by {} at {}", 
        transition.from_state,
        transition.to_state,
        transition.updated_by,
        transition.timestamp
    );
}
```

---

## Security Considerations

### Implemented Measures

#### 1. Role-Based Access Control
- Only addresses with appropriate roles can perform actions
- Roles are stored in persistent storage
- Verification on each critical operation

#### 2. Transition Validation
- Cannot skip states
- Cannot rollback in lifecycle
- Each transition requires correct role

#### 3. Whitelist for Transfers
- Only authorized addresses can receive NFTs
- Prevents unauthorized transfers
- Controlled by ADMIN

#### 4. Pause Functionality
- Allows stopping operations in case of emergency
- Only ADMIN can pause/resume
- Useful for responding to vulnerabilities

#### 5. Authentication
- All functions that modify state require `require_auth()`
- Prevents unauthorized execution

#### 6. Data Validation
- Existence check before creation
- Limit validation (e.g., maximum batch size)
- Type and format validation

### Additional Recommendations

#### For Production

1. **External Audit**: Conduct security audit before deployment
2. **Upgradeability**: Consider implementing upgradeability for future fixes
3. **Rate Limiting**: Implement rate limits to prevent spam
4. **Monitoring**: Configure event and transaction monitoring
5. **Backup**: Maintain backups of critical data off-chain

#### Best Practices

1. **Principle of Least Privilege**: Grant only necessary roles
2. **Key Rotation**: Rotate administrator keys periodically
3. **Code Review**: Review all changes before deployment
4. **Exhaustive Testing**: Increase test coverage, especially integration
5. **Documentation**: Keep documentation updated

---

## Usage Examples

### Example 1: Initial Setup

```rust
let env = Env::default();

// 1. Deploy NFT contract
let nft_contract_id = env.register_contract(None, SeedNFTContract);
let nft_client = SeedNFTContractClient::new(&env, &nft_contract_id);
nft_client.initialize(
    &admin,
    &String::from_str(&env, "Cannabis Seed NFT"),
    &String::from_str(&env, "CSNFT")
);

// 2. Deploy Registry contract
let registry_contract_id = env.register_contract(None, SeedRegistry);
let registry_client = SeedRegistryClient::new(&env, &registry_contract_id);
registry_client.initialize(&admin, &nft_contract_id);

// 3. Configure roles
registry_client.grant_role(&admin, &cultivator, &ROLE_CULTIVATOR);
registry_client.grant_role(&admin, &processor, &ROLE_PROCESSOR);
registry_client.grant_role(&admin, &dispensary, &ROLE_DISPENSARY);

// 4. Configure whitelist
nft_client.add_to_whitelist(&admin, &dispensary);
nft_client.add_to_whitelist(&admin, &processor);
```

### Example 2: Registration and Complete Cycle

```rust
// Register seed
let nft_id = registry_client.register_seed(
    &String::from_str(&env, "SEED-2024-001"),
    &String::from_str(&env, "Indica"),
    &String::from_str(&env, "BATCH-2024-001"),
    &String::from_str(&env, "Colombia"),
    &String::from_str(&env, "Premium Seeds Bank"),
    Some(20u32),  // 20% THC
    Some(2u32),   // 2% CBD
    true          // Organic
);

// Update to germinated
nft_client.update_state(
    &cultivator,
    &nft_id,
    &(LifecycleState::Germinated as u32),
    &Some(String::from_str(&env, "Greenhouse A, Section 3")),
    &Some(24i32),
    &Some(65u32),
    &None
);

// Continue cycle...
// (PlantVegetative, PlantFlowering, etc.)

// Process
nft_client.update_state(
    &processor,
    &nft_id,
    &(LifecycleState::Processed as u32),
    &None,
    &None,
    &None,
    &Some(String::from_str(&env, "Drying and curing completed"))
);

// Distribute
nft_client.update_state(
    &dispensary,
    &nft_id,
    &(LifecycleState::Distributed as u32),
    &Some(String::from_str(&env, "Central Dispensary")),
    &None,
    &None,
    &None
);

// Consume
nft_client.update_state(
    &dispensary,
    &nft_id,
    &(LifecycleState::Consumed as u32),
    &None,
    &None,
    &None,
    &None
);
```

### Example 3: Traceability Query

```rust
// Get seed information
let seed_data = registry_client.get_seed(
    &String::from_str(&env, "SEED-2024-001")
).unwrap();

println!("Variety: {}", seed_data.variety);
println!("Batch: {}", seed_data.batch);
println!("NFT ID: {}", seed_data.nft_id);

// Get current state
let metadata = nft_client.get_metadata(&seed_data.nft_id).unwrap();
println!("Current state: {:?}", metadata.state);
println!("Last update: {}", metadata.updated_at);

// Get complete history
let history = nft_client.get_history(&seed_data.nft_id);
for (i, transition) in history.iter().enumerate() {
    println!("Transition {}: {:?} -> {:?} by {} at {}",
        i + 1,
        transition.from_state,
        transition.to_state,
        transition.updated_by,
        transition.timestamp
    );
}
```

### Example 4: Batch Operations

```rust
// Prepare data
let seed_ids = vec![
    &env,
    String::from_str(&env, "SEED-001"),
    String::from_str(&env, "SEED-002"),
    String::from_str(&env, "SEED-003"),
];

let varieties = vec![
    &env,
    String::from_str(&env, "Indica"),
    String::from_str(&env, "Sativa"),
    String::from_str(&env, "Hybrid"),
];

// ... other vectors

// Register complete batch
let nft_ids = registry_client.register_seeds_batch(
    &seed_ids,
    &varieties,
    &batches,
    &origin_countries,
    &seed_banks,
    &expected_thcs,
    &expected_cbds,
    &organic_flags
);
```

---

## Conclusion

This smart contract system provides a complete solution for cannabis seed traceability on the Stellar blockchain. With its modular architecture, robust role system, and exhaustive validations, the system is designed to meet regulatory requirements while providing transparency and reliability.

### Key Features

✅ Complete end-to-end traceability  
✅ Granular role system  
✅ Strict transition validation  
✅ Rich and extensible metadata  
✅ Immutable history  
✅ Controlled transfers  
✅ Events for auditing  
✅ Pause functionality for emergencies  

### Next Steps

1. Complete integration tests between contracts
2. Implement gas optimization improvements
3. Add additional functionalities as needed
4. Conduct security audit
5. Deploy to testnet for testing
6. Prepare end-user documentation

---

**Document Version**: 1.0  
**Last Updated**: 2024  
**Author**: Cannabis Traceability System  
**License**: See LICENSE file in repository

