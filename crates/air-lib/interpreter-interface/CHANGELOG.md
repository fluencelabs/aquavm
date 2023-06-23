# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.14.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-interface-v0.13.0...air-interpreter-interface-v0.14.0) (2023-06-22)


### ⚠ BREAKING CHANGES

* **avm-server,air-interpreter,aquavm-air:** secret key and particle ID arguments ([#593](https://github.com/fluencelabs/aquavm/issues/593))

### Features

* **avm-server,air-interpreter,aquavm-air:** secret key and particle ID arguments ([#593](https://github.com/fluencelabs/aquavm/issues/593)) ([8ce8af3](https://github.com/fluencelabs/aquavm/commit/8ce8af38232de3f1ac359214386b895356550428))


### Bug Fixes

* **deps:** update rust crate fluence-it-types to 0.4.1 ([#545](https://github.com/fluencelabs/aquavm/issues/545)) ([138501f](https://github.com/fluencelabs/aquavm/commit/138501fff91aaa4082351f65b0e493215b338fbf))

## [0.13.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-interface-v0.12.1...air-interpreter-interface-v0.13.0) (2023-03-15)


### ⚠ BREAKING CHANGES

* **avm:** integrate Marine 0.26.0 ([#461](https://github.com/fluencelabs/aquavm/issues/461))

### Features

* **avm:** integrate Marine 0.26.0 ([#461](https://github.com/fluencelabs/aquavm/issues/461)) ([126d550](https://github.com/fluencelabs/aquavm/commit/126d5507c81a7f978ab9cf06c492b1092a336cf6))

## [Unreleased]

## [0.12.1] - 2022-12-26

+ Remove the new `cid` field of the `InterpreterOutcome` as it is not really needed.

## [0.12.0] - 2022-12-21

+ New `cid` field of the `InterpreterOutcome` contains CID of the data.

## [0.11.1] - 2022-09-13

### Other
- Update all non-major Rust dependencies (#323)
- Update all non-major Rust dependencies (#321)
- Update all non-major Rust dependencies (#309)
- Get rid of unsafe code in the interpreter (#303)
- Refactor `avm-server` interface mod to new crate (#294)
- make clippy happy (#291)
