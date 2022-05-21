use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

use crate::farm::Farm;



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Farming {
    pub wnear_eth: Farm,
    pub wnear_wbtc: Farm,
}

#[near_bindgen]
impl Farming {
     #[init]
    pub fn new() -> Self {
        Self {
            wnear_eth: Farm { 
                pool_name: "wnear_eth".to_string(),
                farmer_balance_list: UnorderedMap::new(b"s".to_vec()),
                add_farm_date: UnorderedMap::new(b"s".to_vec()),
                pool_amount: 0 },
            wnear_wbtc: Farm { 
                pool_name: "wnear_eth".to_string(),
                farmer_balance_list: UnorderedMap::new(b"s".to_vec()),
                add_farm_date: UnorderedMap::new(b"s".to_vec()),
                pool_amount: 0 }, 
        }
    }
    // pub fn new(name: String) -> Self {
    //     Self {
    //         pool_name: name.to_string(),
    //         farmer_balance_list: UnorderedMap::new(b"s".to_vec()),
    //         add_farm_date: UnorderedMap::new(b"s".to_vec()),
    //         pool_amount: 0,
    //     }
    // }

    pub fn get_add_farm_date_by_ac_id(&self, ac_id: &AccountId) -> u64 {
        match self.wnear_eth.add_farm_date.get(&ac_id) {
            Some(s) => s,
            None => env::block_timestamp(),
        }
    }

    pub fn get_farm_duration_year(&self, start_time: u64) -> u128 {
        (env::block_timestamp() - start_time) as u128 / 3154 * 10 ^ 13
    }

    pub fn get_share_of_total_supply(&self, balance: u128) -> u128 {
        (balance * self.wnear_eth.pool_amount) / 100
    }

    pub fn calculate_reward(&self, ac_id: &AccountId, balance: u128) -> u128 {
        (self.get_share_of_total_supply(balance) / 4)
            * self.get_farm_duration_year(self.get_add_farm_date_by_ac_id(&ac_id))
    }

    //--------------------------------------------------------------------------------

    pub fn add_farm(&mut self, amount: u128, account_id: AccountId) {
        let mut balance = match self.wnear_eth.farmer_balance_list.get(&account_id) {
            Some(s) => s,
            None => 0,
        };
        if balance > 0 {
            balance = balance * self.calculate_reward(&account_id, balance);
        }
        //add total amount of pool
        self.wnear_eth.pool_amount += amount;
        self.wnear_eth.add_farm_date
            .insert(&account_id, &env::block_timestamp());
        self.wnear_eth.farmer_balance_list
            .insert(&account_id, &(amount + balance));
    }

    pub fn get_farm(&self, account_id: AccountId) -> Option<u128> {
        self.wnear_eth.farmer_balance_list.get(&account_id)
    }

    pub fn withdraw_farm(&self, account_id: AccountId) {
        //
    }

    pub fn log_all_farmer_balance(&self) {
        env::log("wnear-eth".as_bytes());
        for (k, x) in self.wnear_eth.farmer_balance_list.iter() {
            env::log(format!("Account={k}, Balance={x}", k = k, x = x).as_bytes());
        }
        env::log("wnear-wbtc".as_bytes());
        for (k, x) in self.wnear_wbtc.farmer_balance_list.iter() {
            env::log(format!("Account={k}, Balance={x}", k = k, x = x).as_bytes());
        }
    }
}
