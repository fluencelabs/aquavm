/*
 * Copyright 2020 Fluence Labs Limited
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

use std::hash::Hash;

/// Formats aqua script in a form of S-expressions to a form compatible with the serde_sexpr crate.
pub(super) fn format_aqua(aqua: String) -> String {
    use std::iter::FromIterator;

    let mut formatted_aqua = Vec::with_capacity(aqua.len());
    // whether to skip the next whitespace
    let mut skip_next_whitespace = false;
    // whether c was a closing brace
    let mut was_cbr = false;

    for c in aqua.chars() {
        let is_whitespace = c == ' ';
        if (skip_next_whitespace && is_whitespace) || c == '\n' {
            continue;
        }

        let is_cbr = c == ')';

        skip_next_whitespace = is_whitespace || c == '(' || is_cbr;
        if was_cbr && !is_cbr {
            formatted_aqua.push(' ');
        }

        was_cbr = is_cbr;
        formatted_aqua.push(c)
    }

    String::from_iter(formatted_aqua.into_iter())
}

pub(super) fn dedup<T: Eq + Hash>(mut vec: Vec<T>) -> Vec<T> {
    use std::collections::HashSet;

    let set: HashSet<_> = vec.drain(..).collect();
    set.into_iter().collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn format_aqua_test() {
        let aqua = format!(
            r#"(( ((  (seq (
            (call (%current_peer_id% (add_module ||) (module) module))
            (seq (
                (call (%current_peer_id% (add_blueprint ||) (blueprint) blueprint_id))
                (seq (
                    (call (%current_peer_id% (create ||) (blueprint_id) service_id))
                    (call ({} (|| ||) (service_id) client_result))
                )  )
            ) )
        ))"#,
            "abc"
        );

        let aqua = super::format_aqua(aqua);
        let formatted_aqua = String::from("(((((seq ((call (%current_peer_id% (add_module ||) (module) module)) (seq ((call (%current_peer_id% (add_blueprint ||) (blueprint) blueprint_id)) (seq ((call (%current_peer_id% (create ||) (blueprint_id) service_id)) (call (abc (|| ||) (service_id) client_result))))))))");

        assert_eq!(aqua, formatted_aqua);
    }
}
