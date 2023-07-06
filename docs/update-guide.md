# AquaVM update guide

## AquaVM repo components

Here is the list of main components crucial from the update point of view:

[**AquaVM core**](./air) - the main part of AquaVM interpreter, e.g., it contains instruction implementation
[**AVM client**](./avm/client) - an AquaVM launcher for browser and Node.js targets
[**AVM server**](./avm/server) - an AquaVM launcher for server-side targets
[**AIR parser**](./crates/trace-handler) - a parser of AIR code
[**Interpreter data**](./crates/air-lib/interpreter-data) - contains definition of data that is passed between different AquaVM in so called particle
[**Interpreter interface**](./crates/air-lib/interpreter-interface) - contains definition of AquaVM interface which is used by `AVM server` and test runners
[**Interpreter signature**](./crates/air-lib/interpreter-signatures) - utility crate for signatures, e.g. contains methods for se/de
[**Trace Handler**](./crates/air-lib/trace-handler) - crate intended to handle all interpreter data related stuff

## AquaVM core updating policy

There are three main variables in the code intended to set and check versions of the main AquaVM core components:

### MINIMAL_INTERPRETER_VERSION

This variable sets the minimal supported version of `AquaVM core`. This variable should be updated after every breaking change in `AquaVM core`, `AIR parser`, `Interpreter data`, `Interpreter signature`, `Trace Handler`, and maybe other crates that break an AIR script execution. Particle'll be rejected if it comes from a network containing a version less than the specified `MINIMAL_INTERPRETER_VERSION` with an error propagated to a host running this AquaVM (so, an error message won't be sent to a peer initiated this communication).

### INTERPRETER_VERSION

It represents the version of the current AquaVM instance, this variable is passed in interpreter data and compared with `MINIMAL_INTERPRETER_VERSION` at the preparation stage. It is updated automatically by `AquaVM core` version.

### INTERPRETER_DATA_VERSION

This variable represents the current version of an interpreter data format, it aims to create a more clear error message when a particle is rejected or is failed to deserialize after a breaking change.

## AVM updating policy

Both `AVM client` and `AVM server` versions should be updated simultaniously in case of breaking change in `AquaVM core` interface, e.g., when arguments are changes. Often they must be updated if `Interpreter interface` crate was changed, but they not need to be updated if `Interpreter data` or `AquaVM core` itself was changed.
