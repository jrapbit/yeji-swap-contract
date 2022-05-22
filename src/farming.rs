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
                farmer_balance_list: UnorderedMap::new(b"a".to_vec()),
                add_farm_date: UnorderedMap::new(b"b".to_vec()),
                pool_amount: 0,
            },
            wnear_wbtc: Farm {
                pool_name: "wnear_wbtc".to_string(),
                farmer_balance_list: UnorderedMap::new(b"c".to_vec()),
                add_farm_date: UnorderedMap::new(b"d".to_vec()),
                pool_amount: 0,
            },
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

    pub fn get_farm_duration_second(&self, start_time: u64) -> u128 {
        let farm_second = (env::block_timestamp() - start_time) as u128 / 10^9;
        env::log("farm_second".as_bytes());
        env::log(farm_second.to_string().as_bytes());
        farm_second
    }

    pub fn get_share_of_total_supply(&self, balance: u128) -> u128 {
        let pool_share = (balance) / self.wnear_eth.pool_amount;
        env::log("pool_share".as_bytes());
        env::log(pool_share.to_string().as_bytes());
        pool_share
    }

    pub fn calculate_reward(&self, ac_id: &AccountId, balance: u128) -> u128 {
        let reward = ((self.get_share_of_total_supply(balance) / 4)
            * self.get_farm_duration_second(self.get_add_farm_date_by_ac_id(&ac_id)))
        / 32 * 10^9;
        env::log("reward".as_bytes());
        env::log(reward.to_string().as_bytes());
        reward

    }

    //--------------------------------------------------------------------------------

    pub fn add_farm(&mut self, amount: u128, account_id: AccountId) {
        let mut balance = match self.wnear_eth.farmer_balance_list.get(&account_id) {
            Some(s) => s,
            None => 0,
        };
        env::log("balance before rewarded".as_bytes());
        env::log((balance).to_string().as_bytes());
        if balance > 0 {
            balance = balance * self.calculate_reward(&account_id, balance);
        }
        env::log("balance after rewarded".as_bytes());
        env::log((balance).to_string().as_bytes());
        //add total amount of pool
        self.wnear_eth.pool_amount += amount;
        self.wnear_eth
            .add_farm_date
            .insert(&account_id, &env::block_timestamp());
        self.wnear_eth
            .farmer_balance_list
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
