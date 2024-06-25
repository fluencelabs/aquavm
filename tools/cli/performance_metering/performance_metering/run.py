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
"""Running benches."""

import logging
import operator
import os
import subprocess
import typing

from .bench import Bench
from .db import Db
from .helpers import intermediate_temp_file
from .text_report import TextReporter
from .trace_walker import TraceWalker

DEFAULT_TEST_DIR = "benches/performance_metering"
DEFAULT_REPORT_PATH = "benches/PERFORMANCE.txt"

logger = logging.getLogger(__name__)


def _prepare(args):
    """Prepare the environment: build the tools required."""
    if args.prepare_binaries:
        if args.features:
            features = 'marine,' + args.features
        else:
            features = 'marine'
        logger.info("Build air-interpreter...")
        subprocess.check_call([
            "marine", "build", "--release", "--features", features,
            "--package", "air-interpreter",
        ])
        logger.info("Build air-trace...")
        subprocess.check_call([
            "cargo", "build", "--release", "--package", "aquavm-air-cli",
        ])


def discover_tests(bench_dir: typing.Optional[str]) -> typing.List[Bench]:
    """Discover bench suite elements."""
    if bench_dir is None:
        bench_dir = DEFAULT_TEST_DIR
    return [
        Bench(ent.path)
        for ent in sorted(
                os.scandir(bench_dir),
                key=operator.attrgetter('name'),
        )
        if ent.is_dir(follow_symlinks=True)
    ]


def run(args):
    """Run test suite, saving results to database."""
    _prepare(args)

    suite = discover_tests(args.bench_dir)
    with Db(
            args.path,
            features=args.features,
            host_id=args.host_id,
            merge_results=args.unsafe_merge_results,
    ) as db:
        for bench in suite:
            raw_stats = bench.run(args.repeat, args.tracing_params)
            walker = TraceWalker()
            walker.process(raw_stats)

            combined_stats = walker.to_json(args.repeat)
            total_time = walker.get_total_time(args.repeat)
            memory_sizes = walker.get_memory_sizes(args.repeat)
            db.record(bench, combined_stats, total_time, memory_sizes)

        with intermediate_temp_file(args.report_path or DEFAULT_REPORT_PATH) as out:
            report = TextReporter(db.data)
            report.save_text_report(out)
