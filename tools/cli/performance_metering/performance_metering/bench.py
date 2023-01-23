"""A bench module."""
import json
import logging
import os.path
import subprocess

logger = logging.getLogger(__name__)


class Bench:
    """Single bench consists of `air-trace run` parameters."""

    path: str
    params: dict
    prev_data_path: str
    cur_data_path: str
    air_script_path: str
    native: bool

    def __init__(self, bench_path: str, native: bool = False):
        """Load data."""
        self.path = bench_path

        self.params = _load_params(bench_path)
        self.prev_data_path = discover_file(bench_path, "prev_data.json")
        self.cur_data_path = discover_file(bench_path, "cur_data.json")
        self.air_script_path = discover_file(bench_path, "script.air")
        self.native = native

    def run(self):
        """Run the bench, storing and parsing its output."""
        logger.info("%s", self.get_name())
        return self._execute()

    def _execute(self) -> str:
        proc = subprocess.run(
            [
                "cargo", "run",
                "--quiet",
                "--release",
                "--package", "air-trace",
                "--",
                "run",
                "--json",
                "--repeat", "1",
            ] + (
                ["--native"] if self.native else []
            ) + [
                "--plain",
                "--data", self.cur_data_path,
                "--prev-data", self.prev_data_path,
                "--script", self.air_script_path,
            ] + [
                arg
                for (param, val) in self.params.items()
                for arg in ('--' + param, val)
            ],
            check=True,
            stderr=subprocess.PIPE,
        )
        lines = proc.stderr.decode("utf-8").split('\n')
        return list(map(json.loads, filter(lambda x: x, lines)))

    def get_name(self):
        """Return the bench name."""
        return os.path.basename(self.path)

    def __repr__(self):
        """`repr` implementation."""
        return "Bench({!r}, {!r})".format(
            os.path.basename(self.path),
            self.params
        )


def _load_params(bench_path) -> dict:
    try:
        params_path = os.path.join(bench_path, "params.json")
        with open(params_path, 'r') as inp:
            return json.load(inp)
    except IOError:
        return {}


def discover_file(base_dir: str, filename: str) -> str:
    """Return the file in the base_dir, checking it can be read."""
    path = os.path.join(base_dir, filename)
    with open(path, 'r'):
        pass
    return path
