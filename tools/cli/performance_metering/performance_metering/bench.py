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
"""A bench module."""
import json
import logging
import os.path
import subprocess
from typing import Optional

logger = logging.getLogger(__name__)


class _Params:
    comment: Optional[str]
    args: dict

    def __init__(self, comment, args):
        self.comment = comment
        self.args = args

    @staticmethod
    def _load_params(bench_path):
        try:
            params_path = os.path.join(bench_path, "params.json")
            with open(params_path, 'r', encoding="utf8") as inp:
                data = json.load(inp)
                comment = data.pop('comment')
                return _Params(comment, data)
        except IOError:
            return _Params(None, {})


class Bench:
    """Single bench consists of `air-trace run` parameters."""

    path: str
    params: _Params
    prev_data_path: str
    cur_data_path: str
    air_script_path: str
    native: bool

    def __init__(self, bench_path: str, native: bool = False):
        """Load data."""
        self.path = bench_path

        self.params = _Params._load_params(bench_path)
        self.prev_data_path = discover_file(bench_path, "prev_data.json")
        self.cur_data_path = discover_file(bench_path, "cur_data.json")
        self.air_script_path = discover_file(bench_path, "script.air")
        self.keypair = discover_file(bench_path, "keypair.ed25519")
        try:
            self.call_results = discover_file(bench_path, "call_results.json")
        except IOError:
            self.call_results = None
        self.native = native

    def run(self, repeat, tracing_params):
        """Run the bench, storing and parsing its output."""
        logger.info("Executing %s...", self.get_name())
        return self._execute(repeat, tracing_params)

    def _execute(self, repeat, tracing_params) -> str:
        all_output = []
        for _ in range(repeat):
            proc = subprocess.run(
                [
                    "cargo", "run",
                    "--quiet",
                    "--release",
                    "--package", "aquavm-air-cli",
                    "--",
                    "run",
                    "--json",
                    "--repeat", "1",
                    "--ed25519-key", self.keypair,
                ] + (
                    ["--native"] if self.native else []
                ) + (
                    ["--call-results", self.call_results] if self.call_results
                    else []
                ) + [
                    "--tracing-params", tracing_params,
                    "--plain",
                    "--current-data", self.cur_data_path,
                    "--prev-data", self.prev_data_path,
                    "--script", self.air_script_path,
                ] + [
                    arg
                    for (param, val) in self.params.args.items()
                    for arg in ('--' + param, val)
                ],
                check=True,
                stderr=subprocess.PIPE,
            )
            lines = proc.stderr.decode("utf-8").split('\n')
            all_output.extend(lines)
        return list(map(json.loads, filter(lambda x: x, all_output)))

    def get_name(self):
        """Return the bench name."""
        return os.path.basename(self.path)

    def get_comment(self):
        """Return the bench comment."""
        return self.params.comment

    def __repr__(self):
        """`repr` implementation."""
        return "Bench({!r}, {!r})".format(
            os.path.basename(self.path),
            self.params
        )


def discover_file(base_dir: str, filename: str) -> str:
    """Return the file in the base_dir, checking it can be read."""
    path = os.path.join(base_dir, filename)
    with open(path, 'r', encoding="utf8"):
        pass
    return path
