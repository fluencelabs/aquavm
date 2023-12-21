use air_interpreter_data::InterpreterDataEnvelopeFormat;
use air_interpreter_sede::Format;
use air_interpreter_sede::RmpSerdeFormat;
use air_interpreter_sede::RmpSerdeMultiformat;
use air_interpreter_sede::SerdeJsonFormat;
use air_interpreter_sede::SerdeJsonMultiformat;
use air_test_framework::*;
use air_test_utils::key_utils::derive_dummy_keypair;
use air_test_utils::prelude::*;

use clap::Parser;
use clap::Subcommand;
use itertools::Itertools as _;
use maplit::hashmap;

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

const PARTICLE_ID: &str = "0123456789ABCDEF";
const MAX_STREAM_SIZE: usize = 1023;
const MB: usize = 1024 * 1024;

mod calls;
mod cid_benchmarking;
mod dashboard;
mod data;
mod network_explore;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    bench: Bench,

    #[arg(long)]
    dest_dir: PathBuf,
}

#[derive(Debug, Subcommand)]
enum Bench {
    Dashboard,
    MultipleCids10,
    MultipleCids50,
    MultiplePeers5,
    MultiplePeers8,
    MultiplePeers25,
    MultipleSigs10,
    MultipleSigs30,
    MultipleSigs200,
    NetworkExplore,
    CanonMapKeyByLens,
    CanonMapKeyElementByLens,
    PopulateMapMultipleKeys,
    PopulateMapSingleKey,
    CanonMapMultipleKeys,
    CanonMapSingleKey,
    CanonMapScalarMultipleKeys,
    CanonMapScalarSingleKey,
    LongData,
    BigValuesData,
    CallRequests500,
    CallResults500,
    #[command(name = "parser-10000-100")]
    Parser10000_100,
    #[command(name = "parser-calls-10000-100")]
    ParserCalls10000_100,
    Null,
    #[command(name = "lense-100mb")]
    Lense100MB,
    #[command(name = "map-100mb")]
    Map100MB,
    #[command(name = "canon-map-100mb")]
    CanonMap100MB,
    #[command(name = "call-result-100mb")]
    CallResult100MB,
    #[command(name = "parser-air-100mb")]
    ParserAir100MB,
}

fn main() {
    let args = Cli::parse();

    let data = match args.bench {
        Bench::MultipleCids10 => multiple_cids(10),
        Bench::MultipleCids50 => multiple_cids(50),
        Bench::MultiplePeers5 => multiple_peers(5),
        Bench::MultiplePeers8 => multiple_peers(8),
        Bench::MultiplePeers25 => multiple_peers(25),
        Bench::MultipleSigs10 => multiple_sigs(10),
        Bench::MultipleSigs30 => multiple_sigs(30),
        Bench::MultipleSigs200 => multiple_sigs(200),
        Bench::Dashboard => dashboard::dashboard(),
        Bench::NetworkExplore => network_explore::network_explore(),
        Bench::PopulateMapMultipleKeys => populate_map_multiple_keys(MAX_STREAM_SIZE),
        Bench::PopulateMapSingleKey => populate_map_single_key(770),
        Bench::CanonMapMultipleKeys => canon_map_multiple_keys(MAX_STREAM_SIZE),
        Bench::CanonMapSingleKey => canon_map_single_key(770),
        Bench::CanonMapScalarMultipleKeys => canon_map_scalar_multiple_keys(MAX_STREAM_SIZE),
        Bench::CanonMapScalarSingleKey => canon_map_scalar_single_key(770),
        Bench::CanonMapKeyByLens => canon_map_key_by_lens(770),
        Bench::CanonMapKeyElementByLens => canon_map_key_element_by_lens(770),
        Bench::LongData => long_data(),
        Bench::BigValuesData => big_values_data(),
        Bench::CallRequests500 => calls::call_requests(500),
        Bench::CallResults500 => calls::call_results(500),
        Bench::Parser10000_100 => parser_10000_100(),
        Bench::ParserCalls10000_100 => parser_calls(10000, 100),
        Bench::Null => null(),
        Bench::CallResult100MB => mem_consumption_with_size_in_mb(100),
        Bench::Lense100MB => mem_consumption_w_lense_with_size_in_mb(100),
        Bench::Map100MB => mem_consumption_w_map_2_scalar_with_size_in_mb(100),
        Bench::CanonMap100MB => mem_consumption_w_canon_map_with_size_in_mb(100),
        Bench::ParserAir100MB => mem_consumption_air_100mb(280000, 10),
    };

    save_data(&args.dest_dir, data).unwrap();
}

