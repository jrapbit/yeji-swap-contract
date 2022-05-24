use std::cmp;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::env::log;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use num::integer::Roots;

use crate::farm::Farm;
use crate::pool::Pool;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Farming {
    pub wnear_eth_farm: Farm,
    pub wnear_wbtc_farm: Farm,
    pub wnear_eth_pool: Pool,
    pub wnear_wbtc_pool: Pool,
    pub minimum_liquidity: u128,
}

#[near_bindgen]
impl Farming {
    #[init]
    pub fn new() -> Self {
        Self {
            wnear_eth_farm: Farm {
                pool_name: "wnear_eth".to_string(),
                farmer_balance_list: UnorderedMap::new(b"a".to_vec()),
                add_farm_date: UnorderedMap::new(b"b".to_vec()),
                pool_amount: 0,
            },
            wnear_wbtc_farm: Farm {
                pool_name: "wnear_wbtc".to_string(),
                farmer_balance_list: UnorderedMap::new(b"c".to_vec()),
                add_farm_date: UnorderedMap::new(b"d".to_vec()),
                pool_amount: 0,
            },
            wnear_eth_pool: Pool {
                pool_name: "wnear_eth_pool".to_string(),
                token: 0,
                amount_token0: 0,
                amount_token1: 0,
                constant_k: 0,
            },
            wnear_wbtc_pool: Pool {
                pool_name: "wnear_wbtc_pool".to_string(),
                token: 0,
                amount_token0: 0,
                amount_token1: 0,
                constant_k: 0,
            },
            minimum_liquidity: 1000,
        }
    }

    pub fn get_add_farm_date_by_ac_id(&self, ac_id: &AccountId) -> u64 {
        match self.wnear_eth_farm.add_farm_date.get(&ac_id) {
            Some(s) => s,
            None => env::block_timestamp(),
        }
    }

    pub fn get_farm_duration(&self, start_time: u64) -> u128 {
        let farm_duration = (env::block_timestamp() - start_time) as u128;
        log("farm duration".as_bytes());
        log(farm_duration.to_string().as_bytes());
        farm_duration
    }

    pub fn get_share_of_total_supply_percent(&self, balance: u128) -> u128 {
        let share = (balance * u128::pow(10, 3)) / self.wnear_eth_farm.pool_amount;
        log("share".as_bytes());
        log(share.to_string().as_bytes());
        share
    }
    fn calculate_interest_percent(&self, ac_id: &AccountId, balance: u128) -> u128 {
        let interest = self.get_share_of_total_supply_percent(balance) / 4;
        log("interest".as_bytes());
        log((interest / 10).to_string().as_bytes());
        interest
    }

    pub fn calculate_reward(&self, ac_id: &AccountId, balance: u128) -> u128 {
        let reward = ((self.calculate_interest_percent(ac_id, balance)
            * u128::pow(10, 24)
            * self.get_farm_duration(self.get_add_farm_date_by_ac_id(&ac_id)))
            / (3154 * u128::pow(10, 13)))
            * balance
            / u128::pow(10, 24)
            * u128::pow(10, 3);
        env::log("reward".as_bytes());
        env::log(reward.to_string().as_bytes());
        reward
    }

    //--------------------------------------------------------------------------------
    pub fn get_eth_price_in_wnear(&self) -> u128 {
        self.wnear_eth_pool.amount_token0 / self.wnear_eth_pool.constant_k
    }

    pub fn get_near_price_in_eth(&self) -> u128 {
        self.wnear_eth_pool.amount_token1 / self.wnear_eth_pool.constant_k
    }

    pub fn add_lp_wnear_eth_token(&mut self, amount_wnear: u128, amount_eth: u128) -> u128 {
        if self.wnear_eth_pool.token == 0 {
            //liquidity = Math.sqrt(amount0.mul(amount1)).sub(MINIMUM_LIQUIDITY);
            let liquidity = (amount_eth * amount_wnear).sqrt();
            self.wnear_eth_pool.amount_token0 += amount_wnear;
            self.wnear_eth_pool.amount_token1 += amount_eth;
            self.wnear_eth_pool.token += liquidity;
            self.wnear_eth_pool.constant_k =
                self.wnear_eth_pool.amount_token0 * self.wnear_eth_pool.amount_token1;
            liquidity
        } else {
            //liquidity = Math.min(amount0.mul(_totalSupply) / _reserve0, amount1.mul(_totalSupply) / _reserve1);
            let liquidity = cmp::min(
                (amount_wnear * self.wnear_eth_pool.token) / self.wnear_eth_pool.amount_token0,
                (amount_eth * self.wnear_eth_pool.token) / self.wnear_eth_pool.amount_token1,
            ) * 2;
            self.wnear_eth_pool.amount_token0 += amount_wnear;
            self.wnear_eth_pool.amount_token1 += amount_eth;
            self.wnear_eth_pool.token += liquidity;
            self.wnear_eth_pool.constant_k =
                self.wnear_eth_pool.amount_token0 * self.wnear_eth_pool.amount_token1;
            liquidity
        }
    }

    pub fn process_farming(
        &mut self,
        account_id: AccountId,
        amount_wnear: u128,
        amount_eth: u128,
    ) {
        let liquidity = self.add_lp_wnear_eth_token(amount_wnear, amount_eth);
        self.add_farm(liquidity, account_id)
    }

    pub fn add_farm(&mut self, amount: u128, account_id: AccountId) {
        let mut balance = match self.wnear_eth_farm.farmer_balance_list.get(&account_id) {
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
        self.wnear_eth_farm.pool_amount += amount;
        self.wnear_eth_farm
            .add_farm_date
            .insert(&account_id, &env::block_timestamp());
        self.wnear_eth_farm
            .farmer_balance_list
            .insert(&account_id, &(amount + balance));
    }

    pub fn get_farm(self, account_id: AccountId) -> Option<u128> {
        self.wnear_eth_farm.farmer_balance_list.get(&account_id)
    }

    pub fn withdraw_farm_to_wnear_eth(&mut self, account_id: AccountId) -> (u128, u128) {
        let mut balance = match self.wnear_eth_farm.farmer_balance_list.get(&account_id) {
            Some(s) => s,
            None => 0,
        };
        self.wnear_eth_farm.add_farm_date.remove(&account_id);
        self.wnear_eth_farm.farmer_balance_list.remove(&account_id);
        balance = balance + self.calculate_reward(&account_id, balance);
        // amount0 = liquidity.mul(balance0) / _totalSupply;
        let wnear = (balance * self.wnear_eth_pool.amount_token0) / self.wnear_eth_pool.token;
        // amount1 = liquidity.mul(balance1) / _totalSupply;
        let eth = (balance * self.wnear_eth_pool.amount_token1) / self.wnear_eth_pool.token;
        self.wnear_eth_farm.pool_amount -= balance;
        self.wnear_eth_pool.token -= balance;
        (wnear, eth)
    }

    pub fn log_all_farmer_balance(&self) {
        env::log("wnear-eth".as_bytes());
        for (k, x) in self.wnear_eth_farm.farmer_balance_list.iter() {
            env::log(format!("Account={k}, Balance={x}", k = k, x = x).as_bytes());
        }
        env::log("wnear-wbtc".as_bytes());
        for (k, x) in self.wnear_wbtc_farm.farmer_balance_list.iter() {
            env::log(format!("Account={k}, Balance={x}", k = k, x = x).as_bytes());
        }
    }
}
