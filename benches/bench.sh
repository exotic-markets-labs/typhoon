#!/bin/bash

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Navigate to the programs directory
PROGRAMS_DIR="$SCRIPT_DIR/program-bench/benches/programs"
cd "$PROGRAMS_DIR"

# Build each program with cargo build-sbf
for dir in anchor pinocchio star-frame typhoon; do
    if [ -d "$dir" ]; then
        echo "Building $dir..."
        cd "$dir"
        cargo build-sbf --tools-version 1.51
        cd "$PROGRAMS_DIR"
    fi
done

# Navigate back to the bench directory and run cargo bench
BENCH_DIR="$SCRIPT_DIR/program-bench/benches"
cd "$BENCH_DIR"
cargo bench

