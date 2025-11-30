use soroban_sdk::{symbol_short, Address, Env, Symbol};
use crate::metadata::SeedMetadata;
use crate::roles::{ROLE_ADMIN, get_role_key};

pub struct SeedNFT;

impl SeedNFT {
    const METADATA: Symbol = symbol_short!("METADATA");
    const WHITELIST: Symbol = symbol_short!("WHITELIST");
    const PAUSED: Symbol = symbol_short!("PAUSED");

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

    pub fn store_metadata(env: &Env, token_id: &u128, metadata: &SeedMetadata) {
        env.storage()
            .persistent()
            .set(&(Self::METADATA, token_id), metadata);
    }

    pub fn get_metadata(env: &Env, token_id: &u128) -> Option<SeedMetadata> {
        env.storage()
            .persistent()
            .get(&(Self::METADATA, token_id))
    }

    pub fn add_to_whitelist(env: &Env, caller: &Address, account: &Address) {
        Self::require_role(env, caller, ROLE_ADMIN);
        env.storage()
            .persistent()
            .set(&(Self::WHITELIST, account.clone()), &true);
    }

    pub fn remove_from_whitelist(env: &Env, caller: &Address, account: &Address) {
        Self::require_role(env, caller, ROLE_ADMIN);
        env.storage().persistent().remove(&(Self::WHITELIST, account.clone()));
    }

    pub fn is_whitelisted(env: &Env, account: &Address) -> bool {
        env.storage()
            .persistent()
            .get(&(Self::WHITELIST, account.clone()))
            .unwrap_or(false)
    }
}

