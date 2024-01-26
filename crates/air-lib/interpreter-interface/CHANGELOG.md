# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.17.2](https://github.com/fluencelabs/aquavm/compare/air-interpreter-interface-v0.17.1...air-interpreter-interface-v0.17.2) (2024-01-24)


### Bug Fixes

* **deps:** update rust crate marine-rs-sdk to 0.10.3 ([#782](https://github.com/fluencelabs/aquavm/issues/782)) ([a33e853](https://github.com/fluencelabs/aquavm/commit/a33e8538123de71ea30f88ee5b40cda88f968707))

## [0.17.1](https://github.com/fluencelabs/aquavm/compare/air-interpreter-interface-v0.17.0...air-interpreter-interface-v0.17.1) (2024-01-11)


### Features

* **air,air-cli:** pretty-printing binary interpreter data ([#794](https://github.com/fluencelabs/aquavm/issues/794)) ([d6b1da9](https://github.com/fluencelabs/aquavm/commit/d6b1da9bdc1197e72ef24051293fd06d3842f318))

## [0.17.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-interface-v0.16.0...air-interpreter-interface-v0.17.0) (2024-01-09)


### ⚠ BREAKING CHANGES

* **interface:** Use MessagePack for calls ([#780](https://github.com/fluencelabs/aquavm/issues/780))

### Features

* **interface:** Use MessagePack for calls ([#780](https://github.com/fluencelabs/aquavm/issues/780)) ([325eea7](https://github.com/fluencelabs/aquavm/commit/325eea7e9130e236b4e84ebb883632becffa28b5))

## [0.16.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-interface-v0.15.2...air-interpreter-interface-v0.16.0) (2024-01-03)


### ⚠ BREAKING CHANGES

* **data:** flexible serialization formats ([#757](https://github.com/fluencelabs/aquavm/issues/757))

### Features

* **data:** flexible serialization formats ([#757](https://github.com/fluencelabs/aquavm/issues/757)) ([771d42d](https://github.com/fluencelabs/aquavm/commit/771d42dec43d3081621897edda3735768fd9ff71))


### Bug Fixes

* **deps:** update rust crate marine-call-parameters to 0.10.2 ([#732](https://github.com/fluencelabs/aquavm/issues/732)) ([763bbcb](https://github.com/fluencelabs/aquavm/commit/763bbcb663ba445ed19431929823e7bfcc6d910c))
* **deps:** update rust crate marine-call-parameters to 0.10.3 ([#781](https://github.com/fluencelabs/aquavm/issues/781)) ([518bb95](https://github.com/fluencelabs/aquavm/commit/518bb95a178ab1508d27b1fa71bd205ef05dea8e))

## [0.15.2](https://github.com/fluencelabs/aquavm/compare/air-interpreter-interface-v0.15.1...air-interpreter-interface-v0.15.2) (2023-12-12)


### Bug Fixes

* **deps:** update rust crate marine-rs-sdk to 0.10.2 ([#733](https://github.com/fluencelabs/aquavm/issues/733)) ([05fda3e](https://github.com/fluencelabs/aquavm/commit/05fda3ee16d5d15e7af542a0d69d998d17827c15))

## [0.15.1](https://github.com/fluencelabs/aquavm/compare/air-interpreter-interface-v0.15.0...air-interpreter-interface-v0.15.1) (2023-09-21)


### Bug Fixes

* **deps:** update rust crate marine-rs-sdk to 0.10.0 ([#640](https://github.com/fluencelabs/aquavm/issues/640)) ([b713e44](https://github.com/fluencelabs/aquavm/commit/b713e447fca38e0877a6c0e56bf91880f02bf9e4))

## [0.15.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-interface-v0.14.0...air-interpreter-interface-v0.15.0) (2023-08-17)


### ⚠ BREAKING CHANGES

* update marine-rs-sdk minor version

### Features

* update marine-rs-sdk minor version ([4b4e3bd](https://github.com/fluencelabs/aquavm/commit/4b4e3bde839d1167ea559d49b183d1a76bc93439))

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
