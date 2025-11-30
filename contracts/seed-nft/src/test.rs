#![cfg(test)]
use soroban_sdk::{
    symbol_short, testutils::Address as _, Address, Env, String, Symbol,
};
use crate::{SeedNFTContract, SeedNFTContractClient};
use crate::lifecycle::LifecycleState;
use crate::roles::{ROLE_ADMIN, ROLE_CULTIVATOR, ROLE_PROCESSOR, ROLE_DISPENSARY};

#[test]
fn test_initialize() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let name = String::from_str(&env, "Cannabis Seed NFT");
    let symbol = String::from_str(&env, "CSNFT");
    
    let contract_id = env.register_contract(None, SeedNFTContract);
    let client = SeedNFTContractClient::new(&env, &contract_id);
    
    client.initialize(&admin, &name, &symbol);
    
    assert_eq!(client.name(), name);
    assert_eq!(client.symbol(), symbol);
    assert!(client.has_role(&admin, &ROLE_ADMIN));
    assert!(!client.is_paused());
}

#[test]
fn test_mint() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let name = String::from_str(&env, "Cannabis Seed NFT");
    let symbol = String::from_str(&env, "CSNFT");
    
    let contract_id = env.register_contract(None, SeedNFTContract);
    let client = SeedNFTContractClient::new(&env, &contract_id);
    
    client.initialize(&admin, &name, &symbol);
    
    let token_id = 1u128;
    client.mint(&owner, &token_id);
    
    assert_eq!(client.balance_of(&owner), 1);
    assert_eq!(client.owner_of(&token_id), owner);
    
    let metadata = client.get_metadata(&token_id);
    assert!(metadata.is_some());
    if let Some(meta) = metadata {
        assert_eq!(meta.state as u32, LifecycleState::Seed as u32);
    }
}

#[test]
fn test_state_transitions() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let cultivator = Address::generate(&env);
    let processor = Address::generate(&env);
    let dispensary = Address::generate(&env);
    let owner = Address::generate(&env);
    let name = String::from_str(&env, "Cannabis Seed NFT");
    let symbol = String::from_str(&env, "CSNFT");
    
    let contract_id = env.register_contract(None, SeedNFTContract);
    let client = SeedNFTContractClient::new(&env, &contract_id);
    
    client.initialize(&admin, &name, &symbol);
    client.grant_role(&admin, &cultivator, &ROLE_CULTIVATOR);
    client.grant_role(&admin, &processor, &ROLE_PROCESSOR);
    client.grant_role(&admin, &dispensary, &ROLE_DISPENSARY);
    
    let token_id = 1u128;
    client.mint(&owner, &token_id);
    
    client.update_state(
        &cultivator,
        &token_id,
        &(LifecycleState::Germinated as u32),
        &None,
        &None,
        &None,
        &None,
    );
    
    let metadata = client.get_metadata(&token_id).unwrap();
    assert_eq!(metadata.state as u32, LifecycleState::Germinated as u32);
    
    let history = client.get_history(&token_id);
    assert_eq!(history.len(), 1);
}

#[test]
fn test_whitelist() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let account = Address::generate(&env);
    let name = String::from_str(&env, "Cannabis Seed NFT");
    let symbol = String::from_str(&env, "CSNFT");
    
    let contract_id = env.register_contract(None, SeedNFTContract);
    let client = SeedNFTContractClient::new(&env, &contract_id);
    
    client.initialize(&admin, &name, &symbol);
    
    assert!(!client.is_whitelisted(&account));
    
    client.add_to_whitelist(&admin, &account);
    assert!(client.is_whitelisted(&account));
    
    client.remove_from_whitelist(&admin, &account);
    assert!(!client.is_whitelisted(&account));
}

#[test]
fn test_roles() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let name = String::from_str(&env, "Cannabis Seed NFT");
    let symbol = String::from_str(&env, "CSNFT");
    
    let contract_id = env.register_contract(None, SeedNFTContract);
    let client = SeedNFTContractClient::new(&env, &contract_id);
    
    client.initialize(&admin, &name, &symbol);
    
    assert!(!client.has_role(&user, &ROLE_CULTIVATOR));
    
    client.grant_role(&admin, &user, &ROLE_CULTIVATOR);
    assert!(client.has_role(&user, &ROLE_CULTIVATOR));
    
    client.revoke_role(&admin, &user, &ROLE_CULTIVATOR);
    assert!(!client.has_role(&user, &ROLE_CULTIVATOR));
}

#[test]
fn test_pause() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let name = String::from_str(&env, "Cannabis Seed NFT");
    let symbol = String::from_str(&env, "CSNFT");
    
    let contract_id = env.register_contract(None, SeedNFTContract);
    let client = SeedNFTContractClient::new(&env, &contract_id);
    
    client.initialize(&admin, &name, &symbol);
    
    assert!(!client.is_paused());
    
    client.pause(&admin);
    assert!(client.is_paused());
    
    client.unpause(&admin);
    assert!(!client.is_paused());
}

#[test]
fn test_metadata_update() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let cultivator = Address::generate(&env);
    let owner = Address::generate(&env);
    let name = String::from_str(&env, "Cannabis Seed NFT");
    let symbol = String::from_str(&env, "CSNFT");
    
    let contract_id = env.register_contract(None, SeedNFTContract);
    let client = SeedNFTContractClient::new(&env, &contract_id);
    
    client.initialize(&admin, &name, &symbol);
    client.grant_role(&admin, &cultivator, &ROLE_CULTIVATOR);
    
    let token_id = 1u128;
    client.mint(&owner, &token_id);
    
    let location = Some(String::from_str(&env, "40.7128,-74.0060"));
    let temperature = Some(25i32);
    let humidity = Some(60u32);
    let lab_analysis = Some(String::from_str(&env, "THC: 20%, CBD: 2%"));
    
    client.update_metadata(
        &cultivator,
        &token_id,
        &location,
        &temperature,
        &humidity,
        &lab_analysis,
    );
    
    let metadata = client.get_metadata(&token_id).unwrap();
    assert_eq!(metadata.location, location);
    assert_eq!(metadata.temperature, temperature);
    assert_eq!(metadata.humidity, humidity);
    assert_eq!(metadata.lab_analysis, lab_analysis);
}

