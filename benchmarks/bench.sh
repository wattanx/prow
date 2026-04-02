#!/bin/bash
# Benchmark: compare TypeScript (Bun) vs Rust binary.
#
# Prerequisites:
#   - hyperfine installed (brew install hyperfine)
#   - Both binaries built:
#     - TS: pnpm build (produces dist/prow)
#     - Rust: cd rust && cargo build --release (produces rust/target/release/prow)
#
# Usage: ./benchmarks/bench.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

TS_BIN="$PROJECT_DIR/dist/prow"
RS_BIN="$PROJECT_DIR/rust/target/release/prow"

echo "=== Binary Size ==="
echo -n "TypeScript (Bun): "
ls -lh "$TS_BIN" 2>/dev/null | awk '{print $5}' || echo "not found"
echo -n "Rust:             "
ls -lh "$RS_BIN" 2>/dev/null | awk '{print $5}' || echo "not found"
echo ""

echo "=== Startup Time (--version) ==="
hyperfine \
  --warmup 5 \
  --min-runs 50 \
  --export-json "$SCRIPT_DIR/results-startup.json" \
  "$TS_BIN --version" \
  "$RS_BIN --version"
echo ""

echo "=== Memory Usage (--version) ==="
echo "TypeScript (Bun):"
/usr/bin/time -l "$TS_BIN" --version 2>&1 | grep "maximum resident"
echo "Rust:"
/usr/bin/time -l "$RS_BIN" --version 2>&1 | grep "maximum resident"
