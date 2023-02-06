## Next hardfork changes:
  - computing subtrace lengths in `FoldFSM` (for more details see [PR 138](https://github.com/fluencelabs/aquavm/pull/138))
  - change `Sender` struct serialization way in `CallResult::RequestSentBy`
  - add a separate (empty?) state in `air_interpreter_data::CallResult` for `CallOutputValue::None` for hardening
  - remove serde-based field renaming in data to support outdated data versions
  - place interpreter_version at the beginning of the data to make it deserializable regardless of the following fields
