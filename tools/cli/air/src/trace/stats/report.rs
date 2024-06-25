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

use super::super::utils::parse_tracing_duration;
use super::log_data::LogKey;
use super::log_data::LogRecord;
use super::log_data::Message;

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

    pub(crate) fn consider(&mut self, rec: LogRecord) -> eyre::Result<()> {
        if let Message::Close(close) = &rec.value.fields {
            let time_busy = parse_tracing_duration(&close.time_busy)?;
            *self.data.entry(rec.get_key()).or_default() += time_busy;
        }
        Ok(())
    }
}
