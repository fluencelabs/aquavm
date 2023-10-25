#!/usr/bin/env sh

echo "Warning: this script execution can take more than 30min." >&2

set -e

echo "Pre-build a binary..." >&2
cargo build --quiet --release

for bench in canon-map-key-by-lens \
            canon-map-key-element-by-lens \
            canon-map-multiple-keys \
            canon-map-single-key \
            canon-map-scalar-multiple-keys \
            canon-map-scalar-single-key \
            populate-map-multiple-keys \
            populate-map-single-key \
            multiple-cids10 \
            multiple-peers8 \
            multiple-sigs30 \
            dashboard network-explore;
do
    echo "Generating ${bench} ..." >&2
    DESTDIR="../../benches/performance_metering/${bench}/"
    mkdir -p "${DESTDIR}"
    time cargo run --quiet --release -- --dest-dir "${DESTDIR}" "${bench}"
done
