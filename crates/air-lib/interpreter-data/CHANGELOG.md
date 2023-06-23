# Changelog

* The following workspace dependencies were updated
  * dependencies
    * air-parser bumped from 0.7.2 to 0.7.3

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
