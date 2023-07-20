# The `air_perofrmance_metering` utility

Execute an AquaVM special benchmarking suite and recort results with some meta information to `benches/PERFORMANCE.json` database.

This script is intended to be run from the project root.  It uses the `air` through `cargo`, without installation.

# Installation

Run in the project run:
``` sh
pip install tools/cli/performance_metering

```

# Usage
In the project root, run
``` sh
aquavm_performance_metering run
```

You may also pass the `--repeat N` option to do multiple runs with averaging.
