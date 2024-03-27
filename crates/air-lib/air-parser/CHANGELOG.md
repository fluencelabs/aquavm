# Changelog

## [0.12.0](https://github.com/fluencelabs/aquavm/compare/air-parser-v0.11.2...air-parser-v0.12.0) (2024-02-22)


### ⚠ BREAKING CHANGES

* update marine-rs-sdk to 0.14.0, remove all of json_path remains ([#820](https://github.com/fluencelabs/aquavm/issues/820))

### Features

* update marine-rs-sdk to 0.14.0, remove all of json_path remains ([#820](https://github.com/fluencelabs/aquavm/issues/820)) ([08e8547](https://github.com/fluencelabs/aquavm/commit/08e85478b4716f2ae5f57bc57dcb5d1df63f1b5d))

## [0.11.2](https://github.com/fluencelabs/aquavm/compare/air-parser-v0.11.1...air-parser-v0.11.2) (2024-02-20)


### Features

* **execution-engine:** Rc-based JSON value ([#813](https://github.com/fluencelabs/aquavm/issues/813)) ([0d53f2b](https://github.com/fluencelabs/aquavm/commit/0d53f2bab1a09ae781bf22da6546e750e6172aa7))

## [0.11.1](https://github.com/fluencelabs/aquavm/compare/air-parser-v0.11.0...air-parser-v0.11.1) (2024-01-03)


### Features

* **air-parser:** VM-347 After next validator fold check ([#774](https://github.com/fluencelabs/aquavm/issues/774)) ([c6f157a](https://github.com/fluencelabs/aquavm/commit/c6f157a6e3ee10fa9209b1fa4a305040ce876d00))

## [0.11.0](https://github.com/fluencelabs/aquavm/compare/air-parser-v0.10.0...air-parser-v0.11.0) (2023-12-12)


### ⚠ BREAKING CHANGES

* **air-parser:** optimize Instruction type layout ([#767](https://github.com/fluencelabs/aquavm/issues/767))

### Features

* **air-parser:** optimize Instruction type layout ([#767](https://github.com/fluencelabs/aquavm/issues/767)) ([1673cdf](https://github.com/fluencelabs/aquavm/commit/1673cdf06c1fcdd9d06789b0d9d76e71d1625fea))

## [0.10.0](https://github.com/fluencelabs/aquavm/compare/air-parser-v0.9.0...air-parser-v0.10.0) (2023-09-21)


### ⚠ BREAKING CHANGES

* **execution-engine:** this patch prohibits error code = 0 ([#702](https://github.com/fluencelabs/aquavm/issues/702))

### Features

* **execution-engine:** this patch prohibits error code = 0 ([#702](https://github.com/fluencelabs/aquavm/issues/702)) ([45035cc](https://github.com/fluencelabs/aquavm/commit/45035ccff515344ee8c2dc63f172f00637226778))
* **parser,execution-engine:** allow :error: in fail ([#696](https://github.com/fluencelabs/aquavm/issues/696)) ([bd80a12](https://github.com/fluencelabs/aquavm/commit/bd80a127eaab39f1ba02740e3e67d69cb36a699c))

## [0.9.0](https://github.com/fluencelabs/aquavm/compare/air-parser-v0.8.2...air-parser-v0.9.0) (2023-09-07)


### ⚠ BREAKING CHANGES

* **execution-engine:** canon stream map support [fixes VM-301] ([#648](https://github.com/fluencelabs/aquavm/issues/648))

### Features

* **execution-engine:** canon stream map support [fixes VM-301] ([#648](https://github.com/fluencelabs/aquavm/issues/648)) ([b4cbf8f](https://github.com/fluencelabs/aquavm/commit/b4cbf8f621b77ba2031900f021bf20d0f27e34b8))

## [0.8.2](https://github.com/fluencelabs/aquavm/compare/air-parser-v0.8.1...air-parser-v0.8.2) (2023-09-04)


### Features

* **execution-engine:** a new :error: runtime attribute according with FLIP-11 [fixes VM-329] ([#683](https://github.com/fluencelabs/aquavm/issues/683)) ([20afb79](https://github.com/fluencelabs/aquavm/commit/20afb79e3f345b83c367357171f1802ed2db0a66))

## [0.8.1](https://github.com/fluencelabs/aquavm/compare/air-parser-v0.8.0...air-parser-v0.8.1) (2023-08-03)


### Features

* **air-parser:** canon stream syntax ([#618](https://github.com/fluencelabs/aquavm/issues/618)) ([8871465](https://github.com/fluencelabs/aquavm/commit/88714653247618e72f10391524c430a5c20d3b85))
* **air-parser:** improved canon stream syntax support [fixes VM-293] ([8871465](https://github.com/fluencelabs/aquavm/commit/88714653247618e72f10391524c430a5c20d3b85))

## [0.8.0](https://github.com/fluencelabs/aquavm/compare/air-parser-v0.7.5...air-parser-v0.8.0) (2023-07-16)


### ⚠ BREAKING CHANGES

* **execution-engine:** stream map to scalar conversion using canon instruction [fixes VM-294] ([#610](https://github.com/fluencelabs/aquavm/issues/610))

### Features

* **execution-engine:** Stream Map to Scalar conversion using canon instruction [fixes VM-294] ([fcb4c9d](https://github.com/fluencelabs/aquavm/commit/fcb4c9dab43d82e87f1d6f8a83b72f6891d37bef))
* **execution-engine:** stream map to scalar conversion using canon instruction [fixes VM-294] ([#610](https://github.com/fluencelabs/aquavm/issues/610)) ([fcb4c9d](https://github.com/fluencelabs/aquavm/commit/fcb4c9dab43d82e87f1d6f8a83b72f6891d37bef))

## [0.7.5](https://github.com/fluencelabs/aquavm/compare/air-parser-v0.7.4...air-parser-v0.7.5) (2023-06-22)


### Features

* **execution-engine:** Stream Map initial support [fixes VM-283,VM-284] ([#592](https://github.com/fluencelabs/aquavm/issues/592)) ([9d7d34a](https://github.com/fluencelabs/aquavm/commit/9d7d34a452cb65e968ed68decc67f3bc523a5115))
* **execution-engine:** StreamMap initial support for ap and new instructions [fixes VM-283,VM-284] ([9d7d34a](https://github.com/fluencelabs/aquavm/commit/9d7d34a452cb65e968ed68decc67f3bc523a5115))

## [0.7.4](https://github.com/fluencelabs/aquavm/compare/air-parser-v0.7.3...air-parser-v0.7.4) (2023-03-15)


### Features

* **tools:** merge some tools into the `air` CLI tool ([#509](https://github.com/fluencelabs/aquavm/issues/509)) ([79ac153](https://github.com/fluencelabs/aquavm/commit/79ac153f1dcfc0a77ec511c6e25285728312ad4c))

## [0.7.3](https://github.com/fluencelabs/aquavm/compare/air-parser-v0.7.2...air-parser-v0.7.3) (2023-02-21)


### ⚠ BREAKING CHANGES

* **avm:** improve anomaly detection

### Features

* **air-parser:** improve docs ([#483](https://github.com/fluencelabs/aquavm/issues/483)) ([ae3a8e9](https://github.com/fluencelabs/aquavm/commit/ae3a8e9a503f0ef4c2ec87be7f620d57f3483817))
* **avm:** improve anomaly detection ([5e6863d](https://github.com/fluencelabs/aquavm/commit/5e6863d4d59684d4f2b509ece6e597831e648f05))
* improve parser docs ([ae3a8e9](https://github.com/fluencelabs/aquavm/commit/ae3a8e9a503f0ef4c2ec87be7f620d57f3483817))
