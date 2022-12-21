# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
