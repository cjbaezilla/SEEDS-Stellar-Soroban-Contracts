use soroban_sdk::{Address, String, contracttype};

#[contracttype]
#[derive(Clone)]
pub struct SeedData {
    pub id: String,
    pub created_at: u64,
    pub creator: Address,
    pub variety: String,
    pub batch: String,
    pub origin_country: String,
    pub seed_bank: String,
    pub expected_thc: Option<u32>,
    pub expected_cbd: Option<u32>,
    pub organic_certified: bool,
    pub nft_id: u128,
    pub nft_contract: Address,
}

