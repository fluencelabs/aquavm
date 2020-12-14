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

use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};
use tinyjson::{JsonValue as JValue, JsonValue};

// const INTERPRETER_WASM: &'static [u8] = include_bytes!("aquamarine.wasm");
const PKG_NAME: &'static str = "air-interpreter-wasm";

fn as_array(jv: &JValue) -> Option<&Vec<JsonValue>> {
    match jv {
        JsonValue::Array(packages) => Some(packages),
        _ => None,
    }
}

fn get_src_dir() -> PathBuf {
    let cargo = env::var_os("CARGO").expect("read CARGO from env");
    let metadata = Command::new(cargo)
        .arg("metadata")
        .output()
        .expect("execute `cargo metadata`")
        .stdout;

    let metadata =
        std::str::from_utf8(metadata.as_slice()).expect("`cargo metadata` output is valid utf8");
    let metadata: JValue = metadata.parse().expect("valid json from `cargo metadata`");
    let packages = as_array(&metadata["packages"]).expect(".packages must be an array");

    let package = packages
        .iter()
        .find(|pkg| pkg["name"] == JValue::String(PKG_NAME.to_string()))
        .expect(format!("find package {} in `cargo metadata`", PKG_NAME).as_str());

    let targets = as_array(&package["targets"]).expect("package contain field .targets");
    let target_lib = targets
        .iter()
        .find(|t| t["kind"] == JValue::Array(vec![JValue::String("lib".to_string())]))
        .expect(format!("find target 'lib' in package {:#?}", package).as_str());

    let src_path = match &target_lib["src_path"] {
        JsonValue::String(path) => path,
        _ => panic!("target.src_path must be a string"),
    };

    let src_dir = Path::new(src_path)
        .parent()
        .and_then(|p| p.parent())
        .expect(format!("{:?}/../.. must exist", src_path).as_str());

    src_dir.to_owned()
}

fn main() {
    let src_dir = get_src_dir();
    let wasm = src_dir.join("aquamarine.wasm");

    let out_dir = env::var_os("OUT_DIR").expect("read OUT_DIR from env");
    let out_dest_path = Path::new(&out_dir).join("aquamarine.wasm");
    println!("out_dest_path is {:?}", out_dest_path);
    fs::copy(&wasm, &out_dest_path)
        .expect(format!("copy {:?} to {:?}", wasm, out_dest_path).as_str());

    if let Some(target_dir) = env::var_os("CARGO_BUILD_TARGET_DIR") {
        let target_dest_path = Path::new(&target_dir).join("aquamarine.wasm");
        println!("target_dest_path is {:?}", out_dest_path);

        fs::copy(&wasm, &target_dest_path)
            .expect(format!("copy wasm to target_dest_path: {:?}", target_dest_path).as_str());
    }

    // fs::write(&dest_path, INTERPRETER_WASM)
    //     .expect(format!("Write aquamarine.wasm to {:?}", dest_path).as_str());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=aquamarine.wasm");
}
