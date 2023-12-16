#![no_main]

use air::execute_air;
use risc0_zkvm::guest::env;
use zk_aquavm_interface::AquaVMProvingParameters;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let parameters: AquaVMProvingParameters = env::read();

    let result = execute_air(
        parameters.air,
        parameters.prev_data,
        parameters.current_data,
        parameters.run_params,
        parameters.call_results.into(),
    );
    env::commit(&result);
}
