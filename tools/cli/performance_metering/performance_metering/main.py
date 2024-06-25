#!/usr/bin/env python3
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
"""An AquaVM performance metering tool."""

import argparse
import logging

from . import run


def main():
    """Run main function."""
    logging.basicConfig(level=logging.INFO)

    parser = argparse.ArgumentParser()
    subp = parser.add_subparsers(dest='command')

    run_subparser = subp.add_parser("run")
    run_subparser.add_argument("--path", required=False, type=str)
    run_subparser.add_argument("--report-path", required=False, type=str)
    run_subparser.add_argument("--host-id", required=False, type=str)
    run_subparser.add_argument("--bench-dir", required=False, type=str)
    run_subparser.add_argument("--repeat", required=False, type=int, default=1)
    run_subparser.add_argument("--features", required=False)
    run_subparser.add_argument("--unsafe-merge-results", action="store_true")
    run_subparser.add_argument(
        "--no-prepare-binaries",
        action='store_false',
        dest='prepare_binaries',
    )
    run_subparser.add_argument("--tracing-params", type=str, default="info")

    args = parser.parse_args()

    if args.command == 'run':
        run.run(args)
    else:
        parser.error(f"Unknown command {args.command!r}")


if __name__ == '__main__':
    main()
