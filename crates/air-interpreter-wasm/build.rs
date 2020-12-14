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

// use std::path::Path;
// use std::{env, fs};
//
// const INTERPRETER_WASM: &'static [u8] = include_bytes!("aquamarine.wasm");

fn main() {
    println!("I: air-interpreter-wasm build.rs ran");
    eprintln!("E: air-interpreter-wasm build.rs ran");
    // let out_dir = env::var_os("OUT_DIR").expect("Read OUT_DIR from env");
    // let dest_path = Path::new(&out_dir).join("aquamarine.wasm");
    // println!("aaa Dest path is {:?}", dest_path);
    // fs::write(&dest_path, INTERPRETER_WASM)
    //     .expect(format!("Write aquamarine.wasm to {:?}", dest_path).as_str());
    // println!("cargo:rerun-if-changed=build.rs");
    // println!("cargo:rerun-if-changed=aquamarine.wasm");
}
