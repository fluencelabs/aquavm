#!/usr/bin/env python3
"""An AquaVM performance metering tool."""

import argparse
import logging

from . import db
from . import run


def main():
    """Run main function."""
    parser = argparse.ArgumentParser()
    subp = parser.add_subparsers(dest='command')

    run_subparser = subp.add_parser("run")
    run_subparser.add_argument("--path", required=False, type=str)
    run_subparser.add_argument("--host-id", required=False, type=str)
    run_subparser.add_argument("--bench-dir", required=False, type=str)

    args = parser.parse_args()

    if args.command == 'run':
        run.run(args)
    else:
        parser.error("Unknown command {!r}".format(args.command))


if __name__ == '__main__':
    logging.basicConfig(level=logging.INFO)
    main()
