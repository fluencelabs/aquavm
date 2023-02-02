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
"""Performance measurement database module."""

import datetime
import json
import logging
import platform
from typing import Optional

from .helpers import get_host_id, get_aquavm_version, intermediate_temp_file

DEFAULT_JSON_PATH = "benches/PERFORMANCE.json"
DEFAULT_TEXT_PATH = "benches/PERFORMANCE.yaml"
AQUAVM_TOML_PATH = "air/Cargo.toml"


class Db:
    """Performance measurement database."""

    path: str
    host_id: str
    data: hash

    def __init__(
            self,
            json_path: Optional[str],
            text_path: Optional[str],
            host_id=None
    ):
        """Load data from file, if it exits."""
        if json_path is None:
            json_path = DEFAULT_JSON_PATH
        self.json_path = json_path

        if text_path is None:
            text_path = DEFAULT_TEXT_PATH
        self.text_path = text_path

        if host_id is None:
            host_id = get_host_id()
        self.host_id = host_id

        try:
            with open(json_path, 'r', encoding="utf-8") as inp:
                self.data = json.load(inp)
        except IOError as ex:
            logging.warning("cannot open data at %r: %s", json_path, ex)
            self.data = {}

    def record(self, bench, stats):
        """Record the bench stats."""
        if self.host_id not in self.data:
            self.data[self.host_id] = {"benches": {}}
        bench_name = bench.get_name()

        self.data[self.host_id]["benches"][bench_name] = {
            "stats": stats,
        }
        self.data[self.host_id]["platform"] = platform.platform()
        self.data[self.host_id]["datetime"] = str(
            datetime.datetime.now(datetime.timezone.utc)
        )
        self.data[self.host_id]["version"] = get_aquavm_version(
            AQUAVM_TOML_PATH
        )

        comment = bench.get_comment()
        if comment is not None:
            self.data[self.host_id]["benches"][bench_name]["comment"] = comment

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
