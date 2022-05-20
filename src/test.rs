use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault, env, init};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Test {
    pub number: i16,
}

#[near_bindgen]
impl Test {

    #[init(ignore_state)]
    pub fn test(){
        let log_message = " hello";
        env::log(log_message.as_bytes());
    }
}