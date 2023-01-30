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

from .helpers import format_timedelta, parse_trace_timedelta
from typing import Optional
from itertools import zip_longest


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
        else:
            return "{}::{}".format(self.target, self.span)

    def to_json(self, repeat: int) -> dict:
        """Convert trace to JSON report."""
        result = {
            'duration': format_timedelta(self.execution_time / repeat)
        }
        if self.nested:
            result['nested'] = {
                fname: trace_record.to_json(repeat)
                for (fname, trace_record) in self.nested.items()
            }
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

    def __init__(self):
        """Create a walker."""
        self.stack = []
        self.root = {}

    def process(self, records):
        """Handle all input records, building a call tree in the `root` field."""
        for raw_rec in records:
            logger.debug("raw_rec %r", raw_rec)
            message = raw_rec["fields"]["message"]
            if message in ("enter", "close"):
                span = raw_rec["span"].get("name", "ERROR_missing_span.name")
                target = raw_rec.get("target", None)
                spans = [sp["name"] for sp in raw_rec.get("spans", [])]
                logger.debug("Message: %r", message)
                if message == "close":
                    time_busy = raw_rec["fields"].get("time.busy")
                    rec = self.stack.pop()
                    logger.debug("Poped %r from %r", rec, self.stack)
                    real_rec = self._get_closing_rec(rec)
                    assert rec == real_rec, "{!r} vs {!r}".format(
                        rec, real_rec)
                    rec.execution_time += parse_trace_timedelta(time_busy)
                elif message == "enter":
                    assert span == spans[-1]
                    rec = TraceRecord(message, span, target, None, spans[:-1])
                    self._inject_enter_rec(rec)

    def to_json(self, repeat: int):
        """Convert to JSON."""
        assert not self.stack
        return {
            fname: trace_record.to_json(repeat)
            for (fname, trace_record) in self.root.items()
        }

    def _find_parent(self, rec: TraceRecord) -> TraceRecord:
        parent = _RootStub(self.root)

        for (sp1, tr2) in zip_longest(rec.spans, self.stack):
            # Validity check.  Should hold for single-threaded app.
            assert tr2 is not None, "{!r} vs {!r}".format(
                rec.spans, self.stack)
            assert sp1 == tr2.get_span(), "{!r} vs {!r}".format(
                rec.spans, self.stack)
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
