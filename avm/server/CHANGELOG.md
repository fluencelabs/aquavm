# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.29.0](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.28.1...avm-server-v0.29.0) (2023-02-21)


### ⚠ BREAKING CHANGES

* **avm:** improve anomaly detection

### Features

* **avm:** improve anomaly detection ([5e6863d](https://github.com/fluencelabs/aquavm/commit/5e6863d4d59684d4f2b509ece6e597831e648f05))


### Bug Fixes

* **deps:** update rust crate marine-runtime to 0.24.1 ([#478](https://github.com/fluencelabs/aquavm/issues/478)) ([c408da8](https://github.com/fluencelabs/aquavm/commit/c408da884de9bc62c058dc0a1994dd13bc765fb0))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * avm-data-store bumped from 0.4.1 to 0.5.0
    * avm-interface bumped from 0.28.0 to 0.28.2

## [Unreleased]

## [0.28.1] - 2022-12-26

+ Update `avm-interface` version after `air-interpreter-interface` version bump.

## [0.28.0] - 2022-12-21

+ Update `avm-interface` version after `air-interpreter-interface` version bump.

## [0.27.0] - 2022-11-22

### Changed

- Move `current_peer_id` field/argument to the `ParticleParameters`.
  It is removed from both `AVMConfig` and `AVMRunner::new`, but added to `AVMRunner::call`/`AVMRunner::call_tracing`.
- `ParticleParameters` now has only single lifetime parameter
- Update `avm-interface` version

## [0.26.1] - 2022-09-13

### Other
- Update all non-major Rust dependencies (#323)
