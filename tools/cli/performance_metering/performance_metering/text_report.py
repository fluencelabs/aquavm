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
"""Human readable text report generation."""


class TextReporter:
    """A generator for human readable text report."""
    data: dict
    indent_step = 2

    def __init__(self, data):
        """Construct a reporter for db data."""
        self.data = data

    def save_text_report(self, file):
        """Save report to the file."""
        for machine_id, machine in _sorted_items(self.data):
            _print_indent("Machine {}:".format(machine_id),
                          indent=0, file=file)
            self._save_machine(machine,  file=file)

    def _save_machine(self, machine, file):
        indent = self.indent_step
        _print_indent("Platform: {}".format(machine["platform"]),
                      indent=indent, file=file)
        _print_indent("Timestamp: {}".format(machine["datetime"]),
                      indent=indent, file=file)
        _print_indent("AquaVM version: {}".format(machine["version"]),
                      indent=indent, file=file)
        _print_indent("Benches:", indent=indent, file=file)

        nested_indent = indent + self.indent_step
        for bench_name, bench in _sorted_items(machine["benches"]):
            self._save_bench(
                bench_name, bench, indent=nested_indent, file=file)

    def _save_bench(self, bench_name, bench, indent, file):
        bracketed_text = bench["total_time"]
        try:
            bracketed_text += '; {}'.format(', '.join(bench['memory_sizes']))
        except KeyError:
            pass
        _print_indent(
            "{} ({}): {}".format(
                bench_name,
                bracketed_text,
                bench["comment"],
            ),
            indent=indent, file=file)
        for fname, stats in _sorted_items(bench["stats"]):
            self._save_stats(fname, stats, indent + self.indent_step, file)

    def _save_stats(self, fname, stats, indent, file):
        if isinstance(stats, dict):
            duration = stats["duration"]

            _print_indent(
                "{}: {}".format(fname, duration),
                indent=indent,
                file=file)
            for nested_fname, nested_stats in _sorted_items(stats["nested"]):
                self._save_stats(nested_fname, nested_stats,
                                 indent=(indent + self.indent_step), file=file)
        else:
            assert isinstance(stats, str)
            _print_indent("{}: {}".format(fname, stats),
                          indent=indent, file=file)


def _print_indent(line, indent, file):
    print("{:<{indent}}{}".format("", line, indent=indent), file=file)

def _sorted_items(d):
    return sorted(d.items(), key=lambda pair: pair[0])
