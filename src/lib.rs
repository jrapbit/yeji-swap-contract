use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::collections::UnorderedMap;

mod test;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub farm_list_near: UnorderedMap<AccountId, u128>,
}


#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            farm_list_near: UnorderedMap::new(b"s".to_vec()),
        }
    }

    pub fn add_farm(&mut self, amount: u128) {
        self.farm_list_near.insert(&env::predecessor_account_id(), &amount);

    }

    pub fn delete_farm(&mut self) {
        self.farm_list_near.remove(&env::predecessor_account_id());
    }

    pub fn get_farm2(&self) -> Option<u128> {
        self.farm_list_near.get(&env::predecessor_account_id())
    }

    pub fn get_farm(&self, account_id: AccountId) -> Option<u128> {
        self.farm_list_near.get(&account_id)
    }
}
