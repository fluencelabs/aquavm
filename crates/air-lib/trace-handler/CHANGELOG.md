# Changelog

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.8.0 to 0.8.1
    * aquavm-air-parser bumped from 0.7.5 to 0.8.0

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.11.0 to 0.11.1
    * aquavm-air-parser bumped from 0.8.1 to 0.8.2

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.11.1 to 0.11.2
    * aquavm-air-parser bumped from 0.8.2 to 0.9.0

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.11.2 to 0.11.3
    * aquavm-air-parser bumped from 0.9.0 to 0.10.0
    * polyplets bumped from 0.5.0 to 0.5.1

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-cid bumped from 0.3.0 to 0.4.0
    * air-interpreter-data bumped from 0.11.3 to 0.12.0

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.12.0 to 0.12.1

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-cid bumped from 0.4.0 to 0.5.0
    * air-interpreter-data bumped from 0.12.1 to 0.13.0

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-cid bumped from 0.5.0 to 0.6.0
    * air-interpreter-data bumped from 0.13.0 to 0.14.0

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-cid bumped from 0.6.0 to 0.7.0
    * air-interpreter-data bumped from 0.14.0 to 0.15.0
    * aquavm-air-parser bumped from 0.10.0 to 0.11.0

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-cid bumped from 0.7.0 to 0.8.0
    * air-interpreter-data bumped from 0.15.0 to 0.16.0
    * aquavm-air-parser bumped from 0.11.0 to 0.11.1
    * polyplets bumped from 0.5.1 to 0.5.2

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-cid bumped from 0.8.0 to 0.9.0
    * air-interpreter-data bumped from 0.16.0 to 0.17.0
    * polyplets bumped from 0.5.2 to 0.6.0

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.17.0 to 0.17.1
    * aquavm-air-parser bumped from 0.11.1 to 0.11.2

## [0.5.0](https://github.com/fluencelabs/aquavm/compare/air-trace-handler-v0.4.0...air-trace-handler-v0.5.0) (2023-08-31)


### ⚠ BREAKING CHANGES

* **execution-engine,interpreter-data:** insert state for canon join ([#682](https://github.com/fluencelabs/aquavm/issues/682))

### Features

* **execution-engine,interpreter-data:** insert state for canon join ([#682](https://github.com/fluencelabs/aquavm/issues/682)) ([2b636e8](https://github.com/fluencelabs/aquavm/commit/2b636e808ae1b1422d5cc57c6796f32d4663d37c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.10.0 to 0.11.0

## [0.4.0](https://github.com/fluencelabs/aquavm/compare/air-trace-handler-v0.3.0...air-trace-handler-v0.4.0) (2023-08-17)


### ⚠ BREAKING CHANGES

* update marine-rs-sdk minor version

### Features

* update marine-rs-sdk minor version ([4b4e3bd](https://github.com/fluencelabs/aquavm/commit/4b4e3bde839d1167ea559d49b183d1a76bc93439))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * polyplets bumped from 0.4.1 to 0.5.0

## [0.3.0](https://github.com/fluencelabs/aquavm/compare/air-trace-handler-v0.2.2...air-trace-handler-v0.3.0) (2023-08-03)


### ⚠ BREAKING CHANGES

* **execution-engine:** refactor streams [fixes VM-255] ([#621](https://github.com/fluencelabs/aquavm/issues/621))

### Features

* **execution-engine:** refactor streams [fixes VM-255] ([#621](https://github.com/fluencelabs/aquavm/issues/621)) ([eca52b7](https://github.com/fluencelabs/aquavm/commit/eca52b7191ef1bc5c4573c62412dc735d830c023))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.8.1 to 0.9.0
    * aquavm-air-parser bumped from 0.8.0 to 0.8.1
    * polyplets bumped from 0.3.2 to 0.3.3

## [0.2.1](https://github.com/fluencelabs/aquavm/compare/air-trace-handler-v0.2.0...air-trace-handler-v0.2.1) (2023-06-22)


### Features

* **air:** introduce explicit types for generation numbers ([#530](https://github.com/fluencelabs/aquavm/issues/530)) ([d62fa6f](https://github.com/fluencelabs/aquavm/commit/d62fa6fe6006e59d63d30549074e7b30f80bf687))
* **interpreter-data:** Introduce source information for `canon` data ([#577](https://github.com/fluencelabs/aquavm/issues/577)) ([1d98afe](https://github.com/fluencelabs/aquavm/commit/1d98afeb34b1ee45defc05995c8cf24021449f2b))
* **trace-handler:** sub/-trace len dedicated alias to replace usize [fixes VM-282] ([b480e01](https://github.com/fluencelabs/aquavm/commit/b480e018b4b69b088d4258497866c3b31774b6b1))
* **trace-handler:** TracePos becomes a wrapper for u32 alias [fixes VM-267] ([#544](https://github.com/fluencelabs/aquavm/issues/544)) ([658daf1](https://github.com/fluencelabs/aquavm/commit/658daf1d3f6e733c15a21afc40ddf468ed745d43))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-cid bumped from 0.2.0 to 0.3.0
    * air-interpreter-data bumped from 0.7.0 to 0.8.0
    * air-interpreter-interface bumped from 0.13.0 to 0.14.0
    * aquavm-air-parser bumped from 0.7.4 to 0.7.5

## [0.2.0](https://github.com/fluencelabs/aquavm/compare/air-trace-handler-v0.1.3...air-trace-handler-v0.2.0) (2023-03-21)


### ⚠ BREAKING CHANGES

* **interpreter-data:** 

### Features

* **interpreter-data:** New data format for calls ([#501](https://github.com/fluencelabs/aquavm/issues/501)) ([d502894](https://github.com/fluencelabs/aquavm/commit/d5028942e41e1ac47ce31e20b57c17895f543ac8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.6.4 to 0.7.0

## [0.1.3](https://github.com/fluencelabs/aquavm/compare/air-trace-handler-v0.1.2...air-trace-handler-v0.1.3) (2023-03-15)


### Features

* **tools:** merge some tools into the `air` CLI tool ([#509](https://github.com/fluencelabs/aquavm/issues/509)) ([79ac153](https://github.com/fluencelabs/aquavm/commit/79ac153f1dcfc0a77ec511c6e25285728312ad4c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.6.3 to 0.6.4
    * air-interpreter-interface bumped from 0.12.1 to 0.13.0

## [0.1.2](https://github.com/fluencelabs/aquavm/compare/air-trace-handler-v0.1.1...air-trace-handler-v0.1.2) (2023-03-15)


### Features

* **tools:** merge some tools into the `air` CLI tool ([#509](https://github.com/fluencelabs/aquavm/issues/509)) ([79ac153](https://github.com/fluencelabs/aquavm/commit/79ac153f1dcfc0a77ec511c6e25285728312ad4c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.7.3 to 0.7.4
