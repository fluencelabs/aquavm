# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
