#!/usr/bin/env sh

echo "Warning: this script execution can take more than 30min." >&2

set -e

echo "Pre-build a binary..." >&2
cargo build --quiet --release

for bench in canon-map-single-key \
          ;
do
    echo "Generating ${bench} ..." >&2
    DESTDIR="../../benches/performance_metering/${bench}/"
    mkdir -p "${DESTDIR}"
    time cargo run --quiet --release -- --dest-dir "${DESTDIR}" "${bench}"
done
