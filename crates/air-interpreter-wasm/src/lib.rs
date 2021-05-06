pub const INTERPRETER_WASM: &'static [u8] = include_bytes!("../air_interpreter_server.wasm");

pub mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub use build_info::PKG_VERSION as VERSION;
