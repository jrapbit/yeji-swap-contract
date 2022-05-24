
use near_sdk::{near_bindgen, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Pool {
    pub pool_name: String,
    pub token: u128,
    pub amount_token0: u128,
    pub amount_token1: u128,
    pub constant_k: u128,
}