#!/bin/bash

echo "=== FSD CLI Benchmark ==="
echo ""

# Cleanup
cleanup() {
    rm -rf bench-official bench-rust
    mkdir -p bench-official/src bench-rust/src
}

# Check if both CLIs are available
if ! command -v fsd &> /dev/null; then
    echo "Error: Official 'fsd' CLI not found. Install with: npm install -g @feature-sliced/cli"
    exit 1
fi

if ! command -v rfsd &> /dev/null; then
    echo "Error: 'rfsd' CLI not found. Install with: cargo install --path ."
    exit 1
fi

echo "Official FSD CLI version:"
fsd --version 2>/dev/null || echo "Version info not available"
echo ""

# Test 1: Simple slice creation
echo "Test 1: Create single page"
cleanup
hyperfine \
    --prepare 'rm -rf bench-official/src/pages bench-rust/src/pages' \
    --warmup 3 \
    --runs 20 \
    'cd bench-official && fsd pages home' \
    'cd bench-rust && rfsd add p home'

echo ""
echo "Test 2: Create page with segments"
cleanup
hyperfine \
    --prepare 'rm -rf bench-official/src/pages bench-rust/src/pages' \
    --warmup 3 \
    --runs 20 \
    'cd bench-official && fsd pages home -s ui,model,api' \
    'cd bench-rust && rfsd add p home -s ui,model,api'

echo ""
echo "Test 3: Create multiple pages"
cleanup
hyperfine \
    --prepare 'rm -rf bench-official/src/pages bench-rust/src/pages' \
    --warmup 3 \
    --runs 20 \
    'cd bench-official && fsd pages home about contact profile' \
    'cd bench-rust && rfsd add p home about contact profile'

echo ""
echo "Test 4: Create feature with all segments"
cleanup
hyperfine \
    --prepare 'rm -rf bench-official/src/features bench-rust/src/features' \
    --warmup 3 \
    --runs 20 \
    'cd bench-official && fsd features auth -s ui,model,api,lib,config' \
    'cd bench-rust && rfsd add f auth -s ui,model,api,lib,config'

echo ""
echo "Test 5: Just startup time (--help)"
hyperfine \
    --warmup 10 \
    --runs 50 \
    'fsd --help' \
    'rfsd --help'

cleanup
echo ""
echo "=== Benchmark Complete ==="