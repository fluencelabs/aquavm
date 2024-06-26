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
"""Performance measurement database module."""

import datetime
import json
import logging
import platform
from typing import List, Optional

from .helpers import (
    canonicalize_features, get_host_id, get_aquavm_version,
    intermediate_temp_file
)

DEFAULT_JSON_PATH = "benches/PERFORMANCE.json"
AQUAVM_TOML_PATH = "air/Cargo.toml"


class Db:
    """Performance measurement database."""

    json_path: str
    host_id: str
    data: hash

    def __init__(
        self,
        json_path: Optional[str],
        host_id=None,
        features: Optional[str] = None,
        merge_results: bool = False,
    ):
        """Load data from file, if it exits."""
        if json_path is None:
            json_path = DEFAULT_JSON_PATH
        self.json_path = json_path

        if host_id is None:
            host_id = get_host_id()
        self.host_id = host_id

        self.features = canonicalize_features(features) or ''

        try:
            with open(json_path, 'r', encoding="utf-8") as inp:
                self.data = json.load(inp)
        except IOError as ex:
            logging.warning("cannot open data at %r: %s", json_path, ex)
            self.data = {}
        if not merge_results:
            # clean previous results
            self.data.pop(self.host_id, None)

    def record(
            self, bench, stats, total_time, memory_sizes: Optional[List[str]]
    ):
        """Record the bench stats."""
        if self.host_id not in self.data:
            self.data[self.host_id] = {"benches": {}}
        bench_name = bench.get_name()

        bench_info = {
            "stats": stats,
            "total_time": total_time,
        }
        if memory_sizes is not None:
            bench_info["memory_sizes"] = memory_sizes

        comment = bench.get_comment()
        if comment is not None:
            bench_info["comment"] = comment
        self.data[self.host_id]["benches"][bench_name] = bench_info
        self.data[self.host_id]["platform"] = platform.platform()

        self.data[self.host_id]["features"] = self.features

        self.data[self.host_id]["datetime"] = str(
            datetime.datetime.now(datetime.timezone.utc)
        )
        self.data[self.host_id]["version"] = get_aquavm_version(
            AQUAVM_TOML_PATH
        )

    def save(self):
        """Save the database to JSON."""
        with intermediate_temp_file(self.json_path) as out:
            json.dump(
                self.data, out,
                # for better diffs and readable files:
                sort_keys=True,
                indent=2,
                ensure_ascii=False,
            )
            # Add a new line for data readability
            print("", file=out)

    def __enter__(self):
        """Enter context manager."""
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        """Exit context manger, saving data if the exit is clean."""
        if exc_type is None:
            self.save()
