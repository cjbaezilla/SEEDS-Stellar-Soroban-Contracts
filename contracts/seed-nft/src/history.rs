use soroban_sdk::{Address, String, Vec, contracttype, symbol_short};
use crate::lifecycle::LifecycleState;

#[contracttype]
#[derive(Clone)]
pub struct StateTransition {
    pub from_state: LifecycleState,
    pub to_state: LifecycleState,
    pub timestamp: u64,
    pub updated_by: Address,
    pub notes: Option<String>,
}

pub struct History;

impl History {
    pub fn add_transition(
        env: &soroban_sdk::Env,
        token_id: &u128,
        transition: &StateTransition,
    ) {
        let key = (symbol_short!("HISTORY"), token_id);
        let mut history: Vec<StateTransition> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Vec::new(env));
        
        history.push_back(transition.clone());
        env.storage().persistent().set(&key, &history);
    }

    pub fn get_history(
        env: &soroban_sdk::Env,
        token_id: &u128,
    ) -> Vec<StateTransition> {
        let key = (symbol_short!("HISTORY"), token_id);
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Vec::new(env))
    }
}

