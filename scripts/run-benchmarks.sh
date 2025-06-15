#!/bin/bash
set -e

echo "🏗️  Building benchmark programs..."
cd benches/program-bench/benches/programs
cargo build-sbf

echo "🚀 Running benchmarks..."
cd ../..
cargo bench --bench bench

echo "✅ Benchmarks complete! Results written to benches/BENCHMARK.md" 