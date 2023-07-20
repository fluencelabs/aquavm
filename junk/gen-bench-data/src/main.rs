use air_test_framework::*;
use air_test_utils::prelude::*;
use clap::{Parser, Subcommand};
use itertools::Itertools as _;
use maplit::hashmap;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

const PARTICLE_ID: &str = "0123456789ABCDEF";

// mod dashboard;
// mod network_explore;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    bench: Bench,

    #[arg(long)]
    dest_dir: PathBuf,
}

#[derive(Debug, Subcommand)]
enum Bench {
    // MultipleCids10,
    // MultipleCids50,
    // MultiplePeers5,
    // MultiplePeers14,
    // MultiplePeers25,
    // MultipleSigs10,
    // MultipleSigs50,
    MultipleSigs200,
    // Dashboard,
    // NetworkExplore,
}

fn main() {
    let args = Cli::parse();

    let data = match args.bench {
        // Bench::MultipleCids10 => multiple_cids(10),
        // Bench::MultipleCids50 => multiple_cids(50),
        // Bench::MultiplePeers5 => multiple_peers(5),
        // Bench::MultiplePeers14 => multiple_peers(14),
        // Bench::MultiplePeers25 => multiple_peers(25),
        // Bench::MultipleSigs10 => multiple_sigs(10),
        // Bench::MultipleSigs50 => multiple_sigs(50),
        Bench::MultipleSigs200 => multiple_sigs(200),
        // Bench::Dashboard => dashboard::dashboard(),
        // Bench::NetworkExplore => network_explore::network_explore(),
    };

    save_data(&args.dest_dir, data).unwrap();
}

fn save_data(dest_dir: &Path, data: Data) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::*;

    create_dir_all(dest_dir)?;

    save_file(dest_dir, "script.air", &data.air)?;
    save_file(dest_dir, "prev_data.json", &reformat_json(&data.prev_data))?;
    save_file(dest_dir, "cur_data.json", &reformat_json(&data.cur_data))?;
    save_file(dest_dir, "params.json", &serde_json::to_vec_pretty(&data.params_json)?)?;

    Ok(())
}

/// make zero-indentation data for better git diffs
fn reformat_json(data: &[u8]) -> Vec<u8> {
    use serde::ser::Serialize;

    let obj: serde_json::Value = serde_json::from_slice(data).unwrap();
    let fmt = serde_json::ser::PrettyFormatter::with_indent(&[]);
    let mut out = vec![];
    let mut ser = serde_json::ser::Serializer::with_formatter(&mut out, fmt);
    obj.serialize(&mut ser).unwrap();
    out
}

fn save_file(dest_dir: &Path, filename: &str, data: impl AsRef<[u8]>) -> Result<(), Box<dyn std::error::Error>>{
    use std::fs::*;
    use std::io::prelude::*;

    let mut dest_dir = dest_dir.to_owned();
    dest_dir.push(filename);

    let mut f = File::create(&dest_dir)?;
    f.write_all(data.as_ref())?;

    Ok(())
}

#[derive(Debug, Default)]
pub(crate) struct Data {
    pub(crate) air: String,
    pub(crate) prev_data: Vec<u8>,
    pub(crate) cur_data: Vec<u8>,
    pub(crate) params_json: HashMap<String, String>,
    pub(crate) call_results: Option<serde_json::Value>,
}

fn multiple_cids(size: usize) -> Data {
    let data = (0..size).map(|n| format!(r#""val{}""#, n)).join(",");
    let air_script = format!(include_str!("multiple_cids.air.tmpl"), data = data);

    let exec = AirScriptExecutor::new(
        TestRunParameters::from_init_peer_id("init_peer_id"),
        vec![],
        vec![],
        &air_script,
    ).unwrap();

    let prev_res = exec.execute_one("init_peer_id").unwrap();
    let cur_res = exec.execute_one("other_peer_id").unwrap();

    assert!(!prev_res.next_peer_pks.is_empty());

    let peer_id: String = ("init_peer_id").to_string();

    Data {
        air: air_script,
        prev_data: prev_res.data,
        cur_data: cur_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "verifying multiple CIDs for single peer".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => peer_id,
        },
        call_results: None,
    }
}

fn multiple_peers(size: usize) -> Data {
    let data = (0..size).map(|n| format!(r#"@"p{}""#, n)).join(",");
    let peers: Vec<_> = (0..size).map(|n| format!("p{}", n).into()).collect();
    let air_script = format!(include_str!("multiple_peers.air.tmpl"), data = data);

    let exec = AirScriptExecutor::new(
        TestRunParameters::from_init_peer_id("init_peer_id"),
        vec![],
        peers.clone(),
        &air_script,
    ).unwrap();

    let prev_res = exec.execute_one("init_peer_id").unwrap();

    for peer in &peers {
        exec.execute_one(peer).unwrap();
    }

    let cur_res = exec.execute_one("other_peer_id").unwrap();

    let peer_id: String = ("init_peer_id").to_string();

    Data {
        air: air_script,
        prev_data: prev_res.data,
        cur_data: cur_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "verifying many CIDs for many peers".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => peer_id,
        },
        call_results: None,
    }
}

fn multiple_sigs(size: usize) -> Data {
    let data = (0..size).map(|n| format!(r#""val{}""#, n)).join(",");
    let air_script = format!(include_str!("multiple_sigs.air.tmpl"), data = data);

    let exec = AirScriptExecutor::new(
        TestRunParameters::from_init_peer_id("init_peer_id"),
        vec![],
        vec![],
        &air_script,
    ).unwrap();

    let prev_res = exec.execute_one("init_peer_id").unwrap();
    let cur_res = exec.execute_one("other_peer_id").unwrap();

    assert!(!prev_res.next_peer_pks.is_empty());

    let peer_id: String = ("init_peer_id").to_string();

    Data {
        air: air_script,
        prev_data: prev_res.data,
        cur_data: cur_res.data,
        params_json: hashmap! {
            "comment".to_owned() => "signing multiple CIDs".to_owned(),
            "particle-id".to_owned() => PARTICLE_ID.to_owned(),
            "current-peer-id".to_owned() => peer_id.clone(),
            "init-peer-id".to_owned() => peer_id,
        },
        call_results: None,
    }
}
