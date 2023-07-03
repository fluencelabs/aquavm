#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

use risc0_zkvm::guest::env;
use air::execute_air;
use interface::AquaVMParameters;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let parameters: AquaVMParameters = env::read();

    let result = execute_air(parameters.air, parameters.prev_data, parameters.data,  parameters.params, parameters.call_results);
    env::commit(&result);
}
