/*
 * Copyright 2022 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

use avm_interface::raw_outcome::RawAVMOutcome;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::time::Duration;

/// This trait is used for
///   - persisting prev_data between successive calls of an interpreter
///   - logging previous, current, and new data in case of spikes
pub trait DataStore {
    type Error;

    fn initialize(&mut self) -> Result<(), Self::Error>;

    fn store_data(&mut self, data: &[u8], key: &str) -> Result<(), Self::Error>;

    fn read_data(&mut self, key: &str) -> Result<Vec<u8>, Self::Error>;

    /// Cleanup data that become obsolete.
    fn cleanup_data(&mut self, key: &str) -> Result<(), Self::Error>;

    /// Returns true if an anomaly happened and it's necessary to save execution data
    /// for debugging purposes.
    ///  execution_time - time taken by the interpreter to execute provided script
    ///  memory_delta - a count of bytes on which an interpreter heap has been extended
    ///                 during execution of a particle
    ///  outcome - a result of AquaVM invocation
    fn detect_anomaly(
        &self,
        execution_time: Duration,
        memory_delta: usize,
        outcome: &RawAVMOutcome,
    ) -> bool;

    fn collect_anomaly_data(
        &mut self,
        key: &str,
        anomaly_data: AnomalyData<'_>,
    ) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnomalyData<'data> {
    #[serde(borrow)]
    pub air_script: Cow<'data, str>,
    #[serde(borrow, with = "serde_bytes")]
    pub particle: Cow<'data, [u8]>, // it's byte because of the restriction on trait objects methods
    #[serde(borrow, with = "serde_bytes")]
    pub prev_data: Cow<'data, [u8]>,
    #[serde(borrow, with = "serde_bytes")]
    pub current_data: Cow<'data, [u8]>,
    #[serde(borrow, with = "serde_bytes")]
    pub call_results: Cow<'data, [u8]>,
    #[serde(borrow, with = "serde_bytes")]
    pub avm_outcome: Cow<'data, [u8]>,
    pub execution_time: Duration,
    pub memory_delta: usize,
}

impl<'data> AnomalyData<'data> {
    pub fn new(
        air_script: &'data str,
        particle: &'data [u8],
        prev_data: &'data [u8],
        current_data: &'data [u8],
        call_results: &'data [u8],
        avm_outcome: &'data [u8],
        execution_time: Duration,
        memory_delta: usize,
    ) -> Self {
        Self {
            air_script: air_script.into(),
            particle: particle.into(),
            prev_data: prev_data.into(),
            current_data: current_data.into(),
            call_results: call_results.into(),
            avm_outcome: avm_outcome.into(),
            execution_time,
            memory_delta,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn anomaly_json(
        air_script: &str,
        particle: &[u8],
        prev_data: &[u8],
        current_data: &[u8],
        call_results: &[u8],
        avm_outcome: &[u8],
    ) -> String {
        format!(
            concat!(
                r#"{{"air_script":{air_script},"#,
                r#""particle":{particle},"#,
                r#""prev_data":{prev_data},"#,
                r#""current_data":{current_data},"#,
                r#""call_results":{call_results},"#,
                r#""avm_outcome":{avm_outcome},"#,
                r#""execution_time":{{"secs":42,"nanos":0}},"#,
                r#""memory_delta":123"#,
                r#"}}"#
            ),
            air_script = serde_json::to_string(air_script).unwrap(),
            particle = serde_json::to_string(particle).unwrap(),
            prev_data = serde_json::to_string(prev_data).unwrap(),
            current_data = serde_json::to_string(current_data).unwrap(),
            call_results = serde_json::to_string(call_results).unwrap(),
            avm_outcome = serde_json::to_string(avm_outcome).unwrap(),
        )
    }
    #[test]
    fn anomaly_data_se() {
        let anomaly = AnomalyData::new(
            "(null)",
            br#"{"data":"value"}"#,  // not real data
            br#"{"trace":[]}"#,      // not real data
            br#"{"trace":[1,2,3]}"#, // not real data
            b"{}",                   // not real data
            b"{}",
            Duration::from_secs(42),
            123,
        );

        let json_data = serde_json::to_string(&anomaly).expect("JSON serialize anomaly data");
        let expected = anomaly_json(
            &anomaly.air_script,
            &anomaly.particle,
            &anomaly.prev_data,
            &anomaly.current_data,
            &anomaly.call_results,
            &anomaly.avm_outcome,
        );
        assert_eq!(json_data, expected);
    }

    #[test]
    fn anomaly_data_de() {
        let particle = br#"{"particle":"data"}"#;
        let current_data = br#"{"data":"current"}"#;
        let prev_data = br#"{"data":"prev"}"#;
        let avm_outcome = br#"{"avm":[1,2,3]}"#;
        let call_results = br#"{"call_results": "excellent result"}"#;
        let json_data = anomaly_json(
            "(null)",
            &particle[..],
            &prev_data[..],
            &current_data[..],
            &call_results[..],
            &avm_outcome[..],
        );

        let anomaly: AnomalyData<'_> =
            serde_json::from_str(&json_data).expect("deserialize JSON anomaly data");

        assert_eq!(
            anomaly,
            AnomalyData::new(
                "(null)",
                &particle[..],
                &prev_data[..],
                &current_data[..],
                &call_results[..],
                &avm_outcome[..],
                Duration::from_secs(42),
                123,
            )
        )
    }
}
