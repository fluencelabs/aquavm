#!/usr/bin/env sh

echo "Warning: this script execution can take more than 30min." >&2

set -e

echo "Pre-build a binary..." >&2
cargo build --quiet --release

for bench in multiple-cids10 multiple-cids50 \
             multiple-peers14 multiple-peers25 \
             multiple-sigs50 multiple-sigs200 \
             dashboard network-explore; do
    echo "Generating ${bench} ..." >&2
    if [ "${bench}" == multiple-peers25 ]; then
        echo "WARNING: this bench data generation can take more than 10 minutes..." >&2
    fi
    DESTDIR="../../benches/performance_metering/${bench}/"
    mkdir -p "${DESTDIR}"
    time cargo run --quiet --release -- --dest-dir "${DESTDIR}" "${bench}"
done
