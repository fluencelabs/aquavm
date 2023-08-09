use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Aqua {}

impl Default for Aqua {
    fn default() -> Self {
        Self {}
    }
}

#[near_bindgen]
impl Aqua {
    pub fn example(
        &mut self,
        a: u8, b: u8
    ) -> u8 {
        a.wrapping_add(b)
    }
}
