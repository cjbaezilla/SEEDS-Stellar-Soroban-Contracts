use soroban_sdk::{symbol_short, Symbol};

pub const ROLE_ADMIN: Symbol = symbol_short!("ADMIN");
pub const ROLE_CULTIVATOR: Symbol = symbol_short!("CULTIVAT");
#[allow(dead_code)]
pub const ROLE_PROCESSOR: Symbol = symbol_short!("PROCESS");
#[allow(dead_code)]
pub const ROLE_DISPENSARY: Symbol = symbol_short!("DISPENS");
#[allow(dead_code)]
pub const ROLE_CONSUMER: Symbol = symbol_short!("CONSUMER");

pub fn get_role_key(role: Symbol) -> Symbol {
    role
}

