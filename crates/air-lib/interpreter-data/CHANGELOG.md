# Changelog

* The following workspace dependencies were updated
  * dependencies
    * air-parser bumped from 0.7.2 to 0.7.3

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.7.5 to 0.8.0

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.8.1 to 0.8.2

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.8.2 to 0.9.0

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.9.0 to 0.10.0
    * polyplets bumped from 0.5.0 to 0.5.1

* The following workspace dependencies were updated
  * dependencies
    * air-utils bumped from 0.1.1 to 0.2.0

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.11.2 to 0.12.0
    * polyplets bumped from 0.6.0 to 0.7.0

## [0.17.1](https://github.com/fluencelabs/aquavm/compare/air-interpreter-data-v0.17.0...air-interpreter-data-v0.17.1) (2024-02-20)


### Features

* **execution-engine:** Rc-based JSON value ([#813](https://github.com/fluencelabs/aquavm/issues/813)) ([0d53f2b](https://github.com/fluencelabs/aquavm/commit/0d53f2bab1a09ae781bf22da6546e750e6172aa7))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-utils bumped from 0.2.0 to 0.3.0
    * aquavm-air-parser bumped from 0.11.1 to 0.11.2

## [0.17.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-data-v0.16.0...air-interpreter-data-v0.17.0) (2024-01-11)


### ⚠ BREAKING CHANGES

* **data:** Rkyv for `InterprerterData` ([#783](https://github.com/fluencelabs/aquavm/issues/783))

### Features

* **data:** Rkyv for `InterprerterData` ([#783](https://github.com/fluencelabs/aquavm/issues/783)) ([2e0b54c](https://github.com/fluencelabs/aquavm/commit/2e0b54c2d415a27d2111587b850e981d8a8bcae2))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-cid bumped from 0.8.0 to 0.9.0
    * air-interpreter-signatures bumped from 0.1.6 to 0.1.7
    * polyplets bumped from 0.5.2 to 0.6.0

## [0.16.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-data-v0.15.0...air-interpreter-data-v0.16.0) (2024-01-03)


### ⚠ BREAKING CHANGES

* **data:** values are binary blobs ([#775](https://github.com/fluencelabs/aquavm/issues/775))
* **data:** flexible serialization formats ([#757](https://github.com/fluencelabs/aquavm/issues/757))

### Features

* **data:** flexible serialization formats ([#757](https://github.com/fluencelabs/aquavm/issues/757)) ([771d42d](https://github.com/fluencelabs/aquavm/commit/771d42dec43d3081621897edda3735768fd9ff71))
* **data:** values are binary blobs ([#775](https://github.com/fluencelabs/aquavm/issues/775)) ([f1c7b43](https://github.com/fluencelabs/aquavm/commit/f1c7b43a1ee5cfd2793eb92a2a00ef1a4b185384))


### Bug Fixes

* **deps:** update rust crate fluence-keypair to 0.10.4 ([#752](https://github.com/fluencelabs/aquavm/issues/752)) ([c9a0b87](https://github.com/fluencelabs/aquavm/commit/c9a0b87a4cefa3509b040c24d23cca37757fc030))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.11.0 to 0.11.1
    * air-interpreter-cid bumped from 0.7.0 to 0.8.0
    * air-interpreter-signatures bumped from 0.1.5 to 0.1.6
    * polyplets bumped from 0.5.1 to 0.5.2

## [0.15.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-data-v0.14.0...air-interpreter-data-v0.15.0) (2023-12-12)


### ⚠ BREAKING CHANGES

* **interpreter-cid,interpreter-data:** Support for multiple hash types in CID verification ([#722](https://github.com/fluencelabs/aquavm/issues/722))
* **interpreter-data:** allow only deterministic signature algorithms ([#734](https://github.com/fluencelabs/aquavm/issues/734))

### Features

* **interpreter-cid,interpreter-data:** Support for multiple hash types in CID verification ([#722](https://github.com/fluencelabs/aquavm/issues/722)) ([524c302](https://github.com/fluencelabs/aquavm/commit/524c30243bc544d5e265d9c6c7d1119a447202af))
* **interpreter-data:** allow only deterministic signature algorithms ([#734](https://github.com/fluencelabs/aquavm/issues/734)) ([15ce40a](https://github.com/fluencelabs/aquavm/commit/15ce40a1cd3271feb294666a1ef26d00282eb780))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.10.0 to 0.11.0
    * air-interpreter-cid bumped from 0.6.0 to 0.7.0
    * air-interpreter-signatures bumped from 0.1.4 to 0.1.5

## [0.14.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-data-v0.13.0...air-interpreter-data-v0.14.0) (2023-10-26)


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
    * air-interpreter-signatures bumped from 0.1.3 to 0.1.4

## [0.13.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-data-v0.12.1...air-interpreter-data-v0.13.0) (2023-10-16)


### ⚠ BREAKING CHANGES

* **execution-engine,test-utils,interpreter-data,interpreter-cid:** Rc into CID ([#718](https://github.com/fluencelabs/aquavm/issues/718))

### Features

* **execution-engine,test-utils,interpreter-data,interpreter-cid:** Rc into CID ([#718](https://github.com/fluencelabs/aquavm/issues/718)) ([c2108e0](https://github.com/fluencelabs/aquavm/commit/c2108e0fa09ea83854bb48c640e0cf23883a0bd0))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-cid bumped from 0.4.0 to 0.5.0
    * air-interpreter-signatures bumped from 0.1.2 to 0.1.3

## [0.12.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-data-v0.11.3...air-interpreter-data-v0.12.0) (2023-10-13)


### ⚠ BREAKING CHANGES

* **aquavm-air:** signature checking ([#607](https://github.com/fluencelabs/aquavm/issues/607))

### Features

* **aquavm-air:** signature checking ([#607](https://github.com/fluencelabs/aquavm/issues/607)) ([8a07613](https://github.com/fluencelabs/aquavm/commit/8a076130274c0500025e5c2ea74ec57e4c455971))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-cid bumped from 0.3.0 to 0.4.0
    * air-interpreter-signatures bumped from 0.1.1 to 0.1.2

## [0.11.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-data-v0.10.0...air-interpreter-data-v0.11.0) (2023-08-31)


### ⚠ BREAKING CHANGES

* **execution-engine,interpreter-data:** insert state for canon join ([#682](https://github.com/fluencelabs/aquavm/issues/682))

### Features

* **execution-engine,interpreter-data:** insert state for canon join ([#682](https://github.com/fluencelabs/aquavm/issues/682)) ([2b636e8](https://github.com/fluencelabs/aquavm/commit/2b636e808ae1b1422d5cc57c6796f32d4663d37c))

## [0.10.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-data-v0.9.0...air-interpreter-data-v0.10.0) (2023-08-17)


### ⚠ BREAKING CHANGES

* update marine-rs-sdk minor version

### Features

* update marine-rs-sdk minor version ([4b4e3bd](https://github.com/fluencelabs/aquavm/commit/4b4e3bde839d1167ea559d49b183d1a76bc93439))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * polyplets bumped from 0.4.1 to 0.5.0

## [0.9.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-data-v0.8.1...air-interpreter-data-v0.9.0) (2023-08-03)


### ⚠ BREAKING CHANGES

* **execution-engine:** refactor streams [fixes VM-255] ([#621](https://github.com/fluencelabs/aquavm/issues/621))

### Features

* **execution-engine:** refactor streams [fixes VM-255] ([#621](https://github.com/fluencelabs/aquavm/issues/621)) ([eca52b7](https://github.com/fluencelabs/aquavm/commit/eca52b7191ef1bc5c4573c62412dc735d830c023))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.8.0 to 0.8.1
    * polyplets bumped from 0.3.2 to 0.3.3

## [0.8.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-data-v0.7.0...air-interpreter-data-v0.8.0) (2023-06-22)


### ⚠ BREAKING CHANGES

* **aquavm-air,air-interpreter-signature,air-interpreter-data:** Peer signatures ([#598](https://github.com/fluencelabs/aquavm/issues/598))

### Features

* **air:** introduce explicit types for generation numbers ([#530](https://github.com/fluencelabs/aquavm/issues/530)) ([d62fa6f](https://github.com/fluencelabs/aquavm/commit/d62fa6fe6006e59d63d30549074e7b30f80bf687))
* **aquavm-air,air-interpreter-signature,air-interpreter-data:** Peer signatures ([#598](https://github.com/fluencelabs/aquavm/issues/598)) ([f8b734a](https://github.com/fluencelabs/aquavm/commit/f8b734abde8181cc2b2f11423f9d3bddd48f9fd1))
* **execution-engine:** Stream Map initial support [fixes VM-283,VM-284] ([#592](https://github.com/fluencelabs/aquavm/issues/592)) ([9d7d34a](https://github.com/fluencelabs/aquavm/commit/9d7d34a452cb65e968ed68decc67f3bc523a5115))
* **execution-engine:** StreamMap initial support for ap and new instructions [fixes VM-283,VM-284] ([9d7d34a](https://github.com/fluencelabs/aquavm/commit/9d7d34a452cb65e968ed68decc67f3bc523a5115))
* **interpreter-data:** Introduce source information for `canon` data ([#577](https://github.com/fluencelabs/aquavm/issues/577)) ([1d98afe](https://github.com/fluencelabs/aquavm/commit/1d98afeb34b1ee45defc05995c8cf24021449f2b))
* **trace-handler:** sub/-trace len dedicated alias to replace usize [fixes VM-282] ([b480e01](https://github.com/fluencelabs/aquavm/commit/b480e018b4b69b088d4258497866c3b31774b6b1))
* **trace-handler:** TracePos becomes a wrapper for u32 alias [fixes VM-267] ([#544](https://github.com/fluencelabs/aquavm/issues/544)) ([658daf1](https://github.com/fluencelabs/aquavm/commit/658daf1d3f6e733c15a21afc40ddf468ed745d43))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-utils bumped from 0.1.0 to 0.1.1
    * aquavm-air-parser bumped from 0.7.4 to 0.7.5
    * air-interpreter-interface bumped from 0.13.0 to 0.14.0
    * air-interpreter-cid bumped from 0.2.0 to 0.3.0
    * air-interpreter-signatures bumped from 0.1.0 to 0.1.1

## [0.7.0](https://github.com/fluencelabs/aquavm/compare/air-interpreter-data-v0.6.4...air-interpreter-data-v0.7.0) (2023-03-21)


### ⚠ BREAKING CHANGES

* **interpreter-data:** 

### Features

* **interpreter-data:** New data format for calls ([#501](https://github.com/fluencelabs/aquavm/issues/501)) ([d502894](https://github.com/fluencelabs/aquavm/commit/d5028942e41e1ac47ce31e20b57c17895f543ac8))

## [0.6.4](https://github.com/fluencelabs/aquavm/compare/air-interpreter-data-v0.6.3...air-interpreter-data-v0.6.4) (2023-03-15)


### Features

* **tools:** merge some tools into the `air` CLI tool ([#509](https://github.com/fluencelabs/aquavm/issues/509)) ([79ac153](https://github.com/fluencelabs/aquavm/commit/79ac153f1dcfc0a77ec511c6e25285728312ad4c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.12.1 to 0.13.0

## [0.6.3](https://github.com/fluencelabs/aquavm/compare/air-interpreter-data-v0.6.2...air-interpreter-data-v0.6.3) (2023-03-15)


### Features

* **tools:** merge some tools into the `air` CLI tool ([#509](https://github.com/fluencelabs/aquavm/issues/509)) ([79ac153](https://github.com/fluencelabs/aquavm/commit/79ac153f1dcfc0a77ec511c6e25285728312ad4c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.7.3 to 0.7.4

## [0.6.2](https://github.com/fluencelabs/aquavm/compare/air-interpreter-data-v0.6.1...air-interpreter-data-v0.6.2) (2023-02-08)


### Features

* **trace-handler:** improve data deserialization version check ([#451](https://github.com/fluencelabs/aquavm/issues/451)) ([367546b](https://github.com/fluencelabs/aquavm/commit/367546b82cd5f133b956857bf48d279512b157b2))

## Version 0.6.1

[PR](https://github.com/fluencelabs/aquavm/pull/451):

- move data and interpreter versions into a new structure
- add new API to get versions from data

## Version 0.6.0

[PR 419](https://github.com/fluencelabs/aquavm/pull/419):

- Rename data's `cid_store` field to `value_store`.
- Canon data is stored with CIDs. Values, tetraplets and canon elements are
  stored as CIDs resolved with data's `value_store`, `tetraplet_store` and
  `canon_store` fields respectively.
- Group stores in the data into `cid_info: CidInfo` field.

## Version 0.5.0

[PR 401](https://github.com/fluencelabs/aquavm/pull/401):

- Call result values are stored as CIDs in the data trace. These CIDs refer to a
  new `cid_store` data's field that maps a CID string to a value.

## Version 0.4.1

[PR 367](https://github.com/fluencelabs/aquavm/pull/367):

- add interpreter version in data

## Version 0.4.0

[PR 356](https://github.com/fluencelabs/aquavm/pull/358):

- temporary fix of a bug with fold and canon compatibility

## Version 0.3.0

[PR 292](https://github.com/fluencelabs/aquavm/pull/292):

- added a new state in data for a canon instruction result

## Version 0.2.2

[PR 169](https://github.com/fluencelabs/aquavm/pull/169):

- added a new field for tracking generations of private streams

## Version 0.2.1

[PR 130](https://github.com/fluencelabs/aquavm/pull/130):

- added a new field to track the latest exposed to a peer number of a call
  request
- `RequestSentBy` enum variant of `CallResult` contains a `Sender` enum to
  support call request scheme (this `Sender` will se/de into string, so this
  change won't require a hard fork)

## Version 0.2.0

[PR 74](https://github.com/fluencelabs/aquavm/pull/74) (hard fork):

- added a new state for the `ap` instruction
- added a new state for the `fold` instruction
- added a new field to track data version
- added a new field to track the maximum number of generation of each stream
- changed the serialization scheme of the `par` and `call` instructions in order
  to make it shorter in se view

## Version 0.1.0

The initial version of data with states for the `par` and `call` instruction was
introduced.
