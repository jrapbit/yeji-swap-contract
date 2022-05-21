use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, PanicOnDefault, AccountId};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Farm {
    pub pool_name: String,
    pub farmer_balance_list: UnorderedMap<AccountId, u128>,
    pub add_farm_date: UnorderedMap<AccountId, u64>,
    pub pool_amount: u128,
}

