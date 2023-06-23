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
