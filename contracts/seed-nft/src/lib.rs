#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror, contractevent, symbol_short,
    Address, Env, String, Symbol, Vec,
};
use stellar_tokens::non_fungible::{NonFungibleToken, Base};
use stellar_macros::default_impl;
use crate::nft::SeedNFT;
use crate::metadata::{SeedMetadata, Attribute, OpenSeaMetadata};
use crate::lifecycle::LifecycleState;
use crate::history::{History, StateTransition};
use crate::roles::{ROLE_ADMIN, ROLE_CULTIVATOR, ROLE_PROCESSOR, ROLE_DISPENSARY};

mod nft;
mod metadata;
mod lifecycle;
mod history;
mod roles;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Unauthorized = 1,
    InvalidStateTransition = 2,
    NotWhitelisted = 3,
    Paused = 4,
    TokenNotFound = 5,
}

#[contractevent]
pub struct MintEvent {
    pub to: Address,
    pub token_id: u128,
}

#[contractevent]
pub struct StateTransitionEvent {
    pub token_id: u128,
    pub from_state: u128,
    pub to_state: u128,
    pub updated_by: Address,
}

#[contractevent]
pub struct MetadataUpdateEvent {
    pub token_id: u128,
}

#[contractevent]
pub struct WhitelistEvent {
    pub account: Address,
    pub added: bool,
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
pub struct SeedNFTContract;

#[default_impl]
#[contractimpl]
impl NonFungibleToken for SeedNFTContract {
    type ContractType = Base;
    
    fn transfer_from(e: &Env, spender: Address, from: Address, to: Address, token_id: u32) {
        SeedNFT::require_not_paused(e);
        
        if !SeedNFT::is_whitelisted(e, &to) {
            panic!("Recipient not whitelisted");
        }
        
        Self::ContractType::transfer_from(e, &spender, &from, &to, token_id);
    }
    
    fn approve(e: &Env, approver: Address, approved: Address, token_id: u32, live_until_ledger: u32) {
        SeedNFT::require_not_paused(e);
        Self::ContractType::approve(e, &approver, &approved, token_id, live_until_ledger);
    }
}

#[contractimpl]
impl SeedNFTContract {
    pub fn initialize(env: Env, admin: Address, _name: String, _symbol: String) {
        if env.storage().instance().has(&symbol_short!("INIT")) {
            panic!("Already initialized");
        }
        
        env.storage().instance().set(&symbol_short!("INIT"), &true);
        env.storage().instance().set(&symbol_short!("PAUSED"), &false);
        
        SeedNFT::grant_role(&env, &admin, ROLE_ADMIN);
    }

    pub fn mint(
        env: Env,
        to: Address,
        token_id: u128,
        name: String,
        description: String,
        image: String,
        external_url: Option<String>,
        attributes: Vec<Attribute>,
    ) {
        let token_id_u32 = token_id as u32;
        <SeedNFTContract as NonFungibleToken>::ContractType::mint(&env, &to, token_id_u32);
        
        let metadata = SeedMetadata {
            state: LifecycleState::Seed,
            location: None,
            temperature: None,
            humidity: None,
            lab_analysis: None,
            processor: None,
            distributor: None,
            consumer: None,
            updated_at: env.ledger().timestamp(),
            name,
            description,
            image,
            external_url,
            attributes,
        };
        
        SeedNFT::store_metadata(&env, &token_id, &metadata);
        
        MintEvent { to, token_id }.publish(&env);
    }

    pub fn update_state(
        env: Env,
        caller: Address,
        token_id: u128,
        new_state: u32,
        location: Option<String>,
        temperature: Option<i32>,
        humidity: Option<u32>,
        notes: Option<String>,
    ) {
        caller.require_auth();
        SeedNFT::require_not_paused(&env);
        
        let state = LifecycleState::from_u32(new_state)
            .unwrap_or_else(|| panic!("Invalid state"));
        
        match state {
            LifecycleState::Germinated | LifecycleState::PlantVegetative 
            | LifecycleState::PlantFlowering | LifecycleState::PlantHarvested => {
                SeedNFT::require_role(&env, &caller, ROLE_CULTIVATOR);
            }
            LifecycleState::Processed => {
                SeedNFT::require_role(&env, &caller, ROLE_PROCESSOR);
            }
            LifecycleState::Distributed => {
                SeedNFT::require_role(&env, &caller, ROLE_DISPENSARY);
            }
            LifecycleState::Consumed => {
                SeedNFT::require_role(&env, &caller, ROLE_DISPENSARY);
            }
            _ => panic!("Invalid state transition"),
        }
        
        let mut metadata = SeedNFT::get_metadata(&env, &token_id)
            .unwrap_or_else(|| panic!("Token not found"));
        
        if !metadata.state.can_transition_to(state) {
            panic!("Invalid state transition");
        }
        
        let from_state = metadata.state;
        let timestamp = env.ledger().timestamp();
        
        metadata.state = state;
        metadata.location = location;
        metadata.temperature = temperature;
        metadata.humidity = humidity;
        metadata.updated_at = timestamp;
        
        match state {
            LifecycleState::Processed => {
                metadata.processor = Some(caller.clone());
            }
            LifecycleState::Distributed => {
                metadata.distributor = Some(caller.clone());
            }
            LifecycleState::Consumed => {
                let token_id_u32 = token_id as u32;
                metadata.consumer = Some(<SeedNFTContract as NonFungibleToken>::ContractType::owner_of(&env, token_id_u32));
            }
            _ => {}
        }
        
        SeedNFT::store_metadata(&env, &token_id, &metadata);
        
        let transition = StateTransition {
            from_state,
            to_state: state,
            timestamp,
            updated_by: caller.clone(),
            notes,
        };
        History::add_transition(&env, &token_id, &transition);
        
        StateTransitionEvent {
            token_id,
            from_state: from_state.to_u32() as u128,
            to_state: state.to_u32() as u128,
            updated_by: caller,
        }.publish(&env);
    }

