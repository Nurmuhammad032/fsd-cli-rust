#!/usr/bin/env bash

echo "=== FSD CLI Benchmark ==="
echo ""

# Cleanup
cleanup() {
    rm -rf bench-official bench-rust
    mkdir -p bench-official bench-rust
}

# Test 1: Simple slice creation
echo "Test 1: Create single page"
cleanup
hyperfine \
    --warmup 3 \
    --runs 20 \
    'cd bench-official && fsd pages home' \
    'cd bench-rust && rfsd add p home'

echo ""
echo "Test 2: Create page with segments"
cleanup
hyperfine \
    --warmup 3 \
    --runs 20 \
    'cd bench-official && fsd pages home -s ui,model,api' \
    'cd bench-rust && rfsd add p home -s ui,model,api'

echo ""
echo "Test 3: Create multiple pages"
cleanup
hyperfine \
    --warmup 3 \
    --runs 20 \
    'cd bench-official && fsd pages home about contact profile' \
    'cd bench-rust && rfsd add p home about contact profile'

echo ""
echo "Test 4: Create feature with all segments"
cleanup
hyperfine \
    --warmup 3 \
    --runs 20 \
    'cd bench-official && fsd features auth -s default' \
    'cd bench-rust && rfsd add f auth -s default'

echo ""
echo "Test 5: Initialize full structure"
cleanup
hyperfine \
    --warmup 3 \
    --runs 10 \
    'cd bench-official && fsd init' \
    'cd bench-rust && rfsd init --style full'

cleanup
echo ""
echo "=== Benchmark Complete ==="