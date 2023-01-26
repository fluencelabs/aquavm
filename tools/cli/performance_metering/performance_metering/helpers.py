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
"""Helper functions for performance_metering."""
import datetime

# The ordering of elements is important.
TIME_SUFFIXES = [("ns", 1e-9), ("µs", 1e-6), ("ms", 1e-3), ("s", 1e0)]


def parse_trace_timedelta(inp: str) -> datetime.timedelta:
    """Parse `tracing`-formatted execution times."""
    for (suffix, scale) in TIME_SUFFIXES:
        if inp.endswith(suffix):
            val = float(inp[:-len(suffix)])
            seconds = val * scale
            return datetime.timedelta(seconds=seconds)
    else:
        raise ValueError("Unknown time suffix")


def format_timedelta(td: datetime.timedelta) -> str:
    """Print execution times to `tracing` format."""
    seconds = td.total_seconds()
    for (suffix, scale) in reversed(TIME_SUFFIXES):
        if seconds >= scale:
            return "{:0.2f}{}".format(seconds / scale, suffix)
    else:
        (suffix, scale) = TIME_SUFFIXES[-1]
        return "{:0.2f}{}".format(seconds / scale, suffix)


def get_host_id() -> str:
    """Return a hash of host id."""
    import socket
    import hashlib

    hostname = socket.gethostname().encode('utf-8')
    return hashlib.sha256(hostname).hexdigest()


def get_aquavm_version(path: str) -> str:
    """Get `version` field from a TOML file."""
    import toml
    with open(path, 'r') as inp:
        data = toml.load(inp)
    return data['package']['version']
