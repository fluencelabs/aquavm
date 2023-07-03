// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use methods::{METHOD_NAME_ELF, METHOD_NAME_ID};
use interface::AquaVMParameters;
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Executor, ExecutorEnv,
};

use serde::Serialize;
use serde::Deserialize;

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

const DEFAULT_DATA: &str = "";

pub type CallResults = HashMap<u32, CallServiceResult>;

#[derive(Serialize, Deserialize)]
pub struct CallServiceResult {
    pub ret_code: i32,
    pub result: serde_json::Value,
}

pub(crate) struct ExecutionData<'ctx> {
    pub(crate) air_script: String,
    pub(crate) current_data: String,
    pub(crate) prev_data: String,
    pub(crate) particle: ParticleParameters<'ctx>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleParameters<'ctx> {
    pub init_peer_id: Cow<'ctx, str>,
    pub particle_id: Cow<'ctx, str>,
    pub timestamp: u64,
    pub ttl: u32,
    pub current_peer_id: Cow<'ctx, str>,
}

impl<'ctx> ParticleParameters<'ctx> {
    pub fn new(
        init_peer_id: Cow<'ctx, str>,
        particle_id: Cow<'ctx, str>,
        timestamp: u64,
        ttl: u32,
        current_peer_id: Cow<'ctx, str>,
    ) -> Self {
        Self {
            init_peer_id,
            particle_id,
            timestamp,
            ttl,
            current_peer_id,
        }
    }
}

pub(crate) fn load<'a>() -> ExecutionData<'a> {
    let air_script =
        read_air_script(Some(Path::new("/Users/mike/dev/work/fluence/wasm/aquavm/benches/performance_metering/dashboard/script.air"))).unwrap();
    let prev_data = load_data(Path::new("/Users/mike/dev/work/fluence/wasm/aquavm/benches/performance_metering/dashboard/prev_data.json")).unwrap();
    let current_data = load_data(Path::new("/Users/mike/dev/work/fluence/wasm/aquavm/benches/performance_metering/dashboard/cur_data.json")).unwrap();

    let timestamp = 0;
    let ttl = u32::MAX;
    let init_peer_id = "";
    let current_peer_id = "";

    let particle = ParticleParameters::new(
        init_peer_id.into(),
        "".into(),
        timestamp,
        ttl,
        current_peer_id.into(),
    );

    ExecutionData {
        air_script,
        prev_data,
        current_data,
        particle,
    }
}

fn read_air_script(air_input: Option<&Path>) -> anyhow::Result<String> {
    use std::io::Read;

    let air_script = match air_input {
        Some(in_path) => std::fs::read_to_string(in_path)?,
        None => {
            let mut buffer = String::new();
            let mut stdin = std::io::stdin().lock();

            stdin.read_to_string(&mut buffer)?;
            buffer
        }
    };

    Ok(air_script)
}

fn read_call_results(call_results_path: Option<&Path>) -> CallResults {
    match call_results_path {
        None => CallResults::default(),
        Some(call_results_path) => {
            let call_results_json =
                load_data(call_results_path).unwrap();
            serde_json::from_str(&call_results_json).unwrap()
        }
    }
}

fn load_data(data_path: &Path) -> anyhow::Result<String> {
    Ok(std::fs::read_to_string(data_path)?)
}

fn main() {
    let data = load();
    let call_result = read_call_results(None);

    let mut run_parameters = AquaVMParameters::default();
    run_parameters.air = data.air_script;
    run_parameters.prev_data = data.prev_data.into();
    run_parameters.data = data.current_data.into();
    run_parameters.call_results = serde_json::to_vec(&call_result).unwrap();

    // First, we construct an executor environment
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&run_parameters).unwrap())
        .build();

    // TODO: add guest input to the executor environment using
    // ExecutorEnvBuilder::add_input().
    // To access this method, you'll need to use the alternate construction
    // ExecutorEnv::builder(), which creates an ExecutorEnvBuilder. When you're
    // done adding input, call ExecutorEnvBuilder::build().

    // For example: let env = ExecutorEnv::builder().add_input(&vec).build();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = Executor::from_elf(env, METHOD_NAME_ELF).unwrap();

    println!("before session");
    let time = Instant::now();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    let prove_duration = time.elapsed();
    println!("prove time is {:?}", prove_duration);

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    let time = Instant::now();

    receipt.verify(METHOD_NAME_ID).unwrap();

    let verify_duration = time.elapsed();
    println!("verify duration time is {:?}", verify_duration);
}
