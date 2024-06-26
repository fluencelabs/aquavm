/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub(crate) struct LogRecord {
    pub(crate) target: String,
    pub(crate) span: Span,

    #[serde(flatten)]
    pub(crate) value: LogValue,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Span {
    pub(crate) name: String,
    #[serde(flatten)]
    pub(crate) args: HashMap<String, serde_json::Value>,
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct LogKey {
    pub(crate) target: String,
    pub(crate) span_name: String,
}

#[derive(Deserialize)]
pub(crate) struct LogValue {
    pub(crate) timestamp: String,
    pub(crate) fields: Message,
    pub(crate) level: String,
    pub(crate) spans: Vec<Span>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "message", rename_all = "lowercase")]
pub(crate) enum Message {
    New,
    Enter,
    Close(CloseMessage),
}

#[derive(Debug, Deserialize)]
pub(crate) struct CloseMessage {
    #[serde(rename = "time.busy")]
    pub(crate) time_busy: String,
    #[serde(rename = "time.idle")]
    pub(crate) time_idle: String,
}

impl LogRecord {
    pub(crate) fn get_key(&self) -> LogKey {
        LogKey {
            target: self.target.clone(),
            span_name: self.span.name.clone(),
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::New => write!(f, "new"),
            Message::Enter => write!(f, "enter"),
            Message::Close(c) => write!(f, "close {c}"),
        }
    }
}

impl std::fmt::Display for CloseMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "idle={}, busy={}", self.time_idle, self.time_busy)
    }
}

fn format_argument(val: &serde_json::Value) -> String {
    match val {
        serde_json::Value::String(s) => s.to_owned(),
        _ => val.to_string(),
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use itertools::Itertools as _;

        self.name.fmt(f)?;
        if !self.args.is_empty() {
            "{".fmt(f)?;
            write!(
                f,
                "{}",
                self.args
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, format_argument(v)))
                    .format(", ")
            )?;
            "}".fmt(f)?;
        }
        Ok(())
    }
}
