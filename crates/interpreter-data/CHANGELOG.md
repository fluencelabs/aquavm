## Version 0.2.1

[PR 130](https://github.com/fluencelabs/aquavm/pull/130):  
- added a new field to track the latest exposed to a peer number of a call request
- `RequestSentBy` enum variant of `CallResult` contains a `Sender` enum to support call request scheme (this `Sender` will se/de into string, so this change won't require a hard fork) 

## Version 0.2.0

[PR 74](https://github.com/fluencelabs/aquavm/pull/74) (hard fork):
- added a new state for the `ap` instruction
- added a new state for the `fold` instruction
- added a new field to track data version
- added a new field to track the maximum number of generation of each stream
- changed the serialization scheme of the `par` and `call` instructions in order to make it shorter in se view

## Version 0.1.0

The initial version of data with states for the `par` and `call` instruction was introduced.
