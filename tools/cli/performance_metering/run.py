"""Running benches module."""
import os
import subprocess
import typing

from .bench import Bench
from .db import Db
from .trace import combine_traces, parse_if_interesting

DEFAULT_TEST_DIR = "tools/cli/performance_metering/benches"


def prepare():
    """Prepare the environment: build the tools required."""
    subprocess.check_call([
        "marine", "build", "--release", "--features", "marine",
        "--package", "air-interpreter",
    ])
    subprocess.check_call([
        "cargo", "build", "--release", "--package", "air-trace",
    ])


def discover_tests(bench_dir: typing.Optional[str]) -> list[Bench]:
    """Discover bench suite elements."""
    if bench_dir is None:
        bench_dir = DEFAULT_TEST_DIR
    return list(map(
        lambda filename: Bench(os.path.join(bench_dir, filename)),
        sorted(os.listdir(bench_dir))
    ))


def run(args):
    """Run test suite, saving results to database."""
    prepare()

    suite = discover_tests(args.bench_dir)
    with Db(args.path, args.host_id) as db:
        for bench in suite:
            raw_stats = bench.run()
            interesting_traces = [
                trace
                for trace in map(parse_if_interesting, raw_stats)
                if trace
            ]
            combined_stats = combine_traces(interesting_traces)
            db.record(bench, combined_stats)
