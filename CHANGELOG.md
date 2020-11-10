## Version 0.1.3 (2020-11-11)

- Switched to the new LALRPOP parser ([PR 13](https://github.com/fluencelabs/aquamarine/pull/13)):
    - arguments should be wrapped wit square braces []
    - empty results in call allowed and lead to forget a call result
    
 - Added a few benchmarks
 - Fixed behaviour of the Xor instruction with inner Par instructions ([PR 19](https://github.com/fluencelabs/aquamarine/pull/19))
 - Iterator in the Fold becomes resolvable ([PR 23](https://github.com/fluencelabs/aquamarine/pull/23))   

## Version 0.1.2 (2020-10-29)

- Added new data format ([PR 12](https://github.com/fluencelabs/aquamarine/pull/12)):
    - previously data was a hashmap with variable names to values, and now it is call evidence path that contains call and par evidence states
    - logger is refactored and supports now several log targets
    - stepper decoupled into two crates: `stepper-lib` and `stepper`. To build it for the FCE target the `fce` feature should be specified (`fce build --features fce`)

## Version 0.1.1 (2020-10-23)

- Added join behaviour ([PR 11](https://github.com/fluencelabs/aquamarine/pull/11)):
    - if `call` uses non existing variable, it is just being passed and isn't executed without any error
    - `par` becomes completed when at least one of its subtree is completed    
