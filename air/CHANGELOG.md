# Changelog

## [0.44.0](https://github.com/fluencelabs/aquavm/compare/air-v0.43.1...air-v0.44.0) (2023-08-17)


### ⚠ BREAKING CHANGES

* **polyplets:** move SecurityTetraplets to marine-rs-sdk  ([#674](https://github.com/fluencelabs/aquavm/issues/674))
* update marine-rs-sdk minor version

### Features

* **aquavm-air-cli:** `--near` execution mode [fixes VM-322] ([#672](https://github.com/fluencelabs/aquavm/issues/672)) ([0e80ee7](https://github.com/fluencelabs/aquavm/commit/0e80ee7908913fc896369ff7e00d65eeaf5d9f22))
* **polyplets:** move SecurityTetraplets to marine-rs-sdk  ([#674](https://github.com/fluencelabs/aquavm/issues/674)) ([7a8a460](https://github.com/fluencelabs/aquavm/commit/7a8a46057297317e1b776b13d913b0d42ec0a9af))
* update marine-rs-sdk minor version ([4b4e3bd](https://github.com/fluencelabs/aquavm/commit/4b4e3bde839d1167ea559d49b183d1a76bc93439))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-interface bumped from 0.14.0 to 0.15.0
    * polyplets bumped from 0.4.1 to 0.5.0

## [0.43.1](https://github.com/fluencelabs/aquavm/compare/air-v0.43.0...air-v0.43.1) (2023-08-07)


### Features

* **execution-engine:** rename boxed_values into value_types ([#664](https://github.com/fluencelabs/aquavm/issues/664)) ([20ae5ee](https://github.com/fluencelabs/aquavm/commit/20ae5eeeb9577f24bf27bcb74f17b92257d0e1a8))

## [0.43.0](https://github.com/fluencelabs/aquavm/compare/air-v0.42.0...air-v0.43.0) (2023-08-03)


### ⚠ BREAKING CHANGES

* **execution-engine:** update minimal interpreter version ([#649](https://github.com/fluencelabs/aquavm/issues/649))
* **execution-engine:** refactor streams [fixes VM-255] ([#621](https://github.com/fluencelabs/aquavm/issues/621))

### Features

* **execution-engine:** refactor streams [fixes VM-255] ([#621](https://github.com/fluencelabs/aquavm/issues/621)) ([eca52b7](https://github.com/fluencelabs/aquavm/commit/eca52b7191ef1bc5c4573c62412dc735d830c023))
* **execution-engine:** update minimal interpreter version ([#649](https://github.com/fluencelabs/aquavm/issues/649)) ([0655daa](https://github.com/fluencelabs/aquavm/commit/0655daa89d1105c6e786347f405d46d8e4d213ce))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.8.0 to 0.8.1
    * air-execution-info-collector bumped from 0.7.6 to 0.7.7
    * air-interpreter-data bumped from 0.8.1 to 0.9.0
    * air-trace-handler bumped from 0.2.2 to 0.3.0
    * polyplets bumped from 0.3.2 to 0.3.3

## [0.42.0](https://github.com/fluencelabs/aquavm/compare/air-v0.41.0...air-v0.42.0) (2023-07-16)


### ⚠ BREAKING CHANGES

* **execution-engine:** add error-code invariant check for match/mismatch ([#622](https://github.com/fluencelabs/aquavm/issues/622))
* **execution-engine:** %last_eror%.$.message and $.error_code now return an empty string and 0 correspondingly [fixes VM-313]
* **execution-engine:** unfefined %last_error% now returns an empty … ([#628](https://github.com/fluencelabs/aquavm/issues/628))
* **aquavm-air:** `ap` join behavior ([#631](https://github.com/fluencelabs/aquavm/issues/631))
* **execution-engine:** stream map to scalar conversion using canon instruction [fixes VM-294] ([#610](https://github.com/fluencelabs/aquavm/issues/610))

### Features

* **aquavm-air:** `ap` join behavior ([#631](https://github.com/fluencelabs/aquavm/issues/631)) ([75f5516](https://github.com/fluencelabs/aquavm/commit/75f5516c5803c256e329a7318632fbab13aea491)), closes [#632](https://github.com/fluencelabs/aquavm/issues/632)
* **execution-engine:** %last_eror%.$.message and $.error_code now return an empty string and 0 correspondingly [fixes VM-313] ([d195152](https://github.com/fluencelabs/aquavm/commit/d19515232043462e809d9cd6964042f69a77f4cf))
* **execution-engine:** add error-code invariant check for match/mismatch ([#622](https://github.com/fluencelabs/aquavm/issues/622)) ([33a9d9f](https://github.com/fluencelabs/aquavm/commit/33a9d9f32f84c5b31b59120f9da3c1624e1d5c27))
* **execution-engine:** Stream Map to Scalar conversion using canon instruction [fixes VM-294] ([fcb4c9d](https://github.com/fluencelabs/aquavm/commit/fcb4c9dab43d82e87f1d6f8a83b72f6891d37bef))
* **execution-engine:** stream map to scalar conversion using canon instruction [fixes VM-294] ([#610](https://github.com/fluencelabs/aquavm/issues/610)) ([fcb4c9d](https://github.com/fluencelabs/aquavm/commit/fcb4c9dab43d82e87f1d6f8a83b72f6891d37bef))
* **execution-engine:** unfefined %last_error% now returns an empty … ([#628](https://github.com/fluencelabs/aquavm/issues/628)) ([d195152](https://github.com/fluencelabs/aquavm/commit/d19515232043462e809d9cd6964042f69a77f4cf))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.7.5 to 0.8.0
    * air-execution-info-collector bumped from 0.7.5 to 0.7.6
    * air-interpreter-data bumped from 0.8.0 to 0.8.1
    * air-trace-handler bumped from 0.2.1 to 0.2.2

## [0.41.0](https://github.com/fluencelabs/aquavm/compare/air-v0.40.0...air-v0.41.0) (2023-06-23)


### ⚠ BREAKING CHANGES

* **testing-framework:** restore WASM test executor ([#609](https://github.com/fluencelabs/aquavm/issues/609))

### Miscellaneous Chores

* **testing-framework:** restore WASM test executor ([#609](https://github.com/fluencelabs/aquavm/issues/609)) ([c332cca](https://github.com/fluencelabs/aquavm/commit/c332cca6b75e804412e1f1cc51bdfe0580ea5fdd))

## [0.40.0](https://github.com/fluencelabs/aquavm/compare/air-v0.39.0...air-v0.40.0) (2023-06-22)


### ⚠ BREAKING CHANGES

* **avm-server,air-interpreter,aquavm-air:** secret key and particle ID arguments ([#593](https://github.com/fluencelabs/aquavm/issues/593))
* **aquavm-air,air-interpreter-signature,air-interpreter-data:** Peer signatures ([#598](https://github.com/fluencelabs/aquavm/issues/598))
* **aquavm-air-cli:** Usability enhancements ([#540](https://github.com/fluencelabs/aquavm/issues/540))

### Features

* **air:** introduce explicit types for generation numbers ([#530](https://github.com/fluencelabs/aquavm/issues/530)) ([d62fa6f](https://github.com/fluencelabs/aquavm/commit/d62fa6fe6006e59d63d30549074e7b30f80bf687))
* **aquavm-air-cli:** Usability enhancements ([#540](https://github.com/fluencelabs/aquavm/issues/540)) ([73c1ba7](https://github.com/fluencelabs/aquavm/commit/73c1ba70cca9ba4f6e73880141e738d450565798))
* **aquavm-air,air-interpreter-signature,air-interpreter-data:** Peer signatures ([#598](https://github.com/fluencelabs/aquavm/issues/598)) ([f8b734a](https://github.com/fluencelabs/aquavm/commit/f8b734abde8181cc2b2f11423f9d3bddd48f9fd1))
* **avm-server,air-interpreter,aquavm-air:** secret key and particle ID arguments ([#593](https://github.com/fluencelabs/aquavm/issues/593)) ([8ce8af3](https://github.com/fluencelabs/aquavm/commit/8ce8af38232de3f1ac359214386b895356550428))
* **execution-engine:** introduce resolver ([#574](https://github.com/fluencelabs/aquavm/issues/574)) ([a66541d](https://github.com/fluencelabs/aquavm/commit/a66541de497e0b2f0ed97631d63e35a25215bca3))
* **execution-engine:** remove stream jvaluable implementation ([#576](https://github.com/fluencelabs/aquavm/issues/576)) ([513d33a](https://github.com/fluencelabs/aquavm/commit/513d33a1c5faf972907bd402dfd6ad39cacd1eff))
* **execution-engine:** Stream Map initial support [fixes VM-283,VM-284] ([#592](https://github.com/fluencelabs/aquavm/issues/592)) ([9d7d34a](https://github.com/fluencelabs/aquavm/commit/9d7d34a452cb65e968ed68decc67f3bc523a5115))
* **execution-engine:** StreamMap initial support for ap and new instructions [fixes VM-283,VM-284] ([9d7d34a](https://github.com/fluencelabs/aquavm/commit/9d7d34a452cb65e968ed68decc67f3bc523a5115))
* **interpreter-data:** Introduce source information for `canon` data ([#577](https://github.com/fluencelabs/aquavm/issues/577)) ([1d98afe](https://github.com/fluencelabs/aquavm/commit/1d98afeb34b1ee45defc05995c8cf24021449f2b))
* **trace-handler:** sub/-trace len dedicated alias to replace usize [fixes VM-282] ([b480e01](https://github.com/fluencelabs/aquavm/commit/b480e018b4b69b088d4258497866c3b31774b6b1))
* **trace-handler:** TracePos becomes a wrapper for u32 alias [fixes VM-267] ([#544](https://github.com/fluencelabs/aquavm/issues/544)) ([658daf1](https://github.com/fluencelabs/aquavm/commit/658daf1d3f6e733c15a21afc40ddf468ed745d43))


### Bug Fixes

* **deps:** update rust crate marine-rs-sdk to 0.7.1 ([#568](https://github.com/fluencelabs/aquavm/issues/568)) ([648f297](https://github.com/fluencelabs/aquavm/commit/648f297a2badde312c88d3db9eec085170211aa6))
* **execution-engine:** Fold-over-scalar values' wrong lambda ([#578](https://github.com/fluencelabs/aquavm/issues/578)) ([88fd1f3](https://github.com/fluencelabs/aquavm/commit/88fd1f3095fc47862472baf30ff79964ec662b37))
* **execution-engine:** this removes an unused and impossible check ([#575](https://github.com/fluencelabs/aquavm/issues/575)) ([70f27f7](https://github.com/fluencelabs/aquavm/commit/70f27f7cb6e0ff21be9695a082b1fadf3a2dd05f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.7.4 to 0.7.5
    * air-execution-info-collector bumped from 0.7.4 to 0.7.5
    * air-interpreter-cid bumped from 0.2.0 to 0.3.0
    * air-interpreter-data bumped from 0.7.0 to 0.8.0
    * air-interpreter-signatures bumped from 0.1.0 to 0.1.1
    * air-interpreter-interface bumped from 0.13.0 to 0.14.0
    * air-trace-handler bumped from 0.2.0 to 0.2.1
    * air-utils bumped from 0.1.0 to 0.1.1

## [0.39.0](https://github.com/fluencelabs/aquavm/compare/air-v0.38.0...air-v0.39.0) (2023-03-21)


### ⚠ BREAKING CHANGES

* **interpreter-data:** 

### Features

* **interpreter-data:** New data format for calls ([#501](https://github.com/fluencelabs/aquavm/issues/501)) ([d502894](https://github.com/fluencelabs/aquavm/commit/d5028942e41e1ac47ce31e20b57c17895f543ac8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.6.4 to 0.7.0
    * air-trace-handler bumped from 0.1.3 to 0.2.0

## [0.38.0](https://github.com/fluencelabs/aquavm/compare/air-v0.37.1...air-v0.38.0) (2023-03-15)


### ⚠ BREAKING CHANGES

* **avm:** integrate Marine 0.26.0 ([#461](https://github.com/fluencelabs/aquavm/issues/461))

### Features

* **avm:** integrate Marine 0.26.0 ([#461](https://github.com/fluencelabs/aquavm/issues/461)) ([126d550](https://github.com/fluencelabs/aquavm/commit/126d5507c81a7f978ab9cf06c492b1092a336cf6))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-execution-info-collector bumped from 0.7.3 to 0.7.4
    * air-interpreter-data bumped from 0.6.3 to 0.6.4
    * air-interpreter-interface bumped from 0.12.1 to 0.13.0
    * air-trace-handler bumped from 0.1.2 to 0.1.3

## [0.37.1](https://github.com/fluencelabs/aquavm/compare/air-v0.37.0...air-v0.37.1) (2023-03-15)


### Features

* **tools:** merge some tools into the `air` CLI tool ([#509](https://github.com/fluencelabs/aquavm/issues/509)) ([79ac153](https://github.com/fluencelabs/aquavm/commit/79ac153f1dcfc0a77ec511c6e25285728312ad4c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * aquavm-air-parser bumped from 0.7.3 to 0.7.4
    * air-execution-info-collector bumped from 0.7.2 to 0.7.3
    * air-interpreter-data bumped from 0.6.2 to 0.6.3

## [0.37.0](https://github.com/fluencelabs/aquavm/compare/air-v0.36.0...air-v0.37.0) (2023-03-13)


### ⚠ BREAKING CHANGES

* **execution-engine:** make StreamDontHaveSuchGeneration uncatchable… ([#492](https://github.com/fluencelabs/aquavm/issues/492))

### Features

* **execution-engine:** make StreamDontHaveSuchGeneration uncatchable… ([#492](https://github.com/fluencelabs/aquavm/issues/492)) ([95b2d15](https://github.com/fluencelabs/aquavm/commit/95b2d154ff84caf5efe7a3960922d1d6c39a9ed2))


### Bug Fixes

* **air:** demote some AquaVM logging statements ([#505](https://github.com/fluencelabs/aquavm/issues/505)) ([271b74d](https://github.com/fluencelabs/aquavm/commit/271b74d8f2f1111dfb8393aa81c8f9c9e78ff4d7))
* **execution-engine:** negative tests for prepare_step and farewell_step [fixes VM-251] ([5813c80](https://github.com/fluencelabs/aquavm/commit/5813c80ca2561cb8121792c4123a3b7171b8e2c7))
* **execution-engine:** rename CallResultsNotEmpty into UnprocessedCallResult ([7f6a698](https://github.com/fluencelabs/aquavm/commit/7f6a69851f93f0a7abcc852913b6e7116fd35167))
* negative tests for prepare_step and farewell_step [fixes VM-251] ([#489](https://github.com/fluencelabs/aquavm/issues/489)) ([5813c80](https://github.com/fluencelabs/aquavm/commit/5813c80ca2561cb8121792c4123a3b7171b8e2c7))
* Rename CallResultsNotEmpty into UnprocessedCallResult ([#490](https://github.com/fluencelabs/aquavm/issues/490)) ([7f6a698](https://github.com/fluencelabs/aquavm/commit/7f6a69851f93f0a7abcc852913b6e7116fd35167))

## [0.36.0](https://github.com/fluencelabs/aquavm/compare/air-v0.35.4...air-v0.36.0) (2023-02-27)


### ⚠ BREAKING CHANGES

* **execution-engine:** make fold convergent wrt errors ([#351](https://github.com/fluencelabs/aquavm/issues/351))

### Features

* **execution-engine:** make fold convergent wrt errors ([#351](https://github.com/fluencelabs/aquavm/issues/351)) ([87f7e2f](https://github.com/fluencelabs/aquavm/commit/87f7e2f361891c84315f310967517ddb50773f8d))
* **tools:** VM-194 performance metering ([#440](https://github.com/fluencelabs/aquavm/issues/440)) ([5fdc8e6](https://github.com/fluencelabs/aquavm/commit/5fdc8e68ac67f502f8ece4d8a5935cf7d478d830))
* **trace-handler:** improve data deserialization version check ([#451](https://github.com/fluencelabs/aquavm/issues/451)) ([367546b](https://github.com/fluencelabs/aquavm/commit/367546b82cd5f133b956857bf48d279512b157b2))

## [0.35.1](https://github.com/fluencelabs/aquavm/compare/air-v0.35.0...air-v0.35.1) (2023-02-08)


### Features

* **tools:** VM-194 performance metering ([#440](https://github.com/fluencelabs/aquavm/issues/440)) ([5fdc8e6](https://github.com/fluencelabs/aquavm/commit/5fdc8e68ac67f502f8ece4d8a5935cf7d478d830))
* **trace-handler:** improve data deserialization version check ([#451](https://github.com/fluencelabs/aquavm/issues/451)) ([367546b](https://github.com/fluencelabs/aquavm/commit/367546b82cd5f133b956857bf48d279512b157b2))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * air-interpreter-data bumped from 0.6.1 to 0.6.2
    * air-trace-handler bumped from 0.1.0 to 0.1.1
