# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0 (2023-03-15)


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
