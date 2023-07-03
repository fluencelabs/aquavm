#!/usr/bin/env python3
"""Reindent util.

It produces formatted JSON with 0 indent for smaller diffs.
"""
import argparse
import json


def main():
    """Run reindent util."""
    parser = argparse.ArgumentParser()
    parser.add_argument("filename")
    args = parser.parse_args()

    with open(args.filename) as inp:
        data = json.load(inp)

    with open(args.filename, "w") as out:
        json.dump(data, out, indent=0)


if __name__ == '__main__':
    main()
