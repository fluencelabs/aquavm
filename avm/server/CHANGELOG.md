# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

* The following workspace dependencies were updated
  * dependencies
    * avm-data-store bumped from 0.6.2 to 0.6.3
    * polyplets bumped from 0.3.2 to 0.3.3
    * avm-interface bumped from 0.28.4 to 0.28.5

## [0.32.2](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.32.1...avm-server-v0.32.2) (2023-08-04)


### Bug Fixes

* **deps:** update rust crate marine-runtime to 0.28.0 ([#657](https://github.com/fluencelabs/aquavm/issues/657)) ([ee3c807](https://github.com/fluencelabs/aquavm/commit/ee3c8076dbef8f29b53f1e3a6641c19e396ad105))

## [0.32.0](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.31.0...avm-server-v0.32.0) (2023-06-22)


### ⚠ BREAKING CHANGES

* **avm-server,air-interpreter,aquavm-air:** secret key and particle ID arguments ([#593](https://github.com/fluencelabs/aquavm/issues/593))

### Features

* **avm-server,air-interpreter,aquavm-air:** secret key and particle ID arguments ([#593](https://github.com/fluencelabs/aquavm/issues/593)) ([8ce8af3](https://github.com/fluencelabs/aquavm/commit/8ce8af38232de3f1ac359214386b895356550428))


### Bug Fixes

* **deps:** update rust crate marine-runtime to 0.26.1 ([#546](https://github.com/fluencelabs/aquavm/issues/546)) ([76d263b](https://github.com/fluencelabs/aquavm/commit/76d263b4c80d908ffc8da35fbca9a8862359e6d3))
* **deps:** update rust crate marine-runtime to 0.26.3 ([#558](https://github.com/fluencelabs/aquavm/issues/558)) ([f5c61af](https://github.com/fluencelabs/aquavm/commit/f5c61af7e2da13cb189e3c47f262ac5ae09002a4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.13.0 to 0.14.0
    * air-utils bumped from 0.1.0 to 0.1.1
    * avm-data-store bumped from 0.6.1 to 0.6.2
    * avm-interface bumped from 0.28.3 to 0.28.4

## [0.31.0](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.30.1...avm-server-v0.31.0) (2023-03-15)


### ⚠ BREAKING CHANGES

* **avm:** integrate Marine 0.26.0 ([#461](https://github.com/fluencelabs/aquavm/issues/461))

### Features

* **avm:** integrate Marine 0.26.0 ([#461](https://github.com/fluencelabs/aquavm/issues/461)) ([126d550](https://github.com/fluencelabs/aquavm/commit/126d5507c81a7f978ab9cf06c492b1092a336cf6))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.12.1 to 0.13.0
    * avm-data-store bumped from 0.6.0 to 0.6.1
    * avm-interface bumped from 0.28.2 to 0.28.3

## [0.30.1](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.30.0...avm-server-v0.30.1) (2023-03-13)


### Bug Fixes

* **deps:** update rust crate marine-runtime to 0.25.0 ([#498](https://github.com/fluencelabs/aquavm/issues/498)) ([8c25dbe](https://github.com/fluencelabs/aquavm/commit/8c25dbe8f681f46cbfc5e914614b4f103f0f556a))

## [0.30.0](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.29.0...avm-server-v0.30.0) (2023-02-27)


### ⚠ BREAKING CHANGES

* **data_store:** use particle_id + current_peer_id as prev_data key in DataStore ([#485](https://github.com/fluencelabs/aquavm/issues/485))

### Bug Fixes

* **data_store:** use particle_id + current_peer_id as prev_data key in DataStore ([#485](https://github.com/fluencelabs/aquavm/issues/485)) ([36e1c87](https://github.com/fluencelabs/aquavm/commit/36e1c8762c1888f375adacc21907d98a811d28d9))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * avm-data-store bumped from 0.5.0 to 0.6.0

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
