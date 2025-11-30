#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror, contractevent, symbol_short,
    Address, Env, String, Vec, Symbol,
};
use crate::registry::Registry;
use crate::seed_data::SeedData;
use crate::roles::{ROLE_ADMIN, ROLE_CULTIVATOR};

mod registry;
mod seed_data;
mod roles;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    Paused = 4,
    SeedNotFound = 5,
    InvalidInput = 6,
}

#[contractevent]
pub struct SeedRegisteredEvent {
    pub seed_id: String,
    pub nft_id: u128,
    pub creator: Address,
    pub created_at: u64,
}

#[contractevent]
pub struct SeedRegisteredBatchEvent {
    pub seed_id: String,
    pub nft_id: u128,
}

#[contractevent]
pub struct RoleGrantEvent {
    pub account: Address,
    pub role: Symbol,
}

#[contractevent]
pub struct RoleRevokeEvent {
    pub account: Address,
    pub role: Symbol,
}

#[contractevent]
pub struct PausedEvent {
    pub account: Address,
}

#[contractevent]
pub struct UnpausedEvent {
    pub account: Address,
}

#[contract]
pub struct SeedRegistry;

#[contractimpl]
impl SeedRegistry {
    pub fn initialize(env: Env, admin: Address, nft_contract: Address) {
        Registry::initialize(&env, admin, nft_contract);
    }

    pub fn register_seed(
        env: Env,
        seed_id: String,
        variety: String,
        batch: String,
        origin_country: String,
        seed_bank: String,
        expected_thc: Option<u32>,
        expected_cbd: Option<u32>,
        organic_certified: bool,
    ) -> u128 {
        Registry::require_not_paused(&env);
        let creator = env.current_contract_address();
        creator.require_auth();
        let creator_clone = creator.clone();
        Registry::require_role(&env, &creator_clone, ROLE_CULTIVATOR);

        if Registry::get_seed_data(&env, &seed_id).is_some() {
            panic!("Seed already exists");
        }
        let created_at = env.ledger().timestamp();
        let nft_contract = Registry::get_nft_contract(&env);
        
        let creator_for_nft = creator.clone();
        let nft_id = Self::create_nft(&env, &nft_contract, &seed_id, &creator_for_nft);
        
        let creator_for_data = creator.clone();
        let seed_data = SeedData {
            id: seed_id.clone(),
            created_at,
            creator: creator_for_data,
            variety,
            batch,
            origin_country,
            seed_bank,
            expected_thc,
            expected_cbd,
            organic_certified,
            nft_id,
            nft_contract: nft_contract.clone(),
        };

        Registry::store_seed_data(&env, &seed_id, &seed_data);
        Registry::add_seed_id(&env, &seed_id);
        Registry::increment_seed_count(&env);

        SeedRegisteredEvent {
            seed_id: seed_id.clone(),
            nft_id,
            creator: creator.clone(),
            created_at,
        }.publish(&env);

        nft_id
    }

    pub fn register_seeds_batch(
        env: Env,
        seed_ids: Vec<String>,
        varieties: Vec<String>,
        batches: Vec<String>,
        origin_countries: Vec<String>,
        seed_banks: Vec<String>,
        expected_thcs: Vec<Option<u32>>,
        expected_cbds: Vec<Option<u32>>,
        organic_certified_flags: Vec<bool>,
    ) -> Vec<u128> {
        Registry::require_not_paused(&env);
        let creator = env.current_contract_address();
        creator.require_auth();
        let creator_clone = creator.clone();
        Registry::require_role(&env, &creator_clone, ROLE_CULTIVATOR);

        let len = seed_ids.len();
        if len != varieties.len()
            || len != batches.len()
            || len != origin_countries.len()
            || len != seed_banks.len()
            || len != expected_thcs.len()
            || len != expected_cbds.len()
            || len != organic_certified_flags.len()
        {
            panic!("Invalid batch input: lengths must match");
        }

        if len > 100 {
            panic!("Batch size too large: max 100");
        }

        let mut nft_ids = Vec::new(&env);
        let nft_contract = Registry::get_nft_contract(&env);

        for i in 0..len {
            let seed_id = seed_ids.get(i).unwrap();
            
            if Registry::get_seed_data(&env, &seed_id).is_some() {
                continue;
            }

            let creator = env.current_contract_address();
            creator.require_auth();
            let created_at = env.ledger().timestamp();
            let nft_id = Self::create_nft(&env, &nft_contract, &seed_id, &creator);
            
            let seed_data = SeedData {
                id: seed_id.clone(),
                created_at,
                creator,
                variety: varieties.get(i).unwrap().clone(),
                batch: batches.get(i).unwrap().clone(),
                origin_country: origin_countries.get(i).unwrap().clone(),
                seed_bank: seed_banks.get(i).unwrap().clone(),
                expected_thc: expected_thcs.get(i).unwrap().clone(),
                expected_cbd: expected_cbds.get(i).unwrap().clone(),
                organic_certified: organic_certified_flags.get(i).unwrap().clone(),
                nft_id,
                nft_contract: nft_contract.clone(),
            };

            Registry::store_seed_data(&env, &seed_id, &seed_data);
            Registry::add_seed_id(&env, &seed_id);
            Registry::increment_seed_count(&env);
            nft_ids.push_back(nft_id);

            SeedRegisteredBatchEvent {
                seed_id: seed_id.clone(),
                nft_id,
            }.publish(&env);
        }

        nft_ids
    }

