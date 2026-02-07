#!/bin/sh
# Script de configuración del entorno de desarrollo

set -e

echo "Configurando entorno de desarrollo para TrayNarrator..."

# Configurar git hooks
echo "Configurando git hooks..."
git config core.hooksPath .githooks

# Verificar que Rust esté instalado
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust no está instalado. Instálalo desde https://rustup.rs"
    exit 1
fi

# Instalar componentes necesarios
echo "Instalando componentes de Rust..."
rustup component add rustfmt clippy

# Añadir target de Windows si no está
echo "Añadiendo target de Windows..."
rustup target add x86_64-pc-windows-gnu

echo ""
echo "Configuración completada!"
echo "Los hooks de pre-commit ejecutarán cargo fmt y clippy automáticamente."
