use soroban_sdk::{
    symbol_short, Address, Env, String, Symbol, Vec,
};
use crate::seed_data::SeedData;
use crate::roles::{ROLE_ADMIN, get_role_key};

pub struct Registry;

impl Registry {
    const SEED_COUNT: Symbol = symbol_short!("SEED_CNT");
    const SEED_DATA: Symbol = symbol_short!("SEED_DATA");
    const SEED_IDS: Symbol = symbol_short!("SEED_IDS");
    const NFT_CONTRACT: Symbol = symbol_short!("NFT_CNTR");
    const PAUSED: Symbol = symbol_short!("PAUSED");

    pub fn initialize(env: &Env, admin: Address, nft_contract: Address) {
        if env.storage().instance().has(&Self::SEED_COUNT) {
            panic!("Already initialized");
        }
        
        env.storage().instance().set(&Self::SEED_COUNT, &0u64);
        env.storage().instance().set(&Self::NFT_CONTRACT, &nft_contract);
        env.storage().instance().set(&Self::PAUSED, &false);
        
        Self::grant_role(env, &admin, ROLE_ADMIN);
    }

    pub fn require_not_paused(env: &Env) {
        let paused: bool = env.storage().instance().get(&Self::PAUSED).unwrap_or(false);
        if paused {
            panic!("Contract is paused");
        }
    }

    pub fn require_role(env: &Env, account: &Address, role: Symbol) {
        let role_key = get_role_key(role);
        let has_role: bool = env
            .storage()
            .persistent()
            .get(&(role_key, account.clone()))
            .unwrap_or(false);
        if !has_role {
            panic!("Missing required role");
        }
    }

    pub fn grant_role(env: &Env, account: &Address, role: Symbol) {
        let role_key = get_role_key(role);
        env.storage()
            .persistent()
            .set(&(role_key, account.clone()), &true);
    }

    pub fn revoke_role(env: &Env, account: &Address, role: Symbol) {
        let role_key = get_role_key(role);
        env.storage().persistent().remove(&(role_key, account.clone()));
    }

    pub fn has_role(env: &Env, account: &Address, role: Symbol) -> bool {
        let role_key = get_role_key(role);
        env.storage()
            .persistent()
            .get(&(role_key, account.clone()))
            .unwrap_or(false)
    }

    pub fn pause(env: &Env, account: &Address) {
        Self::require_role(env, account, ROLE_ADMIN);
        env.storage().instance().set(&Self::PAUSED, &true);
    }

    pub fn unpause(env: &Env, account: &Address) {
        Self::require_role(env, account, ROLE_ADMIN);
        env.storage().instance().set(&Self::PAUSED, &false);
    }

    pub fn get_seed_count(env: &Env) -> u64 {
        env.storage()
            .instance()
            .get(&Self::SEED_COUNT)
            .unwrap_or(0u64)
    }

    pub fn increment_seed_count(env: &Env) -> u64 {
        let mut count = Self::get_seed_count(env);
        count += 1;
        env.storage().instance().set(&Self::SEED_COUNT, &count);
        count
    }

    pub fn store_seed_data(env: &Env, seed_id: &String, data: &SeedData) {
        env.storage()
            .persistent()
            .set(&(Self::SEED_DATA, seed_id.clone()), data);
    }

    pub fn get_seed_data(env: &Env, seed_id: &String) -> Option<SeedData> {
        env.storage()
            .persistent()
            .get(&(Self::SEED_DATA, seed_id.clone()))
    }

    pub fn add_seed_id(env: &Env, seed_id: &String) {
        let mut ids: Vec<String> = env
            .storage()
            .persistent()
            .get(&Self::SEED_IDS)
            .unwrap_or_else(|| Vec::new(env));
        
        ids.push_back(seed_id.clone());
        env.storage().persistent().set(&Self::SEED_IDS, &ids);
    }

    pub fn get_seed_ids(env: &Env) -> Vec<String> {
        env.storage()
            .persistent()
            .get(&Self::SEED_IDS)
            .unwrap_or_else(|| Vec::new(env))
    }

    pub fn get_nft_contract(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&Self::NFT_CONTRACT)
            .unwrap()
    }

    pub fn set_nft_contract(env: &Env, account: &Address, nft_contract: &Address) {
        Self::require_role(env, account, ROLE_ADMIN);
        env.storage().instance().set(&Self::NFT_CONTRACT, nft_contract);
    }
}