fn save_data(dest_dir: &Path, data: Data) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::*;

    create_dir_all(dest_dir)?;

    save_file(dest_dir, "script.air", &data.air)?;
    save_file(
        dest_dir,
        "prev_data.json",
        reformat_json_if_possible::<InterpreterDataEnvelopeFormat>(&data.prev_data),
    )?;
    save_file(
        dest_dir,
        "cur_data.json",
        reformat_json_if_possible::<InterpreterDataEnvelopeFormat>(&data.cur_data),
    )?;
    save_file(
        dest_dir,
        "params.json",
        serde_json::to_vec_pretty(&data.params_json)?,
    )?;
    save_file(dest_dir, "keypair.ed25519", &data.keypair)?;

    if let Some(call_results) = data.call_results {
        save_file(
            dest_dir,
            "call_results.json",
            // these call results are intended for manual generation too for the AIR CLI, so
            // simplier representation from avm_interface::CallResults is used, and JSON is used explicitely
            reformat_json_if_possible::<SerdeJsonFormat>(
                &serde_json::to_vec(&call_results).unwrap(),
            ),
        )
        .unwrap();
    }

    Ok(())
}

trait Reformatter: Format<()> {
    fn reformat(data: &[u8]) -> Cow<'_, [u8]>;
}

impl Reformatter for SerdeJsonFormat {
    /// make zero-indentation data for more convenient git diffs
    fn reformat(data: &[u8]) -> Cow<'_, [u8]> {
        use serde::ser::Serialize;

        let obj: serde_json::Value = serde_json::from_slice(data).unwrap();

        let fmt = serde_json::ser::PrettyFormatter::with_indent(&[]);
        let mut out = vec![];
        {
            let mut ser = serde_json::ser::Serializer::with_formatter(&mut out, fmt);
            obj.serialize(&mut ser).unwrap();
        }

        out.into()
    }
}

impl Reformatter for SerdeJsonMultiformat {
    /// make zero-indentation data for more convenient git diffs
    fn reformat(data: &[u8]) -> Cow<'_, [u8]> {
        data.into()
    }
}

impl Reformatter for RmpSerdeFormat {
    fn reformat(data: &[u8]) -> Cow<'_, [u8]> {
        data.into()
    }
}

impl Reformatter for RmpSerdeMultiformat {
    fn reformat(data: &[u8]) -> Cow<'_, [u8]> {
        data.into()
    }
}

/// make zero-indentation data for better git diffs
fn reformat_json_if_possible<R: Reformatter>(data: &[u8]) -> Cow<'_, [u8]> {
    if data.is_empty() {
        return data.into();
    }

    R::reformat(data)
}