    pub fn update_metadata(
        env: Env,
        caller: Address,
        token_id: u128,
        location: Option<String>,
        temperature: Option<i32>,
        humidity: Option<u32>,
        lab_analysis: Option<String>,
        opensea_metadata: Option<OpenSeaMetadata>,
    ) {
        caller.require_auth();
        SeedNFT::require_not_paused(&env);
        SeedNFT::require_role(&env, &caller, ROLE_CULTIVATOR);
        
        let mut metadata = SeedNFT::get_metadata(&env, &token_id)
            .unwrap_or_else(|| panic!("Token not found"));
        
        if location.is_some() {
            metadata.location = location;
        }
        if temperature.is_some() {
            metadata.temperature = temperature;
        }
        if humidity.is_some() {
            metadata.humidity = humidity;
        }
        if lab_analysis.is_some() {
            metadata.lab_analysis = lab_analysis;
        }
        if let Some(opensea) = opensea_metadata {
            if opensea.name.is_some() {
                metadata.name = opensea.name.unwrap();
            }
            if opensea.description.is_some() {
                metadata.description = opensea.description.unwrap();
            }
            if opensea.image.is_some() {
                metadata.image = opensea.image.unwrap();
            }
            if opensea.external_url.is_some() {
                metadata.external_url = opensea.external_url;
            }
            if opensea.attributes.is_some() {
                metadata.attributes = opensea.attributes.unwrap();
            }
        }
        metadata.updated_at = env.ledger().timestamp();
        
        SeedNFT::store_metadata(&env, &token_id, &metadata);
        
        MetadataUpdateEvent { token_id }.publish(&env);
    }

    pub fn get_metadata(env: Env, token_id: u128) -> Option<SeedMetadata> {
        SeedNFT::get_metadata(&env, &token_id)
    }

    pub fn get_history(env: Env, token_id: u128) -> Vec<StateTransition> {
        History::get_history(&env, &token_id)
    }

    pub fn add_to_whitelist(env: Env, caller: Address, account: Address) {
        caller.require_auth();
        SeedNFT::add_to_whitelist(&env, &caller, &account);
        WhitelistEvent { account, added: true }.publish(&env);
    }

    pub fn remove_from_whitelist(env: Env, caller: Address, account: Address) {
        caller.require_auth();
        SeedNFT::remove_from_whitelist(&env, &caller, &account);
        WhitelistEvent { account, added: false }.publish(&env);
    }

    pub fn is_whitelisted(env: Env, account: Address) -> bool {
        SeedNFT::is_whitelisted(&env, &account)
    }

    pub fn grant_role(env: Env, caller: Address, account: Address, role: Symbol) {
        caller.require_auth();
        SeedNFT::require_role(&env, &caller, ROLE_ADMIN);
        let role_clone = role.clone();
        SeedNFT::grant_role(&env, &account, role);
        RoleGrantEvent { account, role: role_clone }.publish(&env);
    }

    pub fn revoke_role(env: Env, caller: Address, account: Address, role: Symbol) {
        caller.require_auth();
        SeedNFT::require_role(&env, &caller, ROLE_ADMIN);
        let role_clone = role.clone();
        SeedNFT::revoke_role(&env, &account, role);
        RoleRevokeEvent { account, role: role_clone }.publish(&env);
    }

    pub fn has_role(env: Env, account: Address, role: Symbol) -> bool {
        SeedNFT::has_role(&env, &account, role)
    }

    pub fn pause(env: Env, caller: Address) {
        caller.require_auth();
        SeedNFT::pause(&env, &caller);
        PausedEvent { account: caller }.publish(&env);
    }

    pub fn unpause(env: Env, caller: Address) {
        caller.require_auth();
        SeedNFT::unpause(&env, &caller);
        UnpausedEvent { account: caller }.publish(&env);
    }

    pub fn is_paused(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&symbol_short!("PAUSED"))
            .unwrap_or(false)
    }

}

#[cfg(test)]
mod test;

