#!/bin/bash
# Lint y formato del código

set -e

echo "=== Formateando código ==="
cargo fmt

echo ""
echo "=== Ejecutando Clippy ==="
cargo clippy --target x86_64-pc-windows-gnu -- -D warnings

echo ""
echo "Todo OK!"
