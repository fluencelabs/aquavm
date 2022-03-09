#!/usr/bin/env bash

set -euo pipefail

for dir in ./features/tetraplets/security_tetraplets/*; do
    # skip non-directory entries
    [ -d "$dir" ] || continue

    (cd "$dir"; marine build)
done
