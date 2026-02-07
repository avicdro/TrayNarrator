#!/bin/bash
# Compilar y desplegar a Windows en un solo paso
# Ejecutar desde WSL

set -e

echo "=== Build + Deploy a Windows ==="
echo ""

./scripts/build.sh
./scripts/deploy-win.sh
