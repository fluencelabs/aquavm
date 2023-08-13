use air_interpreter_interface::InterpreterOutcome;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

use air::execute_air;
use air::RunParameters;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Aqua {}

#[near_bindgen]
impl Aqua {
    #[result_serializer(borsh)]
    pub fn execute_script(
        &self,
        #[serializer(borsh)]
        air_script: String,
        #[serializer(borsh)]
        prev_data: String,
        #[serializer(borsh)]
        current_data: String,
        #[serializer(borsh)]
        run_parameters: String,
        #[serializer(borsh)]
        call_results: String,
    ) -> String {
        let outcome = Self::execute(
            air_script,
            prev_data.into(),
            current_data.into(),
            run_parameters.into(),
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
