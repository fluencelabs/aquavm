#!/usr/bin/env python3
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
