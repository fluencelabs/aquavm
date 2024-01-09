# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.12.1 to 0.13.0

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.13.0 to 0.14.0
    * air-utils bumped from 0.1.0 to 0.1.1

* The following workspace dependencies were updated
  * dependencies
    * polyplets bumped from 0.3.2 to 0.3.3

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.15.0 to 0.15.1
    * polyplets bumped from 0.5.0 to 0.5.1

* The following workspace dependencies were updated
  * dependencies
    * air-utils bumped from 0.1.1 to 0.2.0

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.15.1 to 0.15.2

## [0.31.0](https://github.com/fluencelabs/aquavm/compare/avm-interface-v0.30.0...avm-interface-v0.31.0) (2024-01-09)


### ⚠ BREAKING CHANGES

* **interface:** Use MessagePack for calls ([#780](https://github.com/fluencelabs/aquavm/issues/780))

### Features

* **interface:** Use MessagePack for calls ([#780](https://github.com/fluencelabs/aquavm/issues/780)) ([325eea7](https://github.com/fluencelabs/aquavm/commit/325eea7e9130e236b4e84ebb883632becffa28b5))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.16.0 to 0.17.0

## [0.30.0](https://github.com/fluencelabs/aquavm/compare/avm-interface-v0.29.3...avm-interface-v0.30.0) (2024-01-03)


### ⚠ BREAKING CHANGES

* **data:** flexible serialization formats ([#757](https://github.com/fluencelabs/aquavm/issues/757))

### Features

* **data:** flexible serialization formats ([#757](https://github.com/fluencelabs/aquavm/issues/757)) ([771d42d](https://github.com/fluencelabs/aquavm/commit/771d42dec43d3081621897edda3735768fd9ff71))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.15.2 to 0.16.0
    * polyplets bumped from 0.5.1 to 0.5.2

## [0.29.0](https://github.com/fluencelabs/aquavm/compare/avm-interface-v0.28.5...avm-interface-v0.29.0) (2023-08-17)


### ⚠ BREAKING CHANGES

* update marine-rs-sdk minor version

### Features

* update marine-rs-sdk minor version ([4b4e3bd](https://github.com/fluencelabs/aquavm/commit/4b4e3bde839d1167ea559d49b183d1a76bc93439))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.14.0 to 0.15.0
    * polyplets bumped from 0.4.1 to 0.5.0

## [0.28.2](https://github.com/fluencelabs/aquavm/compare/avm-interface-v0.28.1...avm-interface-v0.28.2) (2023-02-21)


### ⚠ BREAKING CHANGES

* **avm:** improve anomaly detection

### Features

* **avm:** improve anomaly detection ([5e6863d](https://github.com/fluencelabs/aquavm/commit/5e6863d4d59684d4f2b509ece6e597831e648f05))


### Bug Fixes

* **avm-interface:** minor code fix ([#482](https://github.com/fluencelabs/aquavm/issues/482)) ([a1f7a5c](https://github.com/fluencelabs/aquavm/commit/a1f7a5ce74b5002f3283494164a3d57fdd1cbd80))

## [Unreleased]

## [0.28.1] - 2022-12-26

### Changed

- Bump version of `avm-interpreter-interface` to 0.12.1:
  it has removed a new `cid` field in the `InterpreterOutcome`.

## [0.28.0] - 2022-12-21

### Changed

- Bump version of `avm-interpreter-interface` to 0.12.0:
  it has a new `cid` field in the `InterpreterOutcome`.

## [0.27.0] - 2022-11-22

### Added

- Add `current_peer_id` field to the `ParticleParameters`

### Changed

- `ParticleParameters` now has only single lifetime parameter

## [0.26.1] - 2022-09-13

### Fixed
- fix clippy warnings (#319)

### Other
- Update all non-major Rust dependencies (#309)
- Get rid of unsafe code in the interpreter (#303)
