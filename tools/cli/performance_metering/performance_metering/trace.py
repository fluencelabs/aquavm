#
#  Copyright 2023 Fluence Labs Limited
#
#  Licensed under the Apache License, Version 2.0 (the "License");
#  you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
#
"""Trace record handling."""

import datetime
from typing import Optional


class TraceRecord:
    """A parsed trace record."""

    span: str
    execution_time: datetime.timedelta
    raw_time: str

    def __init__(self, span, raw_time):
        """Create a TraceRecord instance."""
        self.span = span
        self.execution_time = _parse_trace_timedelta(raw_time)
        self.raw_time = raw_time


def parse_if_interesting(raw_rec: dict) -> Optional[TraceRecord]:
    """If trace is interesting, parse it; return None otherwise."""
    try:
        if raw_rec["fields"]["message"] == "close":
            span = raw_rec["span"]["name"]
            time = raw_rec["fields"]["time.busy"]
            return TraceRecord(span, time)
    except KeyError:
        return None
    return None


def combine_traces(traces: list[TraceRecord], repeat: int):
    """Calculate cumulutive time for each span."""
    from collections import defaultdict
    combined = defaultdict(datetime.timedelta)

    for trace in traces:
        combined[trace.span] += trace.execution_time

    return {
        span: str(time / repeat) for (span, time) in combined.items()
    }


def _parse_trace_timedelta(inp: str) -> datetime.timedelta:
    for (suffix, scale) in [
            ("ns", 1e-9), ("µs", 1e-6), ("ms", 1e-3), ("s", 1e0)
    ]:
        if inp.endswith(suffix):
            val = float(inp[:-len(suffix)])
            seconds = val * scale
            return datetime.timedelta(seconds=seconds)
    else:
        raise ValueError("Unknown time suffix")
