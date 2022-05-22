use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::env::log;
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

    pub fn get_add_farm_date_by_ac_id(&self, ac_id: &AccountId) -> u64 {
        match self.wnear_eth.add_farm_date.get(&ac_id) {
            Some(s) => s,
            None => env::block_timestamp(),
        }
    }

    pub fn get_farm_duration(&self, start_time: u64) -> u128 {
        let farm_duration = (env::block_timestamp() - start_time) as u128;
        farm_duration
    }

    pub fn get_share_of_total_supply_percent(&self, balance: u128) -> u128 {
        let share = (balance * u128::pow(10, 3)) / self.wnear_eth.pool_amount;
        log("share".as_bytes());
        log(share.to_string().as_bytes());
        share
    }
    fn calculate_interest_percent(&self, ac_id: &AccountId, balance: u128) -> u128 {
        let interest = self.get_share_of_total_supply_percent(balance) / 4;
        log("interest".as_bytes());
        log((interest/10).to_string().as_bytes());
        interest
    }

    pub fn calculate_reward(&self, ac_id: &AccountId, balance: u128) -> u128 {
        let reward = ((self.calculate_interest_percent(ac_id, balance) * u128::pow(10, 24)
            * self.get_farm_duration(self.get_add_farm_date_by_ac_id(&ac_id)))
        / (3154 * u128::pow(10, 13))) * balance
        / u128::pow(10, 24) * u128::pow(10, 3);
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
        env::log("balance before".as_bytes());
        env::log((balance).to_string().as_bytes());
        if balance > 0 {
            balance = balance + self.calculate_reward(&account_id, balance);
        }
        env::log("balance after".as_bytes());
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

    pub fn get_farm(self, account_id: AccountId) -> Option<u128> {
        self.wnear_eth.farmer_balance_list.get(&account_id)
    }

    pub fn withdraw_farm(&mut self, account_id: AccountId) -> u128 {
        let balance = match self.wnear_eth.farmer_balance_list.get(&account_id) {
            Some(s) => s,
            None => 0,
        };
        self.wnear_eth.pool_amount -= balance;
        self.wnear_eth.add_farm_date.remove(&account_id);
        self.wnear_eth.farmer_balance_list.remove(&account_id);
        balance + self.calculate_reward(&account_id, balance)
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
