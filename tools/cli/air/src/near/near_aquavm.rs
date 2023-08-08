use air_interpreter_interface::InterpreterOutcome;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

use air::execute_air;
use air::RunParameters;

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
    pub fn execute_script(
        &self,
        air: String,
        prev_data: String,
        cur_data: String,
        params: String,
        call_results: String,
    ) -> String {
        let outcome = Self::execute(
            air,
            prev_data.into(),
            cur_data.into(),
            params.into(),
            call_results.into(),
        );
        serde_json::to_string(&outcome).unwrap()
    }

    // private method
    fn execute(
        air: String,
        prev_data: Vec<u8>,
        cur_data: Vec<u8>,
        params: Vec<u8>,
        call_results: Vec<u8>,
    ) -> InterpreterOutcome {
        let params: RunParameters =
            serde_json::from_slice(&params).expect("cannot parse RunParameters");

        execute_air(air, prev_data, cur_data, params, call_results)
    }
}
