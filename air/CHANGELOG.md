# Changelog

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
