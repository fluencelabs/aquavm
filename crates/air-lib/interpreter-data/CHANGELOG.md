# Changelog

* The following workspace dependencies were updated
  * dependencies
    * air-parser bumped from 0.7.2 to 0.7.3

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
