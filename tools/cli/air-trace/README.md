# The `air-trace` utility

The main propose of this utility is to run AquaVM with data and analyze its performance.  It has two subcommands: `run` and `stats`.

## `air-trace run`

Executes an AIR script with data in WASM AquaVM.  It has two modes of parameter input: plain and anomaly.

### Common parameters
All common parameters are optional.  Their position is before the mode selector (`--plain` or `--anomaly`).

+ `--call-results PATH` parameter allows you to provide call results for current execution.
+ `--current-peer-id STR` by default is "some_id".
+ `--max-heap-size N` defines maximum heap size for WASM runtime.
+ `--interpreter PATH` option defines the AquaVM WASM binary to be executed.  By default, it is "target/wasm32-wasi/release/air_interpreter_server.wasm", but you can define a global value with the `AIR_INTERPRETER_WASM_PATH` environment variable.  The default presumes that the tool is run from the root of this repository.  Feel free to use option or environment variable to run from any location.
+ with the `--json` option, tracing info is output in machine-readable JSON format.  The output can be later processed with `air-trace stats` subcommand.
+ `--tracing-params` defines tracing crate logging levels.  By default, it is equal to `info` and does trace the most high-level AquaVM constructions (data parsing, AIR script parsing, execution, result construction).  With `debug` level it traces some individual commands, and with `trace` level it traces even more fine grained functionality.

The important option is `--native`.  It runs the AquaVM as the native code that can be profiled with any native profiler.  As input data deserialization and serialization time can be comparable to particle execution time, and short execution times provides less reliable results, one can use `--repeat N` option to repeat particle execution several times.  Execution result is not printed in this case, so you may run `--repeat 1` to suppress it.

Run `air-trace run --help` to see all common parameters.

### Plain mode
In the `--plain` mode, the parameters like AIR script path, data path, previous data path and other particle fields can be provided in separate arguments, of which only `--data` is the required one (and AIR script is read from stdin by default).

Run `air-trace run --plain --help` to see all plain mode options.

### Anomaly mode
In the anomaly mode, the only argument is a path to self-contained anomaly data file obtained from Anomaly Particle Detection System.

Run `air-trace run --anomaly --help` to see all anomaly mode options.

## `air-trace stats`

This subcommand allows to process JSON trace collected with `air-trace run --json`.  It has two primary options:

+ `--pretty` outputs JSON trace in human readable format.
+ `--stats` outputs execution summary.

By default, both options are effective.

The `--sort-stats-by-duration` flag sorts spans by time, not by name, in the report.

Please, note that currently tracing outputs to stdout, and execution result is also printed to stdout.  You may suppress printing the result with `air-trace run --repeat 1` option.

## Known limitations

1. At detailed tracing levels (debug etc), trace formatting time is comparable to traced code execution time and can give incorrect results.
2. Native builds of the utility still depend on Marine, and cannot be built for architectures unsupported by Marine, like Apple Silicon or WASM.  It is yet to be resolved with some refactoring.
3. Traces are printed to stdout.

## Installation

### AIR interpreter server

You need the `marine` tool installed.  Run following command in the repo's root directory:

``` sh
marine build --features marine --package air-interpreter --release
```

It will output the binary to default `--interpreter` path at `target/wasm32-wasi/release/air_interpreter_server.wasm`; if you wish to run the `air-trace` from arbitrary place, store the `air_interpreter_server.wasm` binary in a cool dry place and set `AIR_INTERPRETER_WASM_PATH` variable.

## `air-trace` binary

You need to have Rust toolchain and its `cargo` utility installed.  Run this command from the repo's root directory:

``` sh
cargo install --path tools/cli/air-trace
```