fn save_file(dest_dir: &Path, filename: &str, data: impl AsRef<[u8]>) -> std::io::Result<()> {
    let mut dest_dir = dest_dir.to_owned();
    dest_dir.push(filename);

    std::fs::write(dest_dir, data)
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub(crate) struct Data {
    pub(crate) air: String,
    pub(crate) prev_data: Vec<u8>,
    pub(crate) cur_data: Vec<u8>,
    pub(crate) params_json: HashMap<String, String>,
    pub(crate) call_results: Option<CallResults>,
    pub(crate) keypair: String,
}

fn multiple_cids(size: usize) -> Data {
    let data: String = (0..size).map(|n| format!(r#""val{}""#, n)).join(",");
    let air_script = format!(include_str!("multiple_cids.air.tmpl"), data = data);

    let exec = AirScriptExecutor::<NativeAirRunner>::new(
        TestRunParameters::from_init_peer_id("init_peer_id").with_particle_id(PARTICLE_ID),
        vec![],
        vec![],
        &air_script,
    )
    .unwrap();

    let prev_res = exec.execute_one("init_peer_id").unwrap();
    let cur_res = exec.execute_one("other_peer_id").unwrap();

    assert!(!prev_res.next_peer_pks.is_empty());

    let keypair = exec
        .get_network()
        .get_named_peer_env("init_peer_id")
        .expect("main peer")
        .borrow()
        .get_peer()
        .get_keypair()
        .clone();

    let peer_id: String = exec.resolve_name("init_peer_id").to_string();

    Data {
        air: exec.get_transformed_air_script().to_string(),
        prev_data: prev_res.data,
        cur_data: cur_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "verifying multiple CIDs for single peer".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.to_vec()).into_string(),
    }
}

fn multiple_peers(size: usize) -> Data {
    let data = (0..size).map(|n| format!(r#"@"p{}""#, n)).join(",");
    let peers: Vec<_> = (0..size).map(|n| format!("p{}", n).into()).collect();
    let air_script = format!(include_str!("multiple_peers.air.tmpl"), data = data);

    let exec = AirScriptExecutor::<NativeAirRunner>::new(
        TestRunParameters::from_init_peer_id("init_peer_id").with_particle_id(PARTICLE_ID),
        vec![],
        peers.clone(),
        &air_script,
    )
    .unwrap();

    let prev_res = exec.execute_one("init_peer_id").unwrap();

    for peer in &peers {
        exec.execute_one(peer).unwrap();
    }

    let cur_res = exec.execute_one("other_peer_id").unwrap();

    let keypair = exec
        .get_network()
        .get_named_peer_env("init_peer_id")
        .expect("main peer")
        .borrow()
        .get_peer()
        .get_keypair()
        .clone();

    let peer_id: String = exec.resolve_name("init_peer_id").to_string();

    Data {
        air: exec.get_transformed_air_script().to_string(),
        prev_data: prev_res.data,
        cur_data: cur_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "verifying many CIDs for many peers".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.to_vec()).into_string(),
    }
}

fn multiple_sigs(size: usize) -> Data {
    let data = (0..size).map(|n| format!(r#""val{}""#, n)).join(",");
    let air_script = format!(include_str!("multiple_sigs.air.tmpl"), data = data);

    let exec = AirScriptExecutor::<NativeAirRunner>::new(
        TestRunParameters::from_init_peer_id("init_peer_id").with_particle_id(PARTICLE_ID),
        vec![],
        vec![],
        &air_script,
    )
    .unwrap();

    let prev_res = exec.execute_one("init_peer_id").unwrap();
    let cur_res = exec.execute_one("other_peer_id").unwrap();

    assert!(!prev_res.next_peer_pks.is_empty());

    let keypair = exec
        .get_network()
        .get_named_peer_env("init_peer_id")
        .expect("main peer")
        .borrow()
        .get_peer()
        .get_keypair()
        .clone();

    let peer_id: String = exec.resolve_name("init_peer_id").to_string();

    Data {
        air: exec.get_transformed_air_script().to_string(),
        prev_data: prev_res.data,
        cur_data: cur_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "signing multiple CIDs".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.to_vec()).into_string(),
    }
}

fn canon_map_key_by_lens(size: usize) -> Data {
    let data: String = (0..size).map(|n| format!(r#""val{}""#, n)).join(",");
    let air_script = format!(include_str!("canon_map_key_by_lens.air.tmpl"), data = data);

    let exec = AirScriptExecutor::<NativeAirRunner>::new(
        TestRunParameters::from_init_peer_id("init_peer_id").with_particle_id(PARTICLE_ID),
        vec![],
        vec![],
        &air_script,
    )
    .unwrap();

    let prev_res = exec.execute_one("init_peer_id").unwrap();

    let keypair = exec
        .get_network()
        .get_named_peer_env("other_peer_id")
        .expect("main peer")
        .borrow()
        .get_peer()
        .get_keypair()
        .clone();

    let peer_id: String = exec.resolve_name("other_peer_id").to_string();
    let init_peer_id: String = exec.resolve_name("init_peer_id").to_string();

    Data {
        air: exec.get_transformed_air_script().to_string(),
        prev_data: vec![],
        cur_data: prev_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "benchmarking a map insert operation".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => init_peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.to_vec()).into_string(),
    }
}

fn canon_map_key_element_by_lens(size: usize) -> Data {
    let data: String = (0..size).map(|n| format!(r#""val{}""#, n)).join(",");
    let air_script = format!(
        include_str!("canon_map_key_element_by_lens.air.tmpl"),
        data = data,
        idx = size - 1
    );

    let exec = AirScriptExecutor::<NativeAirRunner>::new(
        TestRunParameters::from_init_peer_id("init_peer_id").with_particle_id(PARTICLE_ID),
        vec![],
        vec![],
        &air_script,
    )
    .unwrap();

    let prev_res = exec.execute_one("init_peer_id").unwrap();

    let keypair = exec
        .get_network()
        .get_named_peer_env("other_peer_id")
        .expect("main peer")
        .borrow()
        .get_peer()
        .get_keypair()
        .clone();

    let peer_id: String = exec.resolve_name("other_peer_id").to_string();
    let init_peer_id: String = exec.resolve_name("init_peer_id").to_string();

    Data {
        air: exec.get_transformed_air_script().to_string(),
        prev_data: vec![],
        cur_data: prev_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "benchmarking a map insert operation".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => init_peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.to_vec()).into_string(),
    }
}

fn populate_map_multiple_keys(size: usize) -> Data {
    let sq_root = (size as f64).sqrt() as usize;
    let data: String = (0..sq_root).map(|n| format!(r#""val{}""#, n)).join(",");
    let air_script = format!(
        include_str!("populate_map_multiple_keys.air.tmpl"),
        data = data
    );

    let exec = AirScriptExecutor::<NativeAirRunner>::new(
        TestRunParameters::from_init_peer_id("init_peer_id").with_particle_id(PARTICLE_ID),
        vec![],
        vec![],
        &air_script,
    )
    .unwrap();

    let prev_res = exec.execute_one("init_peer_id").unwrap();

    let keypair = exec
        .get_network()
        .get_named_peer_env("other_peer_id")
        .expect("main peer")
        .borrow()
        .get_peer()
        .get_keypair()
        .clone();

    let peer_id: String = exec.resolve_name("other_peer_id").to_string();
    let init_peer_id: String = exec.resolve_name("init_peer_id").to_string();

    Data {
        air: exec.get_transformed_air_script().to_string(),
        prev_data: vec![],
        cur_data: prev_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "benchmarking a map insert operation".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => init_peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.to_vec()).into_string(),
    }
}

fn populate_map_single_key(size: usize) -> Data {
    let data: String = (0..size).map(|n| format!(r#""val{}""#, n)).join(",");
    let air_script = format!(
        include_str!("populate_map_single_key.air.tmpl"),
        data = data
    );

    let exec = AirScriptExecutor::<NativeAirRunner>::new(
        TestRunParameters::from_init_peer_id("init_peer_id").with_particle_id(PARTICLE_ID),
        vec![],
        vec![],
        &air_script,
    )
    .unwrap();

    let prev_res = exec.execute_one("init_peer_id").unwrap();

    let keypair = exec
        .get_network()
        .get_named_peer_env("other_peer_id")
        .expect("main peer")
        .borrow()
        .get_peer()
        .get_keypair()
        .clone();

    let peer_id: String = exec.resolve_name("other_peer_id").to_string();
    let init_peer_id: String = exec.resolve_name("init_peer_id").to_string();

    Data {
        air: exec.get_transformed_air_script().to_string(),
        prev_data: vec![],
        cur_data: prev_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "benchmarking a map insert operation".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => init_peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.to_vec()).into_string(),
    }
}

fn canon_map_multiple_keys(size: usize) -> Data {
    let sq_root = (size as f64).sqrt() as usize;
    let data: String = (0..sq_root).map(|n| format!(r#""val{}""#, n)).join(",");
    let air_script = format!(
        include_str!("canon_map_multiple_keys.air.tmpl"),
        data = data
    );

    let exec = AirScriptExecutor::<NativeAirRunner>::new(
        TestRunParameters::from_init_peer_id("init_peer_id").with_particle_id(PARTICLE_ID),
        vec![],
        vec![],
        &air_script,
    )
    .unwrap();

    let prev_res = exec.execute_one("init_peer_id").unwrap();

    let keypair = exec
        .get_network()
        .get_named_peer_env("other_peer_id")
        .expect("main peer")
        .borrow()
        .get_peer()
        .get_keypair()
        .clone();

    let peer_id: String = exec.resolve_name("other_peer_id").to_string();
    let init_peer_id: String = exec.resolve_name("init_peer_id").to_string();

    Data {
        air: exec.get_transformed_air_script().to_string(),
        prev_data: vec![],
        cur_data: prev_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "benchmarking a map insert operation".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => init_peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.to_vec()).into_string(),
    }
}

fn canon_map_single_key(size: usize) -> Data {
    let data: String = (0..size).map(|n| format!(r#""val{}""#, n)).join(",");
    let air_script = format!(include_str!("canon_map_single_key.air.tmpl"), data = data);

    let exec = AirScriptExecutor::<NativeAirRunner>::new(
        TestRunParameters::from_init_peer_id("init_peer_id").with_particle_id(PARTICLE_ID),
        vec![],
        vec![],
        &air_script,
    )
    .unwrap();

    let prev_res = exec.execute_one("init_peer_id").unwrap();

    let keypair = exec
        .get_network()
        .get_named_peer_env("other_peer_id")
        .expect("main peer")
        .borrow()
        .get_peer()
        .get_keypair()
        .clone();

    let peer_id: String = exec.resolve_name("other_peer_id").to_string();
    let init_peer_id: String = exec.resolve_name("init_peer_id").to_string();

    Data {
        air: exec.get_transformed_air_script().to_string(),
        prev_data: vec![],
        cur_data: prev_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "benchmarking a map insert operation".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => init_peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.to_vec()).into_string(),
    }
}

fn canon_map_scalar_multiple_keys(size: usize) -> Data {
    let sq_root = (size as f64).sqrt() as usize;
    let data: String = (0..sq_root).map(|n| format!(r#""val{}""#, n)).join(",");
    let air_script = format!(
        include_str!("canon_map_scalar_multiple_keys.air.tmpl"),
        data = data
    );

    let exec = AirScriptExecutor::<NativeAirRunner>::new(
        TestRunParameters::from_init_peer_id("init_peer_id").with_particle_id(PARTICLE_ID),
        vec![],
        vec![],
        &air_script,
    )
    .unwrap();

    let prev_res = exec.execute_one("init_peer_id").unwrap();

    let keypair = exec
        .get_network()
        .get_named_peer_env("other_peer_id")
        .expect("main peer")
        .borrow()
        .get_peer()
        .get_keypair()
        .clone();

    let peer_id: String = exec.resolve_name("other_peer_id").to_string();
    let init_peer_id: String = exec.resolve_name("init_peer_id").to_string();

    Data {
        air: exec.get_transformed_air_script().to_string(),
        prev_data: vec![],
        cur_data: prev_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "benchmarking a map insert operation".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => init_peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.to_vec()).into_string(),
    }
}

fn canon_map_scalar_single_key(size: usize) -> Data {
    let data: String = (0..size).map(|n| format!(r#""val{}""#, n)).join(",");
    let air_script = format!(
        include_str!("canon_map_scalar_single_key.air.tmpl"),
        data = data
    );

    let exec = AirScriptExecutor::<NativeAirRunner>::new(
        TestRunParameters::from_init_peer_id("init_peer_id").with_particle_id(PARTICLE_ID),
        vec![],
        vec![],
        &air_script,
    )
    .unwrap();

    let prev_res = exec.execute_one("init_peer_id").unwrap();

    let keypair = exec
        .get_network()
        .get_named_peer_env("other_peer_id")
        .expect("main peer")
        .borrow()
        .get_peer()
        .get_keypair()
        .clone();

    let peer_id: String = exec.resolve_name("other_peer_id").to_string();
    let init_peer_id: String = exec.resolve_name("init_peer_id").to_string();

    Data {
        air: exec.get_transformed_air_script().to_string(),
        prev_data: vec![],
        cur_data: prev_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "benchmarking a map insert operation".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => init_peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.to_vec()).into_string(),
    }
}

fn long_data() -> Data {
    use cid_benchmarking::cid_benchmarking_long_data;

    let (keypair, peer_id) = derive_dummy_keypair("init_peer_id");
    let particle_id = "particle_id";
    let cur_data = cid_benchmarking_long_data(&keypair, peer_id.clone(), particle_id);

    Data {
        air: "(null)".to_owned(),
        prev_data: vec![],
        cur_data,
        params_json: hashmap! {
            "comment".to_owned() => "Long data trace".to_owned(),
            "particle-id".to_owned() => particle_id.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.as_inner().to_vec()).into_string(),
    }
}

fn big_values_data() -> Data {
    use cid_benchmarking::cid_benchmarking_big_values_data;

    let (keypair, peer_id) = derive_dummy_keypair("init_peer_id");
    let particle_id = "particle_id";
    let cur_data = cid_benchmarking_big_values_data(&keypair, peer_id.clone(), particle_id);

    Data {
        air: "(null)".to_owned(),
        prev_data: vec![],
        cur_data,
        params_json: hashmap! {
            "comment".to_owned() => "Loading a trace with huge values".to_owned(),
            "particle-id".to_owned() => particle_id.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.as_inner().to_vec()).into_string(),
    }
}

fn generate_random_data(random_data_size: usize) -> Vec<u8> {
    use rand::thread_rng;
    use rand::Rng;

    // hex::encode later prints out every byte as 2 bytes sequence.
    let random_data_size = random_data_size / 2 * MB;

    let mut random_data = Vec::<u8>::with_capacity(random_data_size);
    let mut rng = thread_rng();

    while random_data.len() < random_data_size {
        let remaining_space = random_data_size - random_data.len();
        let chunk_size = std::cmp::min(rng.gen_range(1..=remaining_space), remaining_space);
        let mut chunk = vec![0u8; chunk_size];
        rng.fill(&mut chunk[..]);
        random_data.extend_from_slice(&chunk);
    }

    random_data.truncate(random_data_size);
    random_data
}

fn mem_consumption_with_size_in_mb(data_size: usize) -> Data {
    let random_data = generate_random_data(data_size);

    let air_script = format!(
        include_str!("mem_consumption_generic.air.tmpl"),
        data = format_args!("\"{}\"", hex::encode(random_data))
    );

    let exec = AirScriptExecutor::<NativeAirRunner>::new(
        TestRunParameters::from_init_peer_id("init_peer_id").with_particle_id(PARTICLE_ID),
        vec![],
        vec![],
        &air_script,
    )
    .unwrap();

    let keypair = exec
        .get_network()
        .get_named_peer_env("other_peer_id")
        .expect("main peer")
        .borrow()
        .get_peer()
        .get_keypair()
        .clone();

    let prev_res = exec.execute_one("init_peer_id").unwrap();

    let peer_id: String = exec.resolve_name("other_peer_id").to_string();
    let init_peer_id: String = exec.resolve_name("init_peer_id").to_string();

    Data {
        air: exec.get_transformed_air_script().to_string(),
        prev_data: vec![],
        cur_data: prev_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "benchmarking".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => init_peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.to_vec()).into_string(),
    }
}

fn mem_consumption_w_lense_with_size_in_mb(data_size: usize) -> Data {
    let random_data = generate_random_data(data_size);

    let air_script = format!(
        include_str!("mem_consumption_lense.air.tmpl"),
        data = format_args!("{{\"attrib\": \"{}\"}}", hex::encode(random_data))
    );

    let exec = AirScriptExecutor::<NativeAirRunner>::new(
        TestRunParameters::from_init_peer_id("init_peer_id").with_particle_id(PARTICLE_ID),
        vec![],
        vec![],
        &air_script,
    )
    .unwrap();

    let keypair = exec
        .get_network()
        .get_named_peer_env("other_peer_id")
        .expect("main peer")
        .borrow()
        .get_peer()
        .get_keypair()
        .clone();

    let prev_res = exec.execute_one("init_peer_id").unwrap();

    let peer_id: String = exec.resolve_name("other_peer_id").to_string();
    let init_peer_id: String = exec.resolve_name("init_peer_id").to_string();

    Data {
        air: exec.get_transformed_air_script().to_string(),
        prev_data: vec![],
        cur_data: prev_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "benchmarking".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => init_peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.to_vec()).into_string(),
    }
}

fn mem_consumption_w_map_2_scalar_with_size_in_mb(data_size: usize) -> Data {
    let random_data = generate_random_data(data_size);

    let air_script = format!(
        include_str!("mem_consumption_canon_map.air.tmpl"),
        data = format_args!("\"{}\"", hex::encode(random_data))
    );

    let exec = AirScriptExecutor::<NativeAirRunner>::new(
        TestRunParameters::from_init_peer_id("init_peer_id").with_particle_id(PARTICLE_ID),
        vec![],
        vec![],
        &air_script,
    )
    .unwrap();

    let keypair = exec
        .get_network()
        .get_named_peer_env("other_peer_id")
        .expect("main peer")
        .borrow()
        .get_peer()
        .get_keypair()
        .clone();

    let prev_res = exec.execute_one("init_peer_id").unwrap();

    let peer_id: String = exec.resolve_name("other_peer_id").to_string();
    let init_peer_id: String = exec.resolve_name("init_peer_id").to_string();

    Data {
        air: exec.get_transformed_air_script().to_string(),
        prev_data: vec![],
        cur_data: prev_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "benchmarking".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => init_peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.to_vec()).into_string(),
    }
}

fn mem_consumption_w_canon_map_with_size_in_mb(data_size: usize) -> Data {
    let random_data = generate_random_data(data_size);

    let air_script = format!(
        include_str!("mem_consumption_canon_map_2.air.tmpl"),
        data = format_args!("\"{}\"", hex::encode(random_data))
    );

    let exec = AirScriptExecutor::<NativeAirRunner>::new(
        TestRunParameters::from_init_peer_id("init_peer_id").with_particle_id(PARTICLE_ID),
        vec![],
        vec![],
        &air_script,
    )
    .unwrap();

    let keypair = exec
        .get_network()
        .get_named_peer_env("other_peer_id")
        .expect("main peer")
        .borrow()
        .get_peer()
        .get_keypair()
        .clone();

    let prev_res = exec.execute_one("init_peer_id").unwrap();

    let peer_id: String = exec.resolve_name("other_peer_id").to_string();
    let init_peer_id: String = exec.resolve_name("init_peer_id").to_string();

    Data {
        air: exec.get_transformed_air_script().to_string(),
        prev_data: vec![],
        cur_data: prev_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "benchmarking".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => init_peer_id,
        },
        call_results: None,
        keypair: bs58::encode(keypair.to_vec()).into_string(),
    }
}

fn mem_consumption_air_100mb(calls: usize, vars: usize) -> Data {
    let (keypair, peer_id) = derive_dummy_keypair("init_peer_id");
    let particle_id = "particle_id";

    let vars = (0..vars).map(|n| format!("var{}", n)).collect_vec();
    let init_var = vars[0].clone();
    let statements = vars
        .iter()
        .cycle()
        .take(calls)
        .tuple_windows()
        .map(|(a, b)| format!(r#"(call {a} ("serv" "func") [] {b})"#))
        .collect_vec();

    fn build_tree(statements: &[String]) -> String {
        assert!(!statements.is_empty());
        if statements.len() == 1 {
            statements[0].clone()
        } else {
            let mid = statements.len() / 2;
            format!(
                "(seq {} {})",
                build_tree(&statements[..mid]),
                build_tree(&statements[mid..])
            )
        }
    }

    let tree = build_tree(&statements);
    let air = format!(
        r#"(seq (call "peer" ("serv" "func") [] {}) {})"#,
        init_var, tree
    );

    Data {
        air,
        prev_data: vec![],
        cur_data: vec![],
        call_results: None,
        keypair: bs58::encode(keypair.as_inner().to_vec()).into_string(),
        params_json: hashmap! {
            "comment".to_owned() => "multiple calls parser benchmark".to_owned(),
            "particle-id".to_owned() => particle_id.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => peer_id,
        },
    }
}

fn parser_10000_100() -> Data {
    let air_script = include_str!("parser_10000_100.air");

    let (keypair, peer_id) = derive_dummy_keypair("init_peer_id");
    let particle_id = "particle_id";

    Data {
        air: air_script.to_owned(),
        prev_data: vec![],
        cur_data: vec![],
        call_results: None,
        keypair: bs58::encode(keypair.as_inner().to_vec()).into_string(),
        params_json: hashmap! {
            "comment".to_owned() => "long air script with lot of variable assignments".to_owned(),
            "particle-id".to_owned() => particle_id.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => peer_id,
        },
    }
}

fn parser_calls(calls: usize, vars: usize) -> Data {
    let (keypair, peer_id) = derive_dummy_keypair("init_peer_id");
    let particle_id = "particle_id";

    let vars = (0..vars).map(|n| format!("var{}", n)).collect_vec();
    let init_var = vars[0].clone();
    let statements = vars
        .iter()
        .cycle()
        .take(calls)
        .tuple_windows()
        .map(|(a, b)| format!(r#"(call {a} ("serv" "func") [] {b})"#))
        .collect_vec();

    fn build_tree(statements: &[String]) -> String {
        assert!(!statements.is_empty());
        if statements.len() == 1 {
            statements[0].clone()
        } else {
            let mid = statements.len() / 2;
            format!(
                "(seq {} {})",
                build_tree(&statements[..mid]),
                build_tree(&statements[mid..])
            )
        }
    }

    let tree = build_tree(&statements);
    let air = format!(
        r#"(seq (call "peer" ("serv" "func") [] {}) {})"#,
        init_var, tree
    );

    Data {
        air,
        prev_data: vec![],
        cur_data: vec![],
        call_results: None,
        keypair: bs58::encode(keypair.as_inner().to_vec()).into_string(),
        params_json: hashmap! {
            "comment".to_owned() => "multiple calls parser benchmark".to_owned(),
            "particle-id".to_owned() => particle_id.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => peer_id,
        },
    }
}

fn null() -> Data {
    let air_script = "(null)";

    let (keypair, peer_id) = derive_dummy_keypair("init_peer_id");
    let particle_id = "particle_id";

    Data {
        air: air_script.to_owned(),
        prev_data: vec![],
        cur_data: vec![],
        call_results: None,
        keypair: bs58::encode(keypair.as_inner().to_vec()).into_string(),
        params_json: hashmap! {
            "comment".to_owned() => "Empty data and null script".to_owned(),
            "particle-id".to_owned() => particle_id.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => peer_id,
        },
    }
}
