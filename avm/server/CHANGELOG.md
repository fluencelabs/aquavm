# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

* The following workspace dependencies were updated
  * dependencies
    * avm-data-store bumped from 0.6.2 to 0.6.3
    * polyplets bumped from 0.3.2 to 0.3.3
    * avm-interface bumped from 0.28.4 to 0.28.5

* The following workspace dependencies were updated
  * dependencies
    * air-utils bumped from 0.1.1 to 0.2.0
    * avm-data-store bumped from 0.7.1 to 0.7.2
    * avm-interface bumped from 0.29.1 to 0.29.2

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.16.0 to 0.17.0
    * avm-data-store bumped from 0.7.4 to 0.7.5
    * avm-interface bumped from 0.30.0 to 0.31.0

## [0.38.1](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.38.0...avm-server-v0.38.1) (2024-07-22)


### Bug Fixes

* **avm-server:** remove unused AVM code ([#848](https://github.com/fluencelabs/aquavm/issues/848)) ([50b23c6](https://github.com/fluencelabs/aquavm/commit/50b23c6d9f5746acbddbbe93e3d1c02d7582a56a))

## [0.38.0](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.37.0...avm-server-v0.38.0) (2024-04-15)


### ⚠ BREAKING CHANGES

* **runtime:** switch to async marine ([#815](https://github.com/fluencelabs/aquavm/issues/815))

### Features

* **runtime:** switch to async marine ([#815](https://github.com/fluencelabs/aquavm/issues/815)) ([bf9414e](https://github.com/fluencelabs/aquavm/commit/bf9414e8d4b38f6e487ae1f0f0314e8f6a166824))

## [0.37.0](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.36.0...avm-server-v0.37.0) (2024-02-22)


### ⚠ BREAKING CHANGES

* update marine-rs-sdk to 0.14.0, remove all of json_path remains ([#820](https://github.com/fluencelabs/aquavm/issues/820))

### Features

* update marine-rs-sdk to 0.14.0, remove all of json_path remains ([#820](https://github.com/fluencelabs/aquavm/issues/820)) ([08e8547](https://github.com/fluencelabs/aquavm/commit/08e85478b4716f2ae5f57bc57dcb5d1df63f1b5d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.18.0 to 0.19.0
    * avm-data-store bumped from 0.7.8 to 0.7.9
    * polyplets bumped from 0.6.0 to 0.7.0
    * avm-interface bumped from 0.32.0 to 0.32.1

## [0.36.0](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.35.0...avm-server-v0.36.0) (2024-02-20)


### ⚠ BREAKING CHANGES

* **preparation,memory:** AquaVM preparation step now checks input arguments sizes [fixes VM-425]

### Features

* **preparation,memory:** AquaVM preparation step now checks input arguments sizes [fixes VM-425] ([5afd5cb](https://github.com/fluencelabs/aquavm/commit/5afd5cb3a14753077fbc1aab7e31532054a9f45f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.17.2 to 0.18.0
    * air-utils bumped from 0.2.0 to 0.3.0
    * avm-data-store bumped from 0.7.7 to 0.7.8
    * avm-interface bumped from 0.31.2 to 0.32.0

## [0.35.0](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.34.2...avm-server-v0.35.0) (2024-01-24)


### ⚠ BREAKING CHANGES

* **deps:** update to marine runtime with memory limits and wasmtime  ([#768](https://github.com/fluencelabs/aquavm/issues/768))

### Features

* **deps:** update to marine runtime with memory limits and wasmtime  ([#768](https://github.com/fluencelabs/aquavm/issues/768)) ([3375c7a](https://github.com/fluencelabs/aquavm/commit/3375c7a3b6b029ab5859ff00c1554abc8597542b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.17.1 to 0.17.2
    * avm-data-store bumped from 0.7.6 to 0.7.7
    * avm-interface bumped from 0.31.1 to 0.31.2

## [0.34.2](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.34.1...avm-server-v0.34.2) (2024-01-11)


### Features

* **air,air-cli:** pretty-printing binary interpreter data ([#794](https://github.com/fluencelabs/aquavm/issues/794)) ([d6b1da9](https://github.com/fluencelabs/aquavm/commit/d6b1da9bdc1197e72ef24051293fd06d3842f318))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.17.0 to 0.17.1
    * avm-data-store bumped from 0.7.5 to 0.7.6
    * polyplets bumped from 0.5.2 to 0.6.0
    * avm-interface bumped from 0.31.0 to 0.31.1

## [0.34.0](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.33.4...avm-server-v0.34.0) (2024-01-03)


### ⚠ BREAKING CHANGES

* **data:** flexible serialization formats ([#757](https://github.com/fluencelabs/aquavm/issues/757))

### Features

* **data:** flexible serialization formats ([#757](https://github.com/fluencelabs/aquavm/issues/757)) ([771d42d](https://github.com/fluencelabs/aquavm/commit/771d42dec43d3081621897edda3735768fd9ff71))


### Bug Fixes

* **deps:** update rust crate fluence-keypair to 0.10.4 ([#752](https://github.com/fluencelabs/aquavm/issues/752)) ([c9a0b87](https://github.com/fluencelabs/aquavm/commit/c9a0b87a4cefa3509b040c24d23cca37757fc030))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.15.2 to 0.16.0
    * avm-data-store bumped from 0.7.3 to 0.7.4
    * polyplets bumped from 0.5.1 to 0.5.2
    * avm-interface bumped from 0.29.3 to 0.30.0

## [0.33.4](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.33.3...avm-server-v0.33.4) (2023-12-12)


### Bug Fixes

* **avm-server:** make avm_server::RunnerError a public type ([#764](https://github.com/fluencelabs/aquavm/issues/764)) ([2c78fd5](https://github.com/fluencelabs/aquavm/commit/2c78fd5f7a8581ed006d392b12ca0143f9923a86))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.15.1 to 0.15.2
    * avm-data-store bumped from 0.7.2 to 0.7.3
    * avm-interface bumped from 0.29.2 to 0.29.3

## [0.33.3](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.33.2...avm-server-v0.33.3) (2023-10-26)


### Bug Fixes

* **deps:** update rust crate fluence-keypair to 0.10.3 ([#620](https://github.com/fluencelabs/aquavm/issues/620)) ([88e7dba](https://github.com/fluencelabs/aquavm/commit/88e7dba5f2ed6cf930f9bae52ad6dee7fa9e4ed0))

## [0.33.1](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.33.0...avm-server-v0.33.1) (2023-09-21)


### Bug Fixes

* **deps:** update rust crate marine-rs-sdk to 0.10.0 ([#640](https://github.com/fluencelabs/aquavm/issues/640)) ([b713e44](https://github.com/fluencelabs/aquavm/commit/b713e447fca38e0877a6c0e56bf91880f02bf9e4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.15.0 to 0.15.1
    * avm-data-store bumped from 0.7.0 to 0.7.1
    * polyplets bumped from 0.5.0 to 0.5.1
    * avm-interface bumped from 0.29.0 to 0.29.1

## [0.33.0](https://github.com/fluencelabs/aquavm/compare/avm-server-v0.32.2...avm-server-v0.33.0) (2023-08-17)


### ⚠ BREAKING CHANGES

* update marine-rs-sdk minor version

### Features

* **aquavm-air-cli:** `--near` execution mode [fixes VM-322] ([#672](https://github.com/fluencelabs/aquavm/issues/672)) ([0e80ee7](https://github.com/fluencelabs/aquavm/commit/0e80ee7908913fc896369ff7e00d65eeaf5d9f22))
* update marine-rs-sdk minor version ([4b4e3bd](https://github.com/fluencelabs/aquavm/commit/4b4e3bde839d1167ea559d49b183d1a76bc93439))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.14.0 to 0.15.0
    * polyplets bumped from 0.4.1 to 0.5.0

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
