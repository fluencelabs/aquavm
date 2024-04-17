# Changelog

## [0.63.0](https://github.com/fluencelabs/aquavm/compare/air-v0.62.0...air-v0.63.0) (2024-04-15)


### ⚠ BREAKING CHANGES

* **runtime:** switch to async marine ([#815](https://github.com/fluencelabs/aquavm/issues/815))

### Features

* **runtime:** switch to async marine ([#815](https://github.com/fluencelabs/aquavm/issues/815)) ([bf9414e](https://github.com/fluencelabs/aquavm/commit/bf9414e8d4b38f6e487ae1f0f0314e8f6a166824))

## [0.62.0](https://github.com/fluencelabs/aquavm/compare/air-v0.61.0...air-v0.62.0) (2024-02-22)


### ⚠ BREAKING CHANGES

* Rust channel update dedicated to ENIAC Day

### Features

* Rust channel update dedicated to ENIAC Day ([bab0c00](https://github.com/fluencelabs/aquavm/commit/bab0c002f5200099ab07a8fccf12f8ca7a260b87))

## [0.61.0](https://github.com/fluencelabs/aquavm/compare/air-v0.60.0...air-v0.61.0) (2024-02-22)


### ⚠ BREAKING CHANGES

* update marine-rs-sdk to 0.14.0, remove all of json_path remains ([#820](https://github.com/fluencelabs/aquavm/issues/820))

### Features

* update marine-rs-sdk to 0.14.0, remove all of json_path remains ([#820](https://github.com/fluencelabs/aquavm/issues/820)) ([08e8547](https://github.com/fluencelabs/aquavm/commit/08e85478b4716f2ae5f57bc57dcb5d1df63f1b5d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.11.2 to 0.12.0
    * air-execution-info-collector bumped from 0.7.13 to 0.7.14
    * air-interpreter-data bumped from 0.17.1 to 0.17.2
    * air-interpreter-interface bumped from 0.18.0 to 0.19.0
    * air-trace-handler bumped from 0.5.11 to 0.5.12
    * polyplets bumped from 0.6.0 to 0.7.0

## [0.60.0](https://github.com/fluencelabs/aquavm/compare/air-v0.59.0...air-v0.60.0) (2024-02-20)


### ⚠ BREAKING CHANGES

* **preparation,memory:** AquaVM preparation step now checks input arguments sizes [fixes VM-425]

### Features

* **execution-engine:** Rc-based JSON value ([#813](https://github.com/fluencelabs/aquavm/issues/813)) ([0d53f2b](https://github.com/fluencelabs/aquavm/commit/0d53f2bab1a09ae781bf22da6546e750e6172aa7))
* **preparation,memory:** AquaVM preparation step now checks input arguments sizes [fixes VM-425] ([5afd5cb](https://github.com/fluencelabs/aquavm/commit/5afd5cb3a14753077fbc1aab7e31532054a9f45f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.11.1 to 0.11.2
    * air-execution-info-collector bumped from 0.7.12 to 0.7.13
    * air-interpreter-data bumped from 0.17.0 to 0.17.1
    * air-interpreter-interface bumped from 0.17.2 to 0.18.0
    * air-trace-handler bumped from 0.5.10 to 0.5.11
    * air-utils bumped from 0.2.0 to 0.3.0

## [0.59.0](https://github.com/fluencelabs/aquavm/compare/air-v0.58.0...air-v0.59.0) (2024-01-24)


### ⚠ BREAKING CHANGES

* **deps:** update to marine runtime with memory limits and wasmtime  ([#768](https://github.com/fluencelabs/aquavm/issues/768))

### Features

* **deps:** update to marine runtime with memory limits and wasmtime  ([#768](https://github.com/fluencelabs/aquavm/issues/768)) ([3375c7a](https://github.com/fluencelabs/aquavm/commit/3375c7a3b6b029ab5859ff00c1554abc8597542b))


### Bug Fixes

* **deps:** update rust crate marine-rs-sdk to 0.10.3 ([#782](https://github.com/fluencelabs/aquavm/issues/782)) ([a33e853](https://github.com/fluencelabs/aquavm/commit/a33e8538123de71ea30f88ee5b40cda88f968707))
* **performance:** avoiding particle data printout in errors ([6c1cb28](https://github.com/fluencelabs/aquavm/commit/6c1cb289cc8ed7ba380d134cff1aec8b54c092ec))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.17.1 to 0.17.2

## [0.58.0](https://github.com/fluencelabs/aquavm/compare/air-v0.57.0...air-v0.58.0) (2024-01-11)


### ⚠ BREAKING CHANGES

* **data:** Rkyv for `InterprerterData` ([#783](https://github.com/fluencelabs/aquavm/issues/783))

### Features

* **air,air-cli:** pretty-printing binary interpreter data ([#794](https://github.com/fluencelabs/aquavm/issues/794)) ([d6b1da9](https://github.com/fluencelabs/aquavm/commit/d6b1da9bdc1197e72ef24051293fd06d3842f318))
* **data:** Rkyv for `InterprerterData` ([#783](https://github.com/fluencelabs/aquavm/issues/783)) ([2e0b54c](https://github.com/fluencelabs/aquavm/commit/2e0b54c2d415a27d2111587b850e981d8a8bcae2))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-cid bumped from 0.8.0 to 0.9.0
    * air-interpreter-data bumped from 0.16.0 to 0.17.0
    * air-interpreter-signatures bumped from 0.1.6 to 0.1.7
    * air-interpreter-interface bumped from 0.17.0 to 0.17.1
    * air-trace-handler bumped from 0.5.9 to 0.5.10
    * polyplets bumped from 0.5.2 to 0.6.0

## [0.57.0](https://github.com/fluencelabs/aquavm/compare/air-v0.56.0...air-v0.57.0) (2024-01-09)


### ⚠ BREAKING CHANGES

* **interface:** Use MessagePack for calls ([#780](https://github.com/fluencelabs/aquavm/issues/780))

### Features

* **interface:** Use MessagePack for calls ([#780](https://github.com/fluencelabs/aquavm/issues/780)) ([325eea7](https://github.com/fluencelabs/aquavm/commit/325eea7e9130e236b4e84ebb883632becffa28b5))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.16.0 to 0.17.0

## [0.56.0](https://github.com/fluencelabs/aquavm/compare/air-v0.55.0...air-v0.56.0) (2024-01-03)


### ⚠ BREAKING CHANGES

* **data:** values are binary blobs ([#775](https://github.com/fluencelabs/aquavm/issues/775))
* **data:** flexible serialization formats ([#757](https://github.com/fluencelabs/aquavm/issues/757))

### Features

* **air-parser:** VM-347 After next validator fold check ([#774](https://github.com/fluencelabs/aquavm/issues/774)) ([c6f157a](https://github.com/fluencelabs/aquavm/commit/c6f157a6e3ee10fa9209b1fa4a305040ce876d00))
* **data:** flexible serialization formats ([#757](https://github.com/fluencelabs/aquavm/issues/757)) ([771d42d](https://github.com/fluencelabs/aquavm/commit/771d42dec43d3081621897edda3735768fd9ff71))
* **data:** values are binary blobs ([#775](https://github.com/fluencelabs/aquavm/issues/775)) ([f1c7b43](https://github.com/fluencelabs/aquavm/commit/f1c7b43a1ee5cfd2793eb92a2a00ef1a4b185384))


### Bug Fixes

* **deps:** update rust crate fluence-keypair to 0.10.4 ([#752](https://github.com/fluencelabs/aquavm/issues/752)) ([c9a0b87](https://github.com/fluencelabs/aquavm/commit/c9a0b87a4cefa3509b040c24d23cca37757fc030))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.11.0 to 0.11.1
    * air-execution-info-collector bumped from 0.7.11 to 0.7.12
    * air-interpreter-cid bumped from 0.7.0 to 0.8.0
    * air-interpreter-data bumped from 0.15.0 to 0.16.0
    * air-interpreter-signatures bumped from 0.1.5 to 0.1.6
    * air-interpreter-interface bumped from 0.15.2 to 0.16.0
    * air-trace-handler bumped from 0.5.8 to 0.5.9
    * polyplets bumped from 0.5.1 to 0.5.2

## [0.55.0](https://github.com/fluencelabs/aquavm/compare/air-v0.54.0...air-v0.55.0) (2023-12-12)


### ⚠ BREAKING CHANGES

* **air-parser:** optimize Instruction type layout ([#767](https://github.com/fluencelabs/aquavm/issues/767))
* **interpreter-cid,interpreter-data:** Support for multiple hash types in CID verification ([#722](https://github.com/fluencelabs/aquavm/issues/722))
* **interpreter-data:** allow only deterministic signature algorithms ([#734](https://github.com/fluencelabs/aquavm/issues/734))

### Features

* **air-parser:** optimize Instruction type layout ([#767](https://github.com/fluencelabs/aquavm/issues/767)) ([1673cdf](https://github.com/fluencelabs/aquavm/commit/1673cdf06c1fcdd9d06789b0d9d76e71d1625fea))
* **interpreter-cid,interpreter-data:** Support for multiple hash types in CID verification ([#722](https://github.com/fluencelabs/aquavm/issues/722)) ([524c302](https://github.com/fluencelabs/aquavm/commit/524c30243bc544d5e265d9c6c7d1119a447202af))
* **interpreter-data:** allow only deterministic signature algorithms ([#734](https://github.com/fluencelabs/aquavm/issues/734)) ([15ce40a](https://github.com/fluencelabs/aquavm/commit/15ce40a1cd3271feb294666a1ef26d00282eb780))


### Bug Fixes

* **deps:** update rust crate marine-rs-sdk to 0.10.2 ([#733](https://github.com/fluencelabs/aquavm/issues/733)) ([05fda3e](https://github.com/fluencelabs/aquavm/commit/05fda3ee16d5d15e7af542a0d69d998d17827c15))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.10.0 to 0.11.0
    * air-execution-info-collector bumped from 0.7.10 to 0.7.11
    * air-interpreter-cid bumped from 0.6.0 to 0.7.0
    * air-interpreter-data bumped from 0.14.0 to 0.15.0
    * air-interpreter-signatures bumped from 0.1.4 to 0.1.5
    * air-interpreter-interface bumped from 0.15.1 to 0.15.2
    * air-trace-handler bumped from 0.5.7 to 0.5.8

## [0.54.0](https://github.com/fluencelabs/aquavm/compare/air-v0.53.0...air-v0.54.0) (2023-10-26)


### ⚠ BREAKING CHANGES

* **interpreter-cid:** use Blake3 for CIDs ([#729](https://github.com/fluencelabs/aquavm/issues/729))

### Features

* **interpreter-cid:** use Blake3 for CIDs ([#729](https://github.com/fluencelabs/aquavm/issues/729)) ([776d81a](https://github.com/fluencelabs/aquavm/commit/776d81a1dba2379e4019dc6bf851ae8396550d66))


### Bug Fixes

* **deps:** update rust crate fluence-keypair to 0.10.3 ([#620](https://github.com/fluencelabs/aquavm/issues/620)) ([88e7dba](https://github.com/fluencelabs/aquavm/commit/88e7dba5f2ed6cf930f9bae52ad6dee7fa9e4ed0))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-cid bumped from 0.5.0 to 0.6.0
    * air-interpreter-data bumped from 0.13.0 to 0.14.0
    * air-interpreter-signatures bumped from 0.1.3 to 0.1.4
    * air-trace-handler bumped from 0.5.6 to 0.5.7

## [0.53.0](https://github.com/fluencelabs/aquavm/compare/air-v0.52.0...air-v0.53.0) (2023-10-16)


### ⚠ BREAKING CHANGES

* **execution-engine,test-utils,interpreter-data,interpreter-cid:** Rc into CID ([#718](https://github.com/fluencelabs/aquavm/issues/718))

### Features

* **execution-engine,test-utils,interpreter-data,interpreter-cid:** Rc into CID ([#718](https://github.com/fluencelabs/aquavm/issues/718)) ([c2108e0](https://github.com/fluencelabs/aquavm/commit/c2108e0fa09ea83854bb48c640e0cf23883a0bd0))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-cid bumped from 0.4.0 to 0.5.0
    * air-interpreter-data bumped from 0.12.1 to 0.13.0
    * air-interpreter-signatures bumped from 0.1.2 to 0.1.3
    * air-trace-handler bumped from 0.5.5 to 0.5.6

## [0.52.0](https://github.com/fluencelabs/aquavm/compare/air-v0.51.0...air-v0.52.0) (2023-10-16)


### ⚠ BREAKING CHANGES

* **execution-engine:** intro farewell_if_error_macro ([#719](https://github.com/fluencelabs/aquavm/issues/719))

### Features

* **execution-engine:** intro farewell_if_error_macro ([#719](https://github.com/fluencelabs/aquavm/issues/719)) ([cdcb86c](https://github.com/fluencelabs/aquavm/commit/cdcb86cb554d6462e0a1a50b12aef6571669a7b1))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.12.0 to 0.12.1
    * air-trace-handler bumped from 0.5.4 to 0.5.5
    * air-utils bumped from 0.1.1 to 0.2.0

## [0.51.0](https://github.com/fluencelabs/aquavm/compare/air-v0.50.0...air-v0.51.0) (2023-10-13)


### ⚠ BREAKING CHANGES

* **aquavm-air:** signature checking ([#607](https://github.com/fluencelabs/aquavm/issues/607))

### Features

* **aquavm-air:** signature checking ([#607](https://github.com/fluencelabs/aquavm/issues/607)) ([8a07613](https://github.com/fluencelabs/aquavm/commit/8a076130274c0500025e5c2ea74ec57e4c455971))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-cid bumped from 0.3.0 to 0.4.0
    * air-interpreter-data bumped from 0.11.3 to 0.12.0
    * air-interpreter-signatures bumped from 0.1.1 to 0.1.2
    * air-trace-handler bumped from 0.5.3 to 0.5.4

## [0.50.0](https://github.com/fluencelabs/aquavm/compare/air-v0.49.0...air-v0.50.0) (2023-10-13)


### ⚠ BREAKING CHANGES

* **execution-engine:** map with a lens now returns an appropriate tetraplet [fixes VM-331] ([#706](https://github.com/fluencelabs/aquavm/issues/706))

### Bug Fixes

* **execution-engine:** map with a lens now returns an appropriate tetraplet [fixes VM-331] ([#706](https://github.com/fluencelabs/aquavm/issues/706)) ([ea80f11](https://github.com/fluencelabs/aquavm/commit/ea80f117a0aaba600a1c278c10cc658d392cc7c5))

## [0.49.0](https://github.com/fluencelabs/aquavm/compare/air-v0.48.0...air-v0.49.0) (2023-10-13)


### ⚠ BREAKING CHANGES

* **execution-engine:** fail :error: now bubbles the original error up [fixes VM-342] ([#714](https://github.com/fluencelabs/aquavm/issues/714))
* **execution-engine:** canon map lens access now returns a correct number of kvpairs in a canon stream ([#708](https://github.com/fluencelabs/aquavm/issues/708))

### Features

* **execution-engine:** fail :error: now bubbles the original error up [fixes VM-342] ([#714](https://github.com/fluencelabs/aquavm/issues/714)) ([98870c2](https://github.com/fluencelabs/aquavm/commit/98870c2ff9215914270186db8b8e2015bd54a9d5))


### Bug Fixes

* **execution-engine:** canon map lens access now returns a correct number of kvpairs in a canon stream ([#708](https://github.com/fluencelabs/aquavm/issues/708)) ([f4caa53](https://github.com/fluencelabs/aquavm/commit/f4caa538e46d8c9fab564c4f3b9296e0ae2bd1d3))
* **execution-engine:** ExecutionCtx fmt now prints stream maps out ([#705](https://github.com/fluencelabs/aquavm/issues/705)) ([c52a36c](https://github.com/fluencelabs/aquavm/commit/c52a36c7840d9c6a50223f9313b9a49d4b702924))

## [0.48.0](https://github.com/fluencelabs/aquavm/compare/air-v0.47.0...air-v0.48.0) (2023-09-21)


### ⚠ BREAKING CHANGES

* **execution-engine:** this adds a join behavior for key and value pair used by ap inserting into a map [fixes VM-337] ([#701](https://github.com/fluencelabs/aquavm/issues/701))
* **execution-engine:** this patch prohibits error code = 0 ([#702](https://github.com/fluencelabs/aquavm/issues/702))

### Features

* **execution-engine:** join behavior for canon ([#697](https://github.com/fluencelabs/aquavm/issues/697)) ([4e72abe](https://github.com/fluencelabs/aquavm/commit/4e72abe9a72cd6bfd6a9b09579d5aa627ed25212))
* **execution-engine:** this patch prohibits error code = 0 ([#702](https://github.com/fluencelabs/aquavm/issues/702)) ([45035cc](https://github.com/fluencelabs/aquavm/commit/45035ccff515344ee8c2dc63f172f00637226778))
* **parser,execution-engine:** allow :error: in fail ([#696](https://github.com/fluencelabs/aquavm/issues/696)) ([bd80a12](https://github.com/fluencelabs/aquavm/commit/bd80a127eaab39f1ba02740e3e67d69cb36a699c))


### Bug Fixes

* **deps:** update rust crate marine-rs-sdk to 0.10.0 ([#640](https://github.com/fluencelabs/aquavm/issues/640)) ([b713e44](https://github.com/fluencelabs/aquavm/commit/b713e447fca38e0877a6c0e56bf91880f02bf9e4))
* **execution-engine:** this adds a join behavior for key and value pair used by ap inserting into a map [fixes VM-337] ([#701](https://github.com/fluencelabs/aquavm/issues/701)) ([3a9beed](https://github.com/fluencelabs/aquavm/commit/3a9beed3c5572eefc4aee194d58144d7b424627e))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.9.0 to 0.10.0
    * air-execution-info-collector bumped from 0.7.9 to 0.7.10
    * air-interpreter-data bumped from 0.11.2 to 0.11.3
    * air-interpreter-interface bumped from 0.15.0 to 0.15.1
    * air-trace-handler bumped from 0.5.2 to 0.5.3
    * polyplets bumped from 0.5.0 to 0.5.1

## [0.47.0](https://github.com/fluencelabs/aquavm/compare/air-v0.46.0...air-v0.47.0) (2023-09-07)


### ⚠ BREAKING CHANGES

* **execution-engine:** canon stream map support [fixes VM-301] ([#648](https://github.com/fluencelabs/aquavm/issues/648))

### Features

* **execution-engine:** canon stream map support [fixes VM-301] ([#648](https://github.com/fluencelabs/aquavm/issues/648)) ([b4cbf8f](https://github.com/fluencelabs/aquavm/commit/b4cbf8f621b77ba2031900f021bf20d0f27e34b8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.8.2 to 0.9.0
    * air-execution-info-collector bumped from 0.7.8 to 0.7.9
    * air-interpreter-data bumped from 0.11.1 to 0.11.2
    * air-trace-handler bumped from 0.5.1 to 0.5.2

## [0.46.0](https://github.com/fluencelabs/aquavm/compare/air-v0.45.1...air-v0.46.0) (2023-09-04)


### ⚠ BREAKING CHANGES

* **data:** bump minimal support version ([#691](https://github.com/fluencelabs/aquavm/issues/691))

### Features

* **data:** bump minimal support version ([#691](https://github.com/fluencelabs/aquavm/issues/691)) ([b13dd51](https://github.com/fluencelabs/aquavm/commit/b13dd515da3b88a8b65ed1ca60682055e227bad9))

## [0.45.1](https://github.com/fluencelabs/aquavm/compare/air-v0.45.0...air-v0.45.1) (2023-09-04)


### Features

* **execution-engine:** a new :error: runtime attribute according with FLIP-11 [fixes VM-329] ([#683](https://github.com/fluencelabs/aquavm/issues/683)) ([20afb79](https://github.com/fluencelabs/aquavm/commit/20afb79e3f345b83c367357171f1802ed2db0a66))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.8.1 to 0.8.2
    * air-execution-info-collector bumped from 0.7.7 to 0.7.8
    * air-interpreter-data bumped from 0.11.0 to 0.11.1
    * air-trace-handler bumped from 0.5.0 to 0.5.1

## [0.45.0](https://github.com/fluencelabs/aquavm/compare/air-v0.44.0...air-v0.45.0) (2023-08-31)


### ⚠ BREAKING CHANGES

* **execution-engine,interpreter-data:** insert state for canon join ([#682](https://github.com/fluencelabs/aquavm/issues/682))
* **execution-engine:** this introduces a hardcoded limit for a number of values in a stream [fixes VM-324]
* **execution-engine:** this introduces a hardcoded limit for a number of values in a stream [fixes VM-324] ([#677](https://github.com/fluencelabs/aquavm/issues/677))

### Features

* **execution-engine,interpreter-data:** insert state for canon join ([#682](https://github.com/fluencelabs/aquavm/issues/682)) ([2b636e8](https://github.com/fluencelabs/aquavm/commit/2b636e808ae1b1422d5cc57c6796f32d4663d37c))
* **execution-engine:** this introduces a hardcoded limit for a number of values in a stream [fixes VM-324] ([f943dd0](https://github.com/fluencelabs/aquavm/commit/f943dd00cd8e06546252b5caf04117869abe5b8a))
* **execution-engine:** this introduces a hardcoded limit for a number of values in a stream [fixes VM-324] ([#677](https://github.com/fluencelabs/aquavm/issues/677)) ([f943dd0](https://github.com/fluencelabs/aquavm/commit/f943dd00cd8e06546252b5caf04117869abe5b8a))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.10.0 to 0.11.0
    * air-trace-handler bumped from 0.4.0 to 0.5.0

## [0.44.0](https://github.com/fluencelabs/aquavm/compare/air-v0.43.1...air-v0.44.0) (2023-08-17)


### ⚠ BREAKING CHANGES

* **polyplets:** move SecurityTetraplets to marine-rs-sdk  ([#674](https://github.com/fluencelabs/aquavm/issues/674))
* update marine-rs-sdk minor version

### Features

* **aquavm-air-cli:** `--near` execution mode [fixes VM-322] ([#672](https://github.com/fluencelabs/aquavm/issues/672)) ([0e80ee7](https://github.com/fluencelabs/aquavm/commit/0e80ee7908913fc896369ff7e00d65eeaf5d9f22))
* **polyplets:** move SecurityTetraplets to marine-rs-sdk  ([#674](https://github.com/fluencelabs/aquavm/issues/674)) ([7a8a460](https://github.com/fluencelabs/aquavm/commit/7a8a46057297317e1b776b13d913b0d42ec0a9af))
* update marine-rs-sdk minor version ([4b4e3bd](https://github.com/fluencelabs/aquavm/commit/4b4e3bde839d1167ea559d49b183d1a76bc93439))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.14.0 to 0.15.0
    * polyplets bumped from 0.4.1 to 0.5.0

## [0.43.1](https://github.com/fluencelabs/aquavm/compare/air-v0.43.0...air-v0.43.1) (2023-08-07)


### Features

* **execution-engine:** rename boxed_values into value_types ([#664](https://github.com/fluencelabs/aquavm/issues/664)) ([20ae5ee](https://github.com/fluencelabs/aquavm/commit/20ae5eeeb9577f24bf27bcb74f17b92257d0e1a8))

## [0.43.0](https://github.com/fluencelabs/aquavm/compare/air-v0.42.0...air-v0.43.0) (2023-08-03)


### ⚠ BREAKING CHANGES

* **execution-engine:** update minimal interpreter version ([#649](https://github.com/fluencelabs/aquavm/issues/649))
* **execution-engine:** refactor streams [fixes VM-255] ([#621](https://github.com/fluencelabs/aquavm/issues/621))

### Features

* **execution-engine:** refactor streams [fixes VM-255] ([#621](https://github.com/fluencelabs/aquavm/issues/621)) ([eca52b7](https://github.com/fluencelabs/aquavm/commit/eca52b7191ef1bc5c4573c62412dc735d830c023))
* **execution-engine:** update minimal interpreter version ([#649](https://github.com/fluencelabs/aquavm/issues/649)) ([0655daa](https://github.com/fluencelabs/aquavm/commit/0655daa89d1105c6e786347f405d46d8e4d213ce))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.8.0 to 0.8.1
    * air-execution-info-collector bumped from 0.7.6 to 0.7.7
    * air-interpreter-data bumped from 0.8.1 to 0.9.0
    * air-trace-handler bumped from 0.2.2 to 0.3.0
    * polyplets bumped from 0.3.2 to 0.3.3

## [0.42.0](https://github.com/fluencelabs/aquavm/compare/air-v0.41.0...air-v0.42.0) (2023-07-16)


### ⚠ BREAKING CHANGES

* **execution-engine:** add error-code invariant check for match/mismatch ([#622](https://github.com/fluencelabs/aquavm/issues/622))
* **execution-engine:** %last_eror%.$.message and $.error_code now return an empty string and 0 correspondingly [fixes VM-313]
* **execution-engine:** unfefined %last_error% now returns an empty … ([#628](https://github.com/fluencelabs/aquavm/issues/628))
* **aquavm-air:** `ap` join behavior ([#631](https://github.com/fluencelabs/aquavm/issues/631))
* **execution-engine:** stream map to scalar conversion using canon instruction [fixes VM-294] ([#610](https://github.com/fluencelabs/aquavm/issues/610))

### Features

* **aquavm-air:** `ap` join behavior ([#631](https://github.com/fluencelabs/aquavm/issues/631)) ([75f5516](https://github.com/fluencelabs/aquavm/commit/75f5516c5803c256e329a7318632fbab13aea491)), closes [#632](https://github.com/fluencelabs/aquavm/issues/632)
* **execution-engine:** %last_eror%.$.message and $.error_code now return an empty string and 0 correspondingly [fixes VM-313] ([d195152](https://github.com/fluencelabs/aquavm/commit/d19515232043462e809d9cd6964042f69a77f4cf))
* **execution-engine:** add error-code invariant check for match/mismatch ([#622](https://github.com/fluencelabs/aquavm/issues/622)) ([33a9d9f](https://github.com/fluencelabs/aquavm/commit/33a9d9f32f84c5b31b59120f9da3c1624e1d5c27))
* **execution-engine:** Stream Map to Scalar conversion using canon instruction [fixes VM-294] ([fcb4c9d](https://github.com/fluencelabs/aquavm/commit/fcb4c9dab43d82e87f1d6f8a83b72f6891d37bef))
* **execution-engine:** stream map to scalar conversion using canon instruction [fixes VM-294] ([#610](https://github.com/fluencelabs/aquavm/issues/610)) ([fcb4c9d](https://github.com/fluencelabs/aquavm/commit/fcb4c9dab43d82e87f1d6f8a83b72f6891d37bef))
* **execution-engine:** unfefined %last_error% now returns an empty … ([#628](https://github.com/fluencelabs/aquavm/issues/628)) ([d195152](https://github.com/fluencelabs/aquavm/commit/d19515232043462e809d9cd6964042f69a77f4cf))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.7.5 to 0.8.0
    * air-execution-info-collector bumped from 0.7.5 to 0.7.6
    * air-interpreter-data bumped from 0.8.0 to 0.8.1
    * air-trace-handler bumped from 0.2.1 to 0.2.2

## [0.41.0](https://github.com/fluencelabs/aquavm/compare/air-v0.40.0...air-v0.41.0) (2023-06-23)


### ⚠ BREAKING CHANGES

* **testing-framework:** restore WASM test executor ([#609](https://github.com/fluencelabs/aquavm/issues/609))

### Miscellaneous Chores

* **testing-framework:** restore WASM test executor ([#609](https://github.com/fluencelabs/aquavm/issues/609)) ([c332cca](https://github.com/fluencelabs/aquavm/commit/c332cca6b75e804412e1f1cc51bdfe0580ea5fdd))

## [0.40.0](https://github.com/fluencelabs/aquavm/compare/air-v0.39.0...air-v0.40.0) (2023-06-22)


### ⚠ BREAKING CHANGES

* **avm-server,air-interpreter,aquavm-air:** secret key and particle ID arguments ([#593](https://github.com/fluencelabs/aquavm/issues/593))
* **aquavm-air,air-interpreter-signature,air-interpreter-data:** Peer signatures ([#598](https://github.com/fluencelabs/aquavm/issues/598))
* **aquavm-air-cli:** Usability enhancements ([#540](https://github.com/fluencelabs/aquavm/issues/540))

### Features

* **air:** introduce explicit types for generation numbers ([#530](https://github.com/fluencelabs/aquavm/issues/530)) ([d62fa6f](https://github.com/fluencelabs/aquavm/commit/d62fa6fe6006e59d63d30549074e7b30f80bf687))
* **aquavm-air-cli:** Usability enhancements ([#540](https://github.com/fluencelabs/aquavm/issues/540)) ([73c1ba7](https://github.com/fluencelabs/aquavm/commit/73c1ba70cca9ba4f6e73880141e738d450565798))
* **aquavm-air,air-interpreter-signature,air-interpreter-data:** Peer signatures ([#598](https://github.com/fluencelabs/aquavm/issues/598)) ([f8b734a](https://github.com/fluencelabs/aquavm/commit/f8b734abde8181cc2b2f11423f9d3bddd48f9fd1))
* **avm-server,air-interpreter,aquavm-air:** secret key and particle ID arguments ([#593](https://github.com/fluencelabs/aquavm/issues/593)) ([8ce8af3](https://github.com/fluencelabs/aquavm/commit/8ce8af38232de3f1ac359214386b895356550428))
* **execution-engine:** introduce resolver ([#574](https://github.com/fluencelabs/aquavm/issues/574)) ([a66541d](https://github.com/fluencelabs/aquavm/commit/a66541de497e0b2f0ed97631d63e35a25215bca3))
* **execution-engine:** remove stream jvaluable implementation ([#576](https://github.com/fluencelabs/aquavm/issues/576)) ([513d33a](https://github.com/fluencelabs/aquavm/commit/513d33a1c5faf972907bd402dfd6ad39cacd1eff))
* **execution-engine:** Stream Map initial support [fixes VM-283,VM-284] ([#592](https://github.com/fluencelabs/aquavm/issues/592)) ([9d7d34a](https://github.com/fluencelabs/aquavm/commit/9d7d34a452cb65e968ed68decc67f3bc523a5115))
* **execution-engine:** StreamMap initial support for ap and new instructions [fixes VM-283,VM-284] ([9d7d34a](https://github.com/fluencelabs/aquavm/commit/9d7d34a452cb65e968ed68decc67f3bc523a5115))
* **interpreter-data:** Introduce source information for `canon` data ([#577](https://github.com/fluencelabs/aquavm/issues/577)) ([1d98afe](https://github.com/fluencelabs/aquavm/commit/1d98afeb34b1ee45defc05995c8cf24021449f2b))
* **trace-handler:** sub/-trace len dedicated alias to replace usize [fixes VM-282] ([b480e01](https://github.com/fluencelabs/aquavm/commit/b480e018b4b69b088d4258497866c3b31774b6b1))
* **trace-handler:** TracePos becomes a wrapper for u32 alias [fixes VM-267] ([#544](https://github.com/fluencelabs/aquavm/issues/544)) ([658daf1](https://github.com/fluencelabs/aquavm/commit/658daf1d3f6e733c15a21afc40ddf468ed745d43))


### Bug Fixes

* **deps:** update rust crate marine-rs-sdk to 0.7.1 ([#568](https://github.com/fluencelabs/aquavm/issues/568)) ([648f297](https://github.com/fluencelabs/aquavm/commit/648f297a2badde312c88d3db9eec085170211aa6))
* **execution-engine:** Fold-over-scalar values' wrong lambda ([#578](https://github.com/fluencelabs/aquavm/issues/578)) ([88fd1f3](https://github.com/fluencelabs/aquavm/commit/88fd1f3095fc47862472baf30ff79964ec662b37))
* **execution-engine:** this removes an unused and impossible check ([#575](https://github.com/fluencelabs/aquavm/issues/575)) ([70f27f7](https://github.com/fluencelabs/aquavm/commit/70f27f7cb6e0ff21be9695a082b1fadf3a2dd05f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.7.4 to 0.7.5
    * air-execution-info-collector bumped from 0.7.4 to 0.7.5
    * air-interpreter-cid bumped from 0.2.0 to 0.3.0
    * air-interpreter-data bumped from 0.7.0 to 0.8.0
    * air-interpreter-signatures bumped from 0.1.0 to 0.1.1
    * air-interpreter-interface bumped from 0.13.0 to 0.14.0
    * air-trace-handler bumped from 0.2.0 to 0.2.1
    * air-utils bumped from 0.1.0 to 0.1.1

## [0.39.0](https://github.com/fluencelabs/aquavm/compare/air-v0.38.0...air-v0.39.0) (2023-03-21)


### ⚠ BREAKING CHANGES

* **interpreter-data:** 

### Features

* **interpreter-data:** New data format for calls ([#501](https://github.com/fluencelabs/aquavm/issues/501)) ([d502894](https://github.com/fluencelabs/aquavm/commit/d5028942e41e1ac47ce31e20b57c17895f543ac8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.6.4 to 0.7.0
    * air-trace-handler bumped from 0.1.3 to 0.2.0

## [0.38.0](https://github.com/fluencelabs/aquavm/compare/air-v0.37.1...air-v0.38.0) (2023-03-15)


### ⚠ BREAKING CHANGES

* **avm:** integrate Marine 0.26.0 ([#461](https://github.com/fluencelabs/aquavm/issues/461))

### Features

* **avm:** integrate Marine 0.26.0 ([#461](https://github.com/fluencelabs/aquavm/issues/461)) ([126d550](https://github.com/fluencelabs/aquavm/commit/126d5507c81a7f978ab9cf06c492b1092a336cf6))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-execution-info-collector bumped from 0.7.3 to 0.7.4
    * air-interpreter-data bumped from 0.6.3 to 0.6.4
    * air-interpreter-interface bumped from 0.12.1 to 0.13.0
    * air-trace-handler bumped from 0.1.2 to 0.1.3

## [0.37.1](https://github.com/fluencelabs/aquavm/compare/air-v0.37.0...air-v0.37.1) (2023-03-15)


### Features

* **tools:** merge some tools into the `air` CLI tool ([#509](https://github.com/fluencelabs/aquavm/issues/509)) ([79ac153](https://github.com/fluencelabs/aquavm/commit/79ac153f1dcfc0a77ec511c6e25285728312ad4c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.7.3 to 0.7.4
    * air-execution-info-collector bumped from 0.7.2 to 0.7.3
    * air-interpreter-data bumped from 0.6.2 to 0.6.3

## [0.37.0](https://github.com/fluencelabs/aquavm/compare/air-v0.36.0...air-v0.37.0) (2023-03-13)


### ⚠ BREAKING CHANGES

* **execution-engine:** make StreamDontHaveSuchGeneration uncatchable… ([#492](https://github.com/fluencelabs/aquavm/issues/492))

### Features

* **execution-engine:** make StreamDontHaveSuchGeneration uncatchable… ([#492](https://github.com/fluencelabs/aquavm/issues/492)) ([95b2d15](https://github.com/fluencelabs/aquavm/commit/95b2d154ff84caf5efe7a3960922d1d6c39a9ed2))


### Bug Fixes

* **air:** demote some AquaVM logging statements ([#505](https://github.com/fluencelabs/aquavm/issues/505)) ([271b74d](https://github.com/fluencelabs/aquavm/commit/271b74d8f2f1111dfb8393aa81c8f9c9e78ff4d7))
* **execution-engine:** negative tests for prepare_step and farewell_step [fixes VM-251] ([5813c80](https://github.com/fluencelabs/aquavm/commit/5813c80ca2561cb8121792c4123a3b7171b8e2c7))
* **execution-engine:** rename CallResultsNotEmpty into UnprocessedCallResult ([7f6a698](https://github.com/fluencelabs/aquavm/commit/7f6a69851f93f0a7abcc852913b6e7116fd35167))
* negative tests for prepare_step and farewell_step [fixes VM-251] ([#489](https://github.com/fluencelabs/aquavm/issues/489)) ([5813c80](https://github.com/fluencelabs/aquavm/commit/5813c80ca2561cb8121792c4123a3b7171b8e2c7))
* Rename CallResultsNotEmpty into UnprocessedCallResult ([#490](https://github.com/fluencelabs/aquavm/issues/490)) ([7f6a698](https://github.com/fluencelabs/aquavm/commit/7f6a69851f93f0a7abcc852913b6e7116fd35167))

## [0.36.0](https://github.com/fluencelabs/aquavm/compare/air-v0.35.4...air-v0.36.0) (2023-02-27)


### ⚠ BREAKING CHANGES

* **execution-engine:** make fold convergent wrt errors ([#351](https://github.com/fluencelabs/aquavm/issues/351))

### Features

* **execution-engine:** make fold convergent wrt errors ([#351](https://github.com/fluencelabs/aquavm/issues/351)) ([87f7e2f](https://github.com/fluencelabs/aquavm/commit/87f7e2f361891c84315f310967517ddb50773f8d))
* **tools:** VM-194 performance metering ([#440](https://github.com/fluencelabs/aquavm/issues/440)) ([5fdc8e6](https://github.com/fluencelabs/aquavm/commit/5fdc8e68ac67f502f8ece4d8a5935cf7d478d830))
* **trace-handler:** improve data deserialization version check ([#451](https://github.com/fluencelabs/aquavm/issues/451)) ([367546b](https://github.com/fluencelabs/aquavm/commit/367546b82cd5f133b956857bf48d279512b157b2))

## [0.35.1](https://github.com/fluencelabs/aquavm/compare/air-v0.35.0...air-v0.35.1) (2023-02-08)


### Features

* **tools:** VM-194 performance metering ([#440](https://github.com/fluencelabs/aquavm/issues/440)) ([5fdc8e6](https://github.com/fluencelabs/aquavm/commit/5fdc8e68ac67f502f8ece4d8a5935cf7d478d830))
* **trace-handler:** improve data deserialization version check ([#451](https://github.com/fluencelabs/aquavm/issues/451)) ([367546b](https://github.com/fluencelabs/aquavm/commit/367546b82cd5f133b956857bf48d279512b157b2))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.6.1 to 0.6.2
    * air-trace-handler bumped from 0.1.0 to 0.1.1
