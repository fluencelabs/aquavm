# The `air` CLI utility

The `air` CLI utility is a helper tool for Aqua and AIR developers.  It has three subcommands: `beautify`, `run` and `stats`.

## `air beautify`

Alias: `air b`.

This subcommand reads an AIR script from standard input and prints it in human-readable Python-like representation.  This representation cannot be executed and is intended to be read by mere mortals.

It outputs to standard output or a file.


## `air run`

Alias: `air r`.

Executes an AIR script with data in WASM AquaVM.  It has two modes of parameter input: plain and anomaly.

### Common parameters
All common parameters are optional.  Their position is always before the mode selector (`--plain` or `--anomaly`).

+ `--call-results PATH` parameter allows you to provide call results for current execution.
+ `--max-heap-size N` defines maximum heap size for WASM runtime.
+ `--interpreter PATH` option defines the AquaVM WASM binary to be executed.  By default, it is "target/wasm32-wasi/release/air_interpreter_server.wasm", but you can define a global value with the `AIR_INTERPRETER_WASM_PATH` environment variable.  The default presumes that the tool is run from the root of this repository.  Feel free to use option or environment variable to run from any location.
+ with the `--json` option, tracing info is output (to stderr) in machine-readable JSON format.  The output can be later processed with `air stats` subcommand.
+ `--tracing-params` defines tracing logging levels for AquaVM.  By default, it is equal to `info` and does trace the most high-level AquaVM constructions (data parsing, AIR script parsing, execution).  With `debug` level it traces some individual commands, and with `trace` level it traces even more fine grained functionality, but it induce more overhead.
+ `--runner-tracing-params` defines tracing logging level for the runner.  By default, it is equal to `warn`. 
+ `--random-key` or `--ed25519-key keyfile` (required): use random signing key or load it from a file.

The important option is `--native`.  It runs the AquaVM as the native code that can be profiled with any native profiler.  As input data deserialization and serialization time can be comparable to particle execution time, and short execution times provides less reliable results, one can use `--repeat N` option to repeat particle execution several times.  Execution result is not printed in this case, so you may run `--repeat 1` to suppress it.

Run `air run --help` to see all common parameters.

### Plain mode
In the `--plain` mode, the parameters like AIR script path, data path, previous data path and other particle fields can be provided in separate arguments (all of them are optional, and AIR script is read from stdin by default).

Run `air run --plain --help` to see all plain mode options.

### Anomaly mode
In the anomaly mode, the only argument is a path to self-contained anomaly data file obtained from `rust-peer`'s Anomaly Particle Detection System.

Run `air run --anomaly --help` to see all anomaly mode options.

## `air stats`

Alias: `air s`.

This subcommand allows to process JSON trace collected with `air run --json`.  It has two primary options:

+ `--pretty` outputs JSON trace in human readable format.
+ `--stats` outputs execution summary.

By default, both options are effective.

The `--sort-stats-by-duration` flag sorts spans by time, not by name, in the report.

Please, note that currently tracing outputs to stdout, and execution result is also printed to stdout.  You may suppress printing the result with `air run --repeat 1` option.

## `air near`

Alias: `air n`.  Feature flag: `near`.

Run AIR interpreter built as NEAR smart contract to measure its gas consumption.  The contract has to be built separately, see `tools/wasm/air-near-contract` directory.  Unless you use the default built contract location, you may use `--contract` option or `AIR_NEAR_CONRACT_WASM_PATH` variable.

Other options are equivalent to `air run ... --plain ...` options, except tracing.

The `workspaces` crate in sandbox mode is used for the most accurate NEAR gas measurement.  The `workspaces` build script installs NEAR Sandbox automatically, but on the other machines you have to install it manually and set the `NEAR_SANDBOX_BIN_PATH` variable.

## Known limitations

1. At detailed tracing levels (debug etc), trace formatting time is comparable to traced code execution time and can give incorrect results.

## Installation

### AIR interpreter

Unless you intend to run AIR in native mode, you will need a AIR interpreter build in WASM.  You can build it with the `marine` tool.  Run following command in the repo's root directory:

``` sh
marine build --features marine --package air-interpreter --release
```

It will output the binary to default `--interpreter` path at `target/wasm32-wasi/release/air_interpreter_server.wasm`; if you wish to run the `air` from arbitrary place, store the `air_interpreter_server.wasm` binary in a cool dry place and either set `AIR_INTERPRETER_WASM_PATH` variable or use the `--interpreter` common option of `air run`.

## `air` binary

You need to have Rust toolchain and its `cargo` utility installed.  Run this command to install latest released version from `crates.io`:

``` sh
cargo install aquavm-air-cli
```

You may install developer version from the repo's root:

``` sh
cargo install --path tools/cli/air
```

## `air` CLI native build

You can build fully native or pure WASM `air` CLI build with the following commands:

``` sh
cargo build --no-default-features --release -p aquavm-air-cli
cargo build --no-default-features --release -p aquavm-air-cli --target wasm32-wasi
```

This build doesn't need the AIR interpreter WASM binary.
