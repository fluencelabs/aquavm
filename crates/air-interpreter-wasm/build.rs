/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use serde::Deserialize;
use serde_json::Value as JValue;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

// const INTERPRETER_WASM: &'static [u8] = include_bytes!("aquamarine.wasm");
const PKG_NAME: &'static str = "air-interpreter-wasm";

#[derive(Deserialize)]
struct Target {
    pub src_path: Path,
    pub kind: String,
}

#[derive(Deserialize, Debug)]
struct Package {
    pub name: String,
    pub targets: Vec<Target>,
}

#[derive(Deserialize)]
struct Metadata {
    pub packages: Vec<Package>,
}

fn get_src_dir() -> Path {
    let metadata = Command::new("$CARGO")
        .arg("metadata")
        .output()
        .expect("execute `cargo metadata`")
        .stdout;

    let metadata: Metadata =
        serde_json::from_slice(&metadata).expect("valid json from `cargo metadata`");

    let package = metadata
        .packages
        .into_iter()
        .find(|pkg| pkg.name.as_str() == PKG_NAME)
        .expect(format!("find package {} in `cargo metadata`", PKG_NAME).as_str());

    let target = package
        .targets
        .into_iter()
        .find(|t| t.kind == "lib")
        .expect(format!("find target 'lib' in package {:#?}", package).as_str());

    let src_dir = target
        .src_path
        .parent()
        .expect(format!("{:?} have parent dir", target.src_path).as_str());

    src_dir.to_owned().into()
}

fn main() {
    let src_dir = get_src_dir();
    let wasm = src_dir.join("aquamarine.wasm");

    let out_dir = env::var_os("OUT_DIR").expect("read OUT_DIR from env");
    let out_dest_path = Path::new(&out_dir).join("aquamarine.wasm");
    let target_dir = env::var_os("CARGO_TARGET_DIR").expect("read CARGO_TARGET_DIR from env");
    let target_dest_path = Path::new(&target_dir).join("aquamarine.wasm");
    println!("out_dest_path is {:?}", out_dest_path);
    println!("target_dest_path is {:?}", out_dest_path);

    fs::copy(&wasm, out_dest_path);
    fs::copy(&wasm, target_dest_path);

    // fs::write(&dest_path, INTERPRETER_WASM)
    //     .expect(format!("Write aquamarine.wasm to {:?}", dest_path).as_str());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=aquamarine.wasm");
}
