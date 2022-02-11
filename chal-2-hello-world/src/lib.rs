use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct HelloWorld {}

#[near_bindgen]
impl HelloWorld {
    pub fn say_hello(self) -> String {
        String::from("Hello, World!")
    }
}