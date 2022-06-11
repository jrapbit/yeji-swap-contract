
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ext_contract, near_bindgen, AccountId, Promise, PanicOnDefault};


#[ext_contract(ext_ft)]
trait FungibleToken {
    // change methods
    fn ft_transfer(&mut self, receiver_id: String, amount: String, memo: Option<String>);
    fn ft_transfer_call(&mut self, receiver_id: String, amount: String, memo: Option<String>, msg: String) -> U128;

    // view methods
    fn ft_total_supply(&self) -> String;
    fn ft_balance_of(&self, account_id: String) -> String;

}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub account_id: AccountId
}

#[near_bindgen]
impl Contract {
    
    #[payable]
    pub fn ext_balance(account_id: AccountId, contract_name: String) -> Promise {

        let contract_acc = if contract_name == "near" {
            "wrap.testnet"
        } else if contract_name == "eth" {
            "eth.fakes.testnet"
        } else {
            ""
        };
        ext_ft::ft_balance_of(
            account_id, // ft_balance_of takes an account_id as a parameter
            &contract_acc, // contract account id
            0, // yocto NEAR to attach
            5_000_000_000_000 // gas to attach
        )
    }
    
    #[payable]
    pub fn ext_tranfer(account_id: &AccountId, amount: String, contract_name: String) -> Promise {
        let contract_acc = if contract_name == "near" {
            "wrap.testnet"

        } else if contract_name == "eth" {
            "eth.fakes.testnet"
        } else {
            ""
        };
        // near * 10^14
        // eth * 10^8
        ext_ft::ft_transfer(
            account_id.to_string(),
            amount,
            None, // ft_balance_of takes an account_id as a parameter
            &contract_acc, // contract account id
            1, // yocto NEAR to attach
            5_000_000_000_000 // gas to attach
        )
    }

    #[payable]
    pub fn ext_total_suply(contract_name: String) -> Promise {
        let contract_acc = if contract_name == "near" {
            "wrap.testnet"
        } else if contract_name == "eth" {
            "eth.fakes.testnet"
        } else {
            ""
        };
        ext_ft::ft_total_supply(
             // ft_balance_of takes an account_id as a parameter
            &contract_acc, // contract account id
            0, // yocto NEAR to attach
            5_000_000_000_000 // gas to attach
        )
    }
}
