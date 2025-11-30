# English Documentation

This directory contains the English technical documentation for the Cannabis Seed Traceability System.

## Table of Contents

- [Documentation Files](#documentation-files)
- [Quick Start](#quick-start)
- [Documentation Overview](#documentation-overview)
- [Related Resources](#related-resources)

---

## Documentation Files

### Cannabis Traceability Contracts

**File**: [`CANNABIS_TRACEABILITY_CONTRACTS.md`](CANNABIS_TRACEABILITY_CONTRACTS.md)

Complete reference documentation for the cannabis seed traceability smart contracts, including:

- Detailed contract specifications
- Function references and API documentation
- Usage examples and code snippets
- Workflow documentation
- Security considerations
- Integration patterns

**Key Sections**:
- [Seed Registry Contract](CANNABIS_TRACEABILITY_CONTRACTS.md#seed-registry-contract)
- [Seed NFT Contract](CANNABIS_TRACEABILITY_CONTRACTS.md#seed-nft-contract)
- [Lifecycle Management](CANNABIS_TRACEABILITY_CONTRACTS.md#lifecycle-management)
- [Role-Based Access Control](CANNABIS_TRACEABILITY_CONTRACTS.md#role-based-access-control)

### Soroban Contracts Documentation

**File**: [`SOROBAN_CONTRACTS_DOCUMENTATION.md`](SOROBAN_CONTRACTS_DOCUMENTATION.md)

Comprehensive development guide for working with the Soroban smart contracts, covering:

- Development setup and prerequisites
- Compilation and build instructions
- Testing strategies
- Local network deployment
- Best practices and conventions
- Troubleshooting guide

**Key Sections**:
- [Development Setup](SOROBAN_CONTRACTS_DOCUMENTATION.md#development-setup)
- [Building Contracts](SOROBAN_CONTRACTS_DOCUMENTATION.md#building-contracts)
- [Testing](SOROBAN_CONTRACTS_DOCUMENTATION.md#testing)
- [Deployment](SOROBAN_CONTRACTS_DOCUMENTATION.md#deployment-and-testing-on-local-network)

---

## Quick Start

### For Developers

1. **Start Here**: Read [`SOROBAN_CONTRACTS_DOCUMENTATION.md`](SOROBAN_CONTRACTS_DOCUMENTATION.md) to set up your development environment
2. **Understand the Contracts**: Review [`CANNABIS_TRACEABILITY_CONTRACTS.md`](CANNABIS_TRACEABILITY_CONTRACTS.md) for contract specifications
3. **Build and Test**: Follow the build and testing instructions in the Soroban documentation

### For Integrators

1. **API Reference**: Start with [`CANNABIS_TRACEABILITY_CONTRACTS.md`](CANNABIS_TRACEABILITY_CONTRACTS.md) for function references
2. **Integration Patterns**: Review the integration examples in the contract documentation
3. **Deployment Guide**: See deployment instructions in [`SOROBAN_CONTRACTS_DOCUMENTATION.md`](SOROBAN_CONTRACTS_DOCUMENTATION.md)

---

## Documentation Overview

### Contract Architecture

The system consists of two main smart contracts:

1. **Seed Registry Contract**: Central entry point for seed registration and management
2. **Seed NFT Contract**: Manages lifecycle states, metadata, and NFT transfers

Both contracts work together to provide complete end-to-end traceability from seed registration to consumption.

### Key Concepts

- **Lifecycle States**: 8 distinct states representing the seed's journey
- **Role-Based Access Control**: Granular permissions for different actors
- **Immutable History**: Complete audit trail of all state transitions
- **OpenSea Compatibility**: NFT metadata follows OpenSea standards

---

## Related Resources

### Project Documentation

- **Main README**: [`../../README.md`](../../README.md) - High-level project overview
- **Documentation Index**: [`../README.md`](../README.md) - Complete documentation index

### Spanish Documentation

For Spanish documentation, see [`../es/README.md`](../es/README.md)

### External Resources

- **Soroban Documentation**: https://soroban.stellar.org/docs
- **Soroban SDK Reference**: https://docs.rs/soroban-sdk/
- **OpenZeppelin Stellar Contracts**: https://docs.openzeppelin.com/stellar-contracts
- **Stellar Developer Documentation**: https://developers.stellar.org/

---

## Documentation Structure

```
docs/en/
├── README.md                              # This file
├── CANNABIS_TRACEABILITY_CONTRACTS.md     # Contract specifications
└── SOROBAN_CONTRACTS_DOCUMENTATION.md     # Development guide
```

---

## Contributing

When updating English documentation:

1. **Maintain consistency**: Keep terminology consistent across all documents
2. **Update both versions**: If applicable, update the corresponding Spanish documentation
3. **Cross-reference**: Link to related sections and external resources
4. **Keep current**: Sync documentation with code changes

