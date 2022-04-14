## Version 0.22.0 (2021-04-14)

[PR 243](https://github.com/fluencelabs/aquavm/pull/243):
Clean scalars at the end of scope

[PR 231](https://github.com/fluencelabs/aquavm/pull/231):
Test refactoring

[PR 228](https://github.com/fluencelabs/aquavm/pull/228):
Improve stream determinism

## Version 0.21.0 (2021-02-26)

[PR 225](https://github.com/fluencelabs/aquavm/pull/225):  
Introduce recursive streams

[PR 224](https://github.com/fluencelabs/aquavm/pull/224) [PR 220](https://github.com/fluencelabs/aquavm/pull/224) [PR 217](https://github.com/fluencelabs/aquavm/pull/217) [PR 215](https://github.com/fluencelabs/aquavm/pull/215) [PR 212](https://github.com/fluencelabs/aquavm/pull/212) [PR 207](https://github.com/fluencelabs/aquavm/pull/207):  
Various bugs fixed

[PR 210](https://github.com/fluencelabs/aquavm/pull/210):  
Add API for returning AquaVM consumed memory size

## Version 0.20.0 (2021-12-29)

[PR 205](https://github.com/fluencelabs/aquavm/pull/205):  
Supported scalars in `fail` instructions.

[PR 202](https://github.com/fluencelabs/aquavm/pull/202) [PR 198](https://github.com/fluencelabs/aquavm/pull/198):  
AquaVM errors mechanism was completely refactored.

[PR 207](https://github.com/fluencelabs/aquavm/pull/207):  
Fixed bug with empty array in `match`.

## Version 0.19.0 (2021-12-15)

[PR 196](https://github.com/fluencelabs/aquavm/pull/196):  
Introduced fail instruction.

[PR 194](https://github.com/fluencelabs/aquavm/pull/194):  
Added variables names in resolve errors.

## Version 0.18.0 (2021-12-14)

[PR 192](https://github.com/fluencelabs/aquavm/pull/172):  
Added a possibility to use scalars in lambdas.

[PR 190](https://github.com/fluencelabs/aquavm/pull/190), [PR 186](https://github.com/fluencelabs/aquavm/pull/186), [PR 185](https://github.com/fluencelabs/aquavm/pull/185), [PR 182](https://github.com/fluencelabs/aquavm/pull/182), [PR 181](https://github.com/fluencelabs/aquavm/pull/181):  
Bug fixing.

## Version 0.17.0 (2021-11-24)

[PR 172](https://github.com/fluencelabs/aquavm/pull/172):  
A new instruction intended to restrict a scope of variables was introduced to AquaVM.

[PR 168](https://github.com/fluencelabs/aquavm/pull/168):  
AIR parser and AST was highly refactored to be more suitable to the scalar/stream restriction scheme used in AIR instructions.   

[PR 164](https://github.com/fluencelabs/aquavm/pull/164):  
SecurityTetraplet was decoupled with marine-rs-sdk to have the only one definition in AquaVM that then exported by marine-rs-sdk.

[PR 162](https://github.com/fluencelabs/aquavm/pull/162):  
The scalar scoping scheme was improved in order to support more than two scope levels. 

## Version 0.16.0 (2021-10-18)

[PR 154](https://github.com/fluencelabs/aquavm/pull/154)  
The json path crate has been removed and changed to custom lambda scheme that have a subset of functionality of json path used by the Aqua compiler. The flattening sign `!` is still allowed now, but does nothing.

[PR 150](https://github.com/fluencelabs/aquavm/pull/150), [PR 152](https://github.com/fluencelabs/aquavm/pull/152), [PR 153](https://github.com/fluencelabs/aquavm/pull/153) [PR 160](https://github.com/fluencelabs/aquavm/pull/160)  
Some parts of the interpreter has been refactored to make it more modular. 

[PR 144](https://github.com/fluencelabs/aquavm/pull/144)  
The interpreter changed to be built with `unwind` panic handler and some other debug options were turned on.

## Version 0.15.0 (2021-10-04)

[PR 140](https://github.com/fluencelabs/aquavm/pull/130):  
- the interpreter become async, now it's a pure function without any imports from a peer. Instead of calling import `call_service` from a peer, it now returns call results in the outcome structure, and receives their result in the `invoke` export.
- data structure now includes a new field to track last call request id to give peer more freedom.
- AVM server was completely refactored to support the new interpreter model and to expose a new trait storing data for a node.

[PR 139](https://github.com/fluencelabs/aquavm/pull/139)  
  Senders in `RequestSentBy` could be different now.

[PR 138](https://github.com/fluencelabs/aquavm/pull/138)  
  The computation algo for `subtrace_len` was completely refactored.

[PR 136](https://github.com/fluencelabs/aquavm/pull/136)  
  `serde` and `serde_json` crates were used without locking their version

[PR 133](https://github.com/fluencelabs/aquavm/pull/133)  
  fixed bug with applying json path to an empty stream

[PR 132](https://github.com/fluencelabs/aquavm/pull/132)  
  fix bug with json flattening

## Version 0.14.0 (2021-08-24)

[PR 74](https://github.com/fluencelabs/aquavm/pull/74):  
- introduced a new CRDT-like data format for streams:
  - call results contains different values for streams and scalars
  - introduced a new state for fold whose iterables are streams
- merging scheme was rewritten, and became lazy
- refactor the internal value mechanism
- introduced a new instruction `(ap` responsible for applying json path to scalars and save results as a new scalar or add it to a stream. In the second case it'll produce a new state in a data. 
- introduced a new string literal `[]` represents empty array

## Version 0.10.8 (2021-07-06)

- improve the error message of the invalid executed state error ([PR 121](https://github.com/fluencelabs/aquavm/pull/121))

## Version 0.10.7 (2021-07-01)

- add support of a particle file vault ([PR 120](https://github.com/fluencelabs/aquavm/pull/120))

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

- Switched to the new LALRPOP parser ([PR 13](https://github.com/fluencelabs/aquavm/pull/13)):
    - arguments should be wrapped with square braces []
    - empty results in call allowed and lead to forget a call result
    
 - Added a few benchmarks
 - Fixed behaviour of the Xor instruction with inner Par instructions ([PR 19](https://github.com/fluencelabs/aquavm/pull/19))
 - Iterator in the Fold becomes resolvable ([PR 23](https://github.com/fluencelabs/aquavm/pull/23))   

## Version 0.1.2 (2020-10-29)

- Added new data format ([PR 12](https://github.com/fluencelabs/aquavm/pull/12)):
    - previously data was a hashmap with variable names to values, and now it is call evidence path that contains call and par evidence states
    - logger is refactored and supports now several log targets
    - interpreter decoupled into two crates: `interpreter-lib` and `interpreter`. To build it for the FCE target the `fce` feature should be specified (`fce build --features fce`)

## Version 0.1.1 (2020-10-23)

- Added join behaviour ([PR 11](https://github.com/fluencelabs/aquavm/pull/11)):
    - if `call` uses non existing variable, it is just being passed and isn't executed without any error
    - `par` becomes completed when at least one of its subtree is completed    