    fn create_nft(
        env: &Env,
        _nft_contract: &Address,
        _seed_id: &String,
        _creator: &Address,
    ) -> u128 {
        let nft_id = (Registry::get_seed_count(env) + 1) as u128;
        
        nft_id
    }

    pub fn get_seed(env: Env, seed_id: String) -> Option<SeedData> {
        Registry::get_seed_data(&env, &seed_id)
    }

    pub fn get_seed_count(env: Env) -> u64 {
        Registry::get_seed_count(&env)
    }

    pub fn get_all_seed_ids(env: Env) -> Vec<String> {
        Registry::get_seed_ids(&env)
    }

    pub fn query_seeds_by_variety(env: Env, variety: String) -> Vec<String> {
        let all_ids = Registry::get_seed_ids(&env);
        let mut result = Vec::new(&env);
        
        for i in 0..all_ids.len() {
            let seed_id = all_ids.get(i).unwrap();
            if let Some(seed_data) = Registry::get_seed_data(&env, &seed_id) {
                if seed_data.variety == variety {
                    result.push_back(seed_id.clone());
                }
            }
        }
        
        result
    }

    pub fn query_seeds_by_batch(env: Env, batch: String) -> Vec<String> {
        let all_ids = Registry::get_seed_ids(&env);
        let mut result = Vec::new(&env);
        
        for i in 0..all_ids.len() {
            let seed_id = all_ids.get(i).unwrap();
            if let Some(seed_data) = Registry::get_seed_data(&env, &seed_id) {
                if seed_data.batch == batch {
                    result.push_back(seed_id.clone());
                }
            }
        }
        
        result
    }

    pub fn query_seeds_by_creator(env: Env, creator: Address) -> Vec<String> {
        let all_ids = Registry::get_seed_ids(&env);
        let mut result = Vec::new(&env);
        
        for i in 0..all_ids.len() {
            let seed_id = all_ids.get(i).unwrap();
            if let Some(seed_data) = Registry::get_seed_data(&env, &seed_id) {
                if seed_data.creator == creator {
                    result.push_back(seed_id.clone());
                }
            }
        }
        
        result
    }

    pub fn grant_role(env: Env, caller: Address, account: Address, role: Symbol) {
        caller.require_auth();
        Registry::require_role(&env, &caller, ROLE_ADMIN);
        let role_clone = role.clone();
        Registry::grant_role(&env, &account, role);
        
        RoleGrantEvent { account, role: role_clone }.publish(&env);
    }

    pub fn revoke_role(env: Env, caller: Address, account: Address, role: Symbol) {
        caller.require_auth();
        Registry::require_role(&env, &caller, ROLE_ADMIN);
        let role_clone = role.clone();
        Registry::revoke_role(&env, &account, role);
        
        RoleRevokeEvent { account, role: role_clone }.publish(&env);
    }

    pub fn has_role(env: Env, account: Address, role: Symbol) -> bool {
        Registry::has_role(&env, &account, role)
    }

    pub fn pause(env: Env, caller: Address) {
        caller.require_auth();
        Registry::pause(&env, &caller);
        PausedEvent { account: caller }.publish(&env);
    }

    pub fn unpause(env: Env, caller: Address) {
        caller.require_auth();
        Registry::unpause(&env, &caller);
        UnpausedEvent { account: caller }.publish(&env);
    }

    pub fn is_paused(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&symbol_short!("PAUSED"))
            .unwrap_or(false)
    }

    pub fn set_nft_contract(env: Env, caller: Address, nft_contract: Address) {
        caller.require_auth();
        Registry::set_nft_contract(&env, &caller, &nft_contract);
    }

    pub fn get_nft_contract(env: Env) -> Address {
        Registry::get_nft_contract(&env)
    }
}

#[cfg(test)]
mod test;

