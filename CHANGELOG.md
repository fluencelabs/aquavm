## Version 0.10.6 (2021-06-10)

- fixed the error message for incorrect json path in `%last_error%` ([PR 119](https://github.com/fluencelabs/aquavm/pull/119))

## Version 0.10.5 (2021-06-10)

- json path applied to scalar values becomes non-joinable ([PR 118](https://github.com/fluencelabs/aquavm/pull/118))

## Version 0.10.4 (2021-06-09)

- `%last_error%` includes `peer_id` now, that contains id of a peer where an error occurred ([PR 117](https://github.com/fluencelabs/aquavm/pull/117)).

## Version 0.10.3-0.10.1 (2021-06-04)

- improved logger initialization and interface for wasm-bindgen target ([PR 116](https://github.com/fluencelabs/aquavm/pull/116), [PR 115](https://github.com/fluencelabs/aquavm/pull/115)).

## Version 0.10.0 (2021-06-09)

- `%last_error%` becomes an object of type
```rust
pub struct LastError {
    /// text representation of an instruction that caused the last error
    pub instruction: String,

    /// text representation of an error message
    pub msg: String,
}
```
and it's possible to address its fields separately: `%last_error%.$.instruction`, `%last_error%.$.msg` ([PR 112](https://github.com/fluencelabs/aquavm/pull/112)).

## Version 0.1.3 (2020-11-11)

- Switched to the new LALRPOP parser ([PR 13](https://github.com/fluencelabs/air/pull/13)):
    - arguments should be wrapped with square braces []
    - empty results in call allowed and lead to forget a call result
    
 - Added a few benchmarks
 - Fixed behaviour of the Xor instruction with inner Par instructions ([PR 19](https://github.com/fluencelabs/air/pull/19))
 - Iterator in the Fold becomes resolvable ([PR 23](https://github.com/fluencelabs/air/pull/23))   

## Version 0.1.2 (2020-10-29)

- Added new data format ([PR 12](https://github.com/fluencelabs/air/pull/12)):
    - previously data was a hashmap with variable names to values, and now it is call evidence path that contains call and par evidence states
    - logger is refactored and supports now several log targets
    - interpreter decoupled into two crates: `interpreter-lib` and `interpreter`. To build it for the FCE target the `fce` feature should be specified (`fce build --features fce`)

## Version 0.1.1 (2020-10-23)

- Added join behaviour ([PR 11](https://github.com/fluencelabs/air/pull/11)):
    - if `call` uses non existing variable, it is just being passed and isn't executed without any error
    - `par` becomes completed when at least one of its subtree is completed    
