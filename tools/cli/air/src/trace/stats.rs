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

mod log_data;
mod report;

use self::log_data::{LogRecord, Message};

use clap::Parser;

#[derive(Parser)]
#[clap(about = "Print human-readable AquaVM JSON trace or provide execution stats")]
pub(crate) struct Args {
    #[clap(long)]
    pretty: bool,
    #[clap(long)]
    stats: bool,

    #[clap(long)]
    sort_stats_by_duration: bool,
}

pub(crate) fn stats(mut args: Args) -> eyre::Result<()> {
    use std::io::Write;

    if !args.pretty && !args.stats {
        args.pretty = true;
        args.stats = true;
    }

    let stderr = std::io::stderr();
    let mut stderr = stderr.lock();
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut stats = self::report::StatsReport::new();

    #[allow(clippy::significant_drop_in_scrutinee)]
    for rec in read_logs(stdin) {
        let rec = rec?;

        if args.pretty {
            print_log_record(&mut stderr, &rec)?;
        }
        if args.stats {
            stats.consider(rec)?;
        }
    }

    if args.stats {
        if args.pretty {
            writeln!(stderr)?;
        }
        stats.report(&mut stderr, args.sort_stats_by_duration)?;
    }
    Ok(())
}

fn read_logs<R: std::io::BufRead>(input: R) -> impl Iterator<Item = eyre::Result<LogRecord>> {
    input.lines().filter_map(|r| match r {
        Ok(line) => {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                Some(serde_json::from_str(line).map_err(eyre::Error::from))
            }
        }
        Err(err) => Some(Err(err.into())),
    })
}

fn print_log_record<W: std::io::Write>(mut out: W, log_record: &LogRecord) -> std::io::Result<()> {
    use itertools::Itertools as _;

    let val = &log_record.value;

    write!(
        out,
        "{timestamp}  {level} ",
        timestamp = val.timestamp,
        level = val.level,
    )?;
    if !val.spans.is_empty() {
        write!(out, "{spans}", spans = val.spans.iter().join(":"),)?;
    }
    if matches!(&val.fields, Message::Close(_)) {
        if !val.spans.is_empty() {
            write!(out, ":")?;
        }
        write!(out, "{span}", span = log_record.span)?;
    }
    writeln!(
        out,
        ": {target}: {fields}",
        target = log_record.target,
        fields = val.fields,
    )
}
