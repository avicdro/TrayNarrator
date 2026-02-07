#!/bin/bash
# Compilar el .exe para Windows desde WSL/Linux

set -e

echo "Compilando TrayNarrator para Windows..."

cargo build --release --target x86_64-pc-windows-gnu

echo ""
echo "Compilación exitosa!"
echo "Binario: target/x86_64-pc-windows-gnu/release/tray_narrator.exe"
