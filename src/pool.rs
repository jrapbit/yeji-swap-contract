
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Pool {
    pub wnear_eth_wnear: u128,
    pub wnear_eth_eth: u128,
}

#[near_bindgen]
impl Pool {
    #[init]
    pub fn start() -> Self {
        Self {
                wnear_eth_eth: 0,
                wnear_eth_wnear: 0,
        }
    }
}

