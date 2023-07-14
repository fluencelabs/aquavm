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
"""Trace stateful processing."""

import datetime
import logging
from itertools import zip_longest
from typing import Optional, List
import humanize

from .helpers import format_timedelta, parse_trace_timedelta

logger = logging.getLogger(__name__)


class TraceRecord:
    """Trace record grouped by fully-qualified function name."""

    message: str
    span: str
    target: str
    execution_time: datetime.timedelta
    spans: list
    nested: dict

    def __init__(
            self,
            message: str,
            span: str,
            target: str,
            raw_time: Optional[str],
            spans: list
    ):
        """Create a TraceRecord instance."""
        self.message = message
        self.span = span
        self.target = target
        self.execution_time = parse_trace_timedelta(raw_time)
        self.spans = spans
        self.nested = {}

    def get_span(self):
        """Get current span."""
        return self.span

    def get_parents(self):
        """Get parent spans."""
        return iter(self.spans)

    def get_func_name(self) -> str:
        """Return qualified function name."""
        if self.target is None:
            return self.span
        return f"{self.target}::{self.span}"

    def to_json(self, repeat: int) -> dict:
        """Convert trace to JSON report."""
        duration = format_timedelta(self.execution_time / repeat)
        if self.nested:
            prefix = _common_prefix(self.nested)
            nested = {
                _split_prefix(fname, prefix): trace_record.to_json(repeat)
                for (fname, trace_record) in self.nested.items()
            }

            result = {
                "common_prefix": "::".join(prefix),
                "duration": duration,
                "nested": nested,
            }
        else:
            result = duration
        return result

    def __repr__(self):
        """Return debug representation."""
        return "{}@{}<span={!r}, spans={!r}, time={}, nested={!r}>".format(
            self.__class__.__name__,
            id(self),
            self.span,
            self.spans,
            self.execution_time,
            self.nested,
        )


def _common_prefix(nested: dict) -> List[str]:
    items = iter(nested.keys())
    prefix = next(items).split("::")[:-1]
    for fname in items:
        fname_split = fname.split("::")[:-1]
        new_prefix = []
        for (old, new) in zip(prefix, fname_split):
            if old == new:
                new_prefix.append(old)
            else:
                break
        prefix = new_prefix
    return prefix


def _split_prefix(fname, prefix):
    fname_prefix = fname.split("::")[len(prefix):]
    logger.debug("split_prefix %r -> %r", fname, fname_prefix)
    return "::".join(fname_prefix)


class _RootStub:
    nested: dict

    def __init__(self, root):
        self.nested = root


class TraceWalker:
    """Trace stateful processing: convert a sequence of trace events
    into a call tree."""

    stack: list
    # Maps from fully-qualified func name to a trace
    root: dict

    memory_sizes: List[int]

    def __init__(self):
        """Create a walker."""
        self.stack = []
        self.root = {}
        self.memory_sizes = []

    def process(self, records):
        """With all input records, building a call tree in the `root` field."""
        for raw_rec in records:
            logger.debug("raw_rec %r", raw_rec)
            raw_fields = raw_rec["fields"]

            if "message" in raw_fields:
                message = raw_fields["message"]
                if message in ("enter", "close"):
                    span = raw_rec["span"].get(
                        "name", "ERROR_missing_span.name")
                    target = raw_rec.get("target", None)
                    spans = [sp["name"] for sp in raw_rec.get("spans", [])]
                    logger.debug("Message: %r", message)
                    if message == "close":
                        time_busy = raw_rec["fields"].get("time.busy")
                        rec = self.stack.pop()
                        logger.debug("Poped %r from %r", rec, self.stack)
                        real_rec = self._get_closing_rec(rec)
                        assert rec == real_rec, f"{rec!r} vs {real_rec!r}"
                        rec.execution_time += parse_trace_timedelta(time_busy)
                    elif message == "enter":
                        assert span == spans[-1]
                        rec = TraceRecord(
                            message, span, target, None, spans[:-1])
                        self._inject_enter_rec(rec)
            if "memory_size" in raw_fields:
                self._handle_memory_stat(raw_fields["memory_size"])

    def to_json(self, repeat: int):
        """Convert to JSON."""
        assert not self.stack
        return {
            fname: trace_record.to_json(repeat)
            for (fname, trace_record) in self.root.items()
        }

    def get_total_time(self, repeat: int):
        """Get total execution time."""
        assert not self.stack
        root_time = sum(
            (node.execution_time for node in self.root.values()),
            start=datetime.timedelta()
        ) / repeat
        return format_timedelta(root_time)

    def get_memory_sizes(self, repeat: int) -> Optional[str]:
        """Get average memory size."""
        def format_size(size):
            return humanize.naturalsize(size, binary=True, format="%.3f")

        if self.memory_sizes:
            self.memory_sizes.sort()
            min_size = self.memory_sizes[0]
            max_size = self.memory_sizes[-1]
            return [format_size(min_size), format_size(max_size)]
        return None

    def _find_parent(self, rec: TraceRecord) -> TraceRecord:
        parent = _RootStub(self.root)

        for (sp1, tr2) in zip_longest(rec.spans, self.stack):
            # Validity check.  Should hold for single-threaded app.
            assert tr2 is not None, f"{rec.spans!r} vs {self.stack!r}"
            assert sp1 == tr2.get_span(), f"{rec.spans!r} vs {self.stack!r}"
            parent = parent.nested[tr2.get_func_name()]
        return parent

    def _inject_enter_rec(self, rec: TraceRecord):
        parent = self._find_parent(rec)

        fname = rec.get_func_name()
        if fname not in parent.nested:
            logger.debug("Inserting %r to %r", rec, parent)
            parent.nested[fname] = rec
        else:
            rec = parent.nested[fname]
        logger.debug("Push %r to %r", rec, self.stack)
        self.stack.append(rec)

    def _get_closing_rec(self, rec: TraceRecord):
        parent = self._find_parent(rec)

        fname = rec.get_func_name()
        real_rec = parent.nested[fname]
        return real_rec

    def _handle_memory_stat(self, size: int):
        self.memory_sizes.append(size)
