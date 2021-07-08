/*
 * Copyright 2021 Fluence Labs Limited
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

use super::*;

use serde_json::json;

fn generate_test_trace() -> ExecutionTrace {
    let value = json!([]);
    let trace = vec![
        scalar_jvalue(value.clone()),
        scalar_jvalue(value.clone()),
        scalar_jvalue(value.clone()),
        scalar_jvalue(value.clone()),
        scalar_jvalue(value),
    ];

    trace.into()
}

#[test]
fn slider_is_exhaustive() {
    let trace = generate_test_trace();

    let slider = TraceSlider::new(trace.clone().into());

    assert_eq!(slider.subtrace_len(), trace.len());

    let trace_len = trace.len();
    for (id, state) in trace.into_iter().enumerate() {
        let actual_value = slider.next_state();
        let expected_value = Some(state);
        assert_eq!(actual_value, expected_value);

        assert_eq!(slider.subtrace_len(), trace_len - id - 1);
    }

    let actual_value = slider.next_state();
    assert_eq!(actual_value, None);
}

#[test]
fn none_on_empty() {
    let trace = vec![];
    let slider = TraceSlider::new(trace.clone().into());

    assert_eq!(slider.subtrace_len(), trace.len());

    let actual_value = slider.next_state();
    assert_eq!(actual_value, None);
}

#[test]
fn none_on_empty_interval() {
    let trace = generate_test_trace();

    let slider = TraceSlider::new(trace.clone().into());
    slider.set_subtrace_len(0);

    let actual_value = slider.next_state();
    assert_eq!(actual_value, None);

    let mut actual_remainder = slider.remaining_interval().unwrap();
    assert_eq!(actual_remainder.next(), None);
}

#[test]
fn remainder_on_subinterval() {
    let mut trace = generate_test_trace();

    let slider = TraceSlider::new(trace.clone().into());
    let actual_value = slider.next_state();
    assert_eq!(actual_value, Some(trace.remove(0).unwrap()));

    let actual_remainder = slider.remaining_interval().unwrap();
    assert_eq!(actual_remainder.len(), trace.len());
}

#[test]
fn none_after_remainder() {
    let trace = generate_test_trace();

    let slider = TraceSlider::new(trace.clone().into());

    let actual_remainder = slider.remaining_interval().unwrap();
    assert_eq!(actual_remainder.len(), trace.len());

    let actual_value = slider.next_state();
    assert_eq!(actual_value, None);
    assert_eq!(slider.subtrace_len(), 0);
}
