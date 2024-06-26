#
# AquaVM Workflow Engine
#
# Copyright (C) 2024 Fluence DAO
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation version 3 of the
# License.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
#
"""Helper functions for performance_metering."""
import datetime
import hashlib
import os.path
import socket
import tempfile
from contextlib import contextmanager
from typing import Optional

import toml

# The ordering of elements is important.
TIME_SUFFIXES = [("ns", 1e-9), ("µs", 1e-6), ("ms", 1e-3), ("s", 1e0)]


def parse_trace_timedelta(inp: Optional[str]) -> datetime.timedelta:
    """Parse `tracing`-formatted execution times."""
    if inp is None:
        return datetime.timedelta()
    for (suffix, scale) in TIME_SUFFIXES:
        if inp.endswith(suffix):
            val = float(inp[:-len(suffix)])
            seconds = val * scale
            return datetime.timedelta(seconds=seconds)
    raise ValueError("Unknown time suffix")


def format_timedelta(td: datetime.timedelta) -> str:
    """Print execution times to `tracing` format."""
    seconds = td.total_seconds()
    for (suffix, scale) in reversed(TIME_SUFFIXES):
        if seconds >= scale:
            return f"{seconds / scale:0.2f}{suffix}"
    # else
    (suffix, scale) = TIME_SUFFIXES[-1]
    return f"{seconds / scale:0.2f}{suffix}"


def get_host_id() -> str:
    """Return a hash of host id."""

    hostname = socket.gethostname().encode('utf-8')
    return hashlib.sha256(hostname).hexdigest()


def get_aquavm_version(path: str) -> str:
    """Get `version` field from a TOML file."""
    with open(path, 'r', encoding="utf8") as inp:
        data = toml.load(inp)
    return data['package']['version']


@contextmanager
def intermediate_temp_file(target_file: str):
    """
    Context manager that create an intermediate temp file.

    It to be used as an itermediate for owerwriting the target file on
    success.
    """
    out = tempfile.NamedTemporaryFile(
        mode="w+",
        dir=os.path.dirname(target_file),
        prefix=os.path.basename(target_file) + ".",
        encoding="utf-8",
        delete=False,
    )
    try:
        yield out
        out.flush()
        os.rename(out.name, target_file)
        out = None
    finally:
        if out is not None:
            out.close()
            try:
                os.remove(out.name)
            except OSError:
                pass


def canonicalize_features(features: Optional[str]) -> Optional[str]:
    """Canonicalize comma-separate Rust feature list."""
    if features is None:
        return None
    uniq_features = set(features.split(','))
    sorted_features = sorted(uniq_features)
    return ','.join(sorted_features)
