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

use super::log_data::{LogKey, LogRecord, Message};
use crate::utils::parse_tracing_duration;

use std::{collections::HashMap, time::Duration};

#[derive(Default)]
pub(crate) struct StatsReport {
    data: HashMap<LogKey, Duration>,
}

impl StatsReport {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn report<W: std::io::Write>(
        self,
        mut out: W,
        sort_stats_by_duration: bool,
    ) -> std::io::Result<()> {
        writeln!(out, "*** Statistics ***")?;
        let mut stats_data: Vec<_> = self.data.into_iter().collect();
        if sort_stats_by_duration {
            stats_data.sort_unstable_by(|a, b| (a.1, &a.0).cmp(&(b.1, &b.0)).reverse());
        } else {
            stats_data.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        }
        for (k, v) in stats_data {
            writeln!(out, "{} {}: {:?}", k.target, k.span_name, v)?;
        }
        Ok(())
    }

    pub(crate) fn consider(&mut self, rec: LogRecord) -> anyhow::Result<()> {
        if let Message::Close(close) = &rec.value.fields {
            let time_busy = parse_tracing_duration(&close.time_busy)?;
            *self.data.entry(rec.get_key()).or_default() += time_busy;
        }
        Ok(())
    }
}
