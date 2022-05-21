//! This contract implements simple counter backed by storage on blockchain.
//!
//! The contract provides methods to [increment] / [decrement] counter and
//! [get it's current value][get_num] or [reset].
//!
//! [increment]: struct.Counter.html#method.increment
//! [decrement]: struct.Counter.html#method.decrement
//! [get_num]: struct.Counter.html#method.get_num
//! [reset]: struct.Counter.html#method.reset

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{serde_json::json, env, ext_contract, near_bindgen, AccountId, json_types::U128, Promise, log, PanicOnDefault, json_types::ValidAccountId, Balance, Gas};


#[ext_contract(ext_ft)]
trait FungibleToken {
    // change methods
    fn ft_transfer(&mut self, receiver_id: String, amount: String, memo: Option<String>);
    fn ft_transfer_call(&mut self, receiver_id: String, amount: String, memo: Option<String>, msg: String) -> U128;

    // view methods
    fn ft_total_supply(&self) -> String;
    fn ft_balance_of(&self, account_id: String) -> String;

}

// use std::io;
// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub account_id: AccountId
}

#[near_bindgen]
impl Contract {
    #[payable]

    pub fn ext_balance(account_id: AccountId) -> Promise {
        ext_ft::ft_balance_of(
            account_id, // ft_balance_of takes an account_id as a parameter
            &"wrap.testnet", // contract account id
            0, // yocto NEAR to attach
            5_000_000_000_000 // gas to attach
        )
    }
    
    #[payable]
    pub fn ext_tranfer(account_id: AccountId, amount: String) -> Promise {
        ext_ft::ft_transfer(
            account_id,
            amount,
            None, // ft_balance_of takes an account_id as a parameter
            &"wrap.testnet", // contract account id
            env::attached_deposit(), // yocto NEAR to attach
            5_000_000_000_000 // gas to attach
        )
    }

    #[payable]
    pub fn ext_total_suply() -> Promise {
        ext_ft::ft_total_supply(
             // ft_balance_of takes an account_id as a parameter
            &"wrap.testnet", // contract account id
            0, // yocto NEAR to attach
            5_000_000_000_000 // gas to attach
        )
    }
}

// --------------------------------------------------------------------
//near view wrap.testnet ft_balance_of '{"account_id": "biotest03.testnet"}'

// #[near_bindgen]
// #[derive(Default, BorshDeserialize, BorshSerialize)]
// pub struct PromiseA {}

// pub const PROMIS_A_ACCOUNT_ID: &str = "wrap.testnet";

// const NO_DEPOSIT: Balance = 0;

// const BASIC_GAS: Gas = 5_000_000_000_000;

// #[near_bindgen]
// impl PromiseA {
//     pub fn call_b(&mut self, account_id: ValidAccountId) {
//         log!("Calling B at @{} from A @{}", account_id.as_ref(), env::current_account_id());
//         let account_id: AccountId = account_id.into();
//         Promise::new(account_id.clone()).function_call(
//             b"ft_balance_of".to_vec(), 
//             format!("{{\"account__id\": \"biotest03.testnet\"}}").into_bytes(),
//             NO_DEPOSIT, 
//             BASIC_GAS
//         );

//     }
// }

// --------------------------------------------------------------------
// pub fn my_method() -> Promise {
//     // Create a new promise, which will create a new (empty) ActionReceipt
//     // Internally this will use env:promise_batch_create
//     let cross_contract_call = Promise::new(
//         "wrap.testnet".to_string(), // the recipient of this ActionReceipt (contract account id)
//     )
//     // attach a function call action to the ActionReceipt
//     .function_call(
//         b"ft_balance_of".to_vec(), // the function call will invoke the ft_balance_of method on the wrap.testnet
//         json!({ "account_id": "bioleta.testnet".to_string() }) // method arguments
//             .to_string()
//             .into_bytes(),
//         0,                 // amount of yoctoNEAR to attach
//         5_000_000_000_000, // gas to attach
//     );

//     // Create another promise, which will create another (empty) ActionReceipt.
//     let callback = Promise::new(
//         env::current_account_id(), // the recipient of this ActionReceipt (&self)
//     )
//     .function_call(
//         b"my_callback".to_vec(), // the function call will be a callback function
//         b"{}".to_vec(),          // method arguments
//         0,                       // amount of yoctoNEAR to attach
//         5_000_000_000_000,       // gas to attach
//     );

//     // Make the callback ActionReceipt dependent on the cross_contract_call ActionReceipt
//     // callback will now remain postponed until cross_contract_call finishes
//     cross_contract_call.then(callback)
// }
// use near_sdk::json::{U128};
// use near_sdk::{ext_contract};



