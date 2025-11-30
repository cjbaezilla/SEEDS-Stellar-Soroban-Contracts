#![cfg(test)]
use soroban_sdk::{
    symbol_short, testutils::Address as _, Address, Env, String, Symbol,
};
use crate::{SeedRegistry, SeedRegistryClient};
use crate::roles::{ROLE_ADMIN, ROLE_CULTIVATOR};

#[test]
fn test_initialize() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let nft_contract = Address::generate(&env);
    
    let contract_id = env.register_contract(None, SeedRegistry);
    let client = SeedRegistryClient::new(&env, &contract_id);
    
    client.initialize(&admin, &nft_contract);
    
    assert_eq!(client.get_seed_count(), 0);
    assert!(client.has_role(&admin, &ROLE_ADMIN));
}

#[test]
fn test_register_seed() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let cultivator = Address::generate(&env);
    let nft_contract = Address::generate(&env);
    
    let contract_id = env.register_contract(None, SeedRegistry);
    let client = SeedRegistryClient::new(&env, &contract_id);
    
    client.initialize(&admin, &nft_contract);
    client.grant_role(&admin, &cultivator, &ROLE_CULTIVATOR);
    
    let seed_id = String::from_str(&env, "SEED-001");
    let variety = String::from_str(&env, "Indica");
    let batch = String::from_str(&env, "BATCH-2024-001");
    let origin_country = String::from_str(&env, "Colombia");
    let seed_bank = String::from_str(&env, "Bank-001");
    
    env.as_contract(&contract_id, || {
        env.prng().seed(12345);
    });
    
    assert_eq!(client.get_seed_count(), 0);
}

#[test]
fn test_roles() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let nft_contract = Address::generate(&env);
    
    let contract_id = env.register_contract(None, SeedRegistry);
    let client = SeedRegistryClient::new(&env, &contract_id);
    
    client.initialize(&admin, &nft_contract);
    
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
    let nft_contract = Address::generate(&env);
    
    let contract_id = env.register_contract(None, SeedRegistry);
    let client = SeedRegistryClient::new(&env, &contract_id);
    
    client.initialize(&admin, &nft_contract);
    
    assert!(!client.is_paused());
    
    client.pause(&admin);
    assert!(client.is_paused());
    
    client.unpause(&admin);
    assert!(!client.is_paused());
}

#[test]
fn test_queries() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let nft_contract = Address::generate(&env);
    
    let contract_id = env.register_contract(None, SeedRegistry);
    let client = SeedRegistryClient::new(&env, &contract_id);
    
    client.initialize(&admin, &nft_contract);
    
    let all_ids = client.get_all_seed_ids();
    assert_eq!(all_ids.len(), 0);
    
    let by_variety = client.query_seeds_by_variety(&String::from_str(&env, "Indica"));
    assert_eq!(by_variety.len(), 0);
    
    let by_batch = client.query_seeds_by_batch(&String::from_str(&env, "BATCH-001"));
    assert_eq!(by_batch.len(), 0);
    
    let by_creator = client.query_seeds_by_creator(&admin);
    assert_eq!(by_creator.len(), 0);
}

