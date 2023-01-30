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
from .helpers import format_timedelta, parse_trace_timedelta


class TraceRecord:
    """A parsed trace record."""

    span: str
    target: str
    spans: list
    nested: dict
    execution_time: datetime.timedelta
    raw_time: str

    def __init__(self, span, target, raw_time, spans):
        """Create a TraceRecord instance."""
        self.span = span
        self.execution_time = parse_trace_timedelta(raw_time)
        self.raw_time = raw_time
        self.spans = spans
        self.nested = {}

    def get_func_name(self) -> str:
        """Return qualified function name."""
        if self.target is None:
            return self.span
        else:
            return "{}::{}".format(self.target, self.span)


def parse_if_interesting(raw_rec: dict) -> Optional[TraceRecord]:
    """If trace is interesting, parse it; return None otherwise."""
    try:
        if raw_rec["fields"]["message"] == "close":
            span = raw_rec["span"].get("name", "ERROR_missing_span.name")
            target = raw_rec.get("target", None),
            time = raw_rec["fields"]["time.busy"]
            spans = [sp["name"] for sp in raw_rec.get("spans", [])]
            return TraceRecord(span, target, time, spans)
    except KeyError:
        return None
    return None


def combine_traces(traces: list[TraceRecord], repeat: int):
    """Calculate cumulutive time for each span."""
    from collections import defaultdict
    combined = defaultdict(datetime.timedelta)

    for trace in traces:
        combined[trace.get_func_name()] += trace.execution_time

    return {
        span: format_timedelta(time / repeat)
        for (span, time) in combined.items()
    }
