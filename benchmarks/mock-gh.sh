#!/bin/bash
# Mock gh CLI that returns a fixed JSON response.
# Usage: PATH="$(pwd)/benchmarks:$PATH" prow
#
# This script pretends to be `gh` and returns sample PR data
# so benchmarks don't depend on network or authentication.

cat "$(dirname "$0")/sample-response.json"
