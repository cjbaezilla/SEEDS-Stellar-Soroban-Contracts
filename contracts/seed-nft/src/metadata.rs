use soroban_sdk::{Address, String, contracttype, Vec};
use crate::lifecycle::LifecycleState;

#[contracttype]
#[derive(Clone)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

#[contracttype]
#[derive(Clone)]
pub struct OpenSeaMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub external_url: Option<String>,
    pub attributes: Option<Vec<Attribute>>,
}

#[contracttype]
#[derive(Clone)]
pub struct SeedMetadata {
    pub state: LifecycleState,
    pub location: Option<String>,
    pub temperature: Option<i32>,
    pub humidity: Option<u32>,
    pub lab_analysis: Option<String>,
    pub processor: Option<Address>,
    pub distributor: Option<Address>,
    pub consumer: Option<Address>,
    pub updated_at: u64,
    pub name: String,
    pub description: String,
    pub image: String,
    pub external_url: Option<String>,
    pub attributes: Vec<Attribute>,
}

