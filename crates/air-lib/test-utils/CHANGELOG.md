# Changelog

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.6.2 to 0.6.3
    * avm-interface bumped from 0.28.1 to 0.28.2
    * avm-server bumped from 0.28.1 to 0.29.0

* The following workspace dependencies were updated
  * dependencies
    * avm-server bumped from 0.29.0 to 0.30.0

* The following workspace dependencies were updated
  * dependencies
    * avm-server bumped from 0.30.0 to 0.30.1

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air bumped from 0.37.1 to 0.38.0
    * air-interpreter-data bumped from 0.6.3 to 0.6.4
    * air-interpreter-interface bumped from 0.12.1 to 0.13.0
    * avm-interface bumped from 0.28.2 to 0.28.3
    * avm-server bumped from 0.30.1 to 0.31.0

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air bumped from 0.41.0 to 0.42.0
    * air-interpreter-data bumped from 0.8.0 to 0.8.1

* The following workspace dependencies were updated
  * dependencies
    * avm-server bumped from 0.32.1 to 0.32.2

## [0.8.0](https://github.com/fluencelabs/aquavm/compare/air-test-utils-v0.7.1...air-test-utils-v0.8.0) (2023-08-03)


### ⚠ BREAKING CHANGES

* **execution-engine:** refactor streams [fixes VM-255] ([#621](https://github.com/fluencelabs/aquavm/issues/621))

### Features

* **air-test-utils:** `print_trace` prints values ([#633](https://github.com/fluencelabs/aquavm/issues/633)) ([c530c93](https://github.com/fluencelabs/aquavm/commit/c530c93fcbffe187797d48d65bf8478dcafa8de5))
* **execution-engine:** refactor streams [fixes VM-255] ([#621](https://github.com/fluencelabs/aquavm/issues/621)) ([eca52b7](https://github.com/fluencelabs/aquavm/commit/eca52b7191ef1bc5c4573c62412dc735d830c023))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air bumped from 0.42.0 to 0.43.0
    * air-interpreter-data bumped from 0.8.1 to 0.9.0
    * avm-interface bumped from 0.28.4 to 0.28.5
    * avm-server bumped from 0.32.0 to 0.32.1

## [0.7.0](https://github.com/fluencelabs/aquavm/compare/air-test-utils-v0.6.0...air-test-utils-v0.7.0) (2023-06-23)


### ⚠ BREAKING CHANGES

* **testing-framework:** restore WASM test executor ([#609](https://github.com/fluencelabs/aquavm/issues/609))

### Miscellaneous Chores

* **testing-framework:** restore WASM test executor ([#609](https://github.com/fluencelabs/aquavm/issues/609)) ([c332cca](https://github.com/fluencelabs/aquavm/commit/c332cca6b75e804412e1f1cc51bdfe0580ea5fdd))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air bumped from 0.40.0 to 0.41.0

## [0.6.0](https://github.com/fluencelabs/aquavm/compare/air-test-utils-v0.5.0...air-test-utils-v0.6.0) (2023-06-22)


### ⚠ BREAKING CHANGES

* **avm-server,air-interpreter,aquavm-air:** secret key and particle ID arguments ([#593](https://github.com/fluencelabs/aquavm/issues/593))
* **aquavm-air,air-interpreter-signature,air-interpreter-data:** Peer signatures ([#598](https://github.com/fluencelabs/aquavm/issues/598))

### Features

* **air:** introduce explicit types for generation numbers ([#530](https://github.com/fluencelabs/aquavm/issues/530)) ([d62fa6f](https://github.com/fluencelabs/aquavm/commit/d62fa6fe6006e59d63d30549074e7b30f80bf687))
* **aquavm-air,air-interpreter-signature,air-interpreter-data:** Peer signatures ([#598](https://github.com/fluencelabs/aquavm/issues/598)) ([f8b734a](https://github.com/fluencelabs/aquavm/commit/f8b734abde8181cc2b2f11423f9d3bddd48f9fd1))
* **avm-server,air-interpreter,aquavm-air:** secret key and particle ID arguments ([#593](https://github.com/fluencelabs/aquavm/issues/593)) ([8ce8af3](https://github.com/fluencelabs/aquavm/commit/8ce8af38232de3f1ac359214386b895356550428))
* **interpreter-data:** Introduce source information for `canon` data ([#577](https://github.com/fluencelabs/aquavm/issues/577)) ([1d98afe](https://github.com/fluencelabs/aquavm/commit/1d98afeb34b1ee45defc05995c8cf24021449f2b))
* **trace-handler:** TracePos becomes a wrapper for u32 alias [fixes VM-267] ([#544](https://github.com/fluencelabs/aquavm/issues/544)) ([658daf1](https://github.com/fluencelabs/aquavm/commit/658daf1d3f6e733c15a21afc40ddf468ed745d43))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air bumped from 0.39.0 to 0.40.0
    * air-interpreter-cid bumped from 0.2.0 to 0.3.0
    * air-interpreter-data bumped from 0.7.0 to 0.8.0
    * air-interpreter-interface bumped from 0.13.0 to 0.14.0
    * avm-interface bumped from 0.28.3 to 0.28.4
    * avm-server bumped from 0.31.0 to 0.32.0

## [0.5.0](https://github.com/fluencelabs/aquavm/compare/air-test-utils-v0.4.7...air-test-utils-v0.5.0) (2023-03-21)


### ⚠ BREAKING CHANGES

* **interpreter-data:** 

### Features

* **interpreter-data:** New data format for calls ([#501](https://github.com/fluencelabs/aquavm/issues/501)) ([d502894](https://github.com/fluencelabs/aquavm/commit/d5028942e41e1ac47ce31e20b57c17895f543ac8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air bumped from 0.38.0 to 0.39.0
    * air-interpreter-data bumped from 0.6.4 to 0.7.0

## [0.4.6](https://github.com/fluencelabs/aquavm/compare/air-test-utils-v0.4.5...air-test-utils-v0.4.6) (2023-03-15)


### Features

* **tools:** merge some tools into the `air` CLI tool ([#509](https://github.com/fluencelabs/aquavm/issues/509)) ([79ac153](https://github.com/fluencelabs/aquavm/commit/79ac153f1dcfc0a77ec511c6e25285728312ad4c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air bumped from 0.37.0 to 0.37.1

## [0.4.2](https://github.com/fluencelabs/aquavm/compare/air-test-utils-v0.4.1...air-test-utils-v0.4.2) (2023-02-08)


### Features

* **trace-handler:** improve data deserialization version check ([#451](https://github.com/fluencelabs/aquavm/issues/451)) ([367546b](https://github.com/fluencelabs/aquavm/commit/367546b82cd5f133b956857bf48d279512b157b2))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.6.1 to 0.6.2
