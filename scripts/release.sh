#!/bin/bash
# Script para crear un release de TrayNarrator
# Uso: ./scripts/release.sh v1.0.0

set -e

VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Uso: ./scripts/release.sh <version>"
    echo "Ejemplo: ./scripts/release.sh v1.0.0"
    exit 1
fi

echo "Creando release $VERSION..."

# Verificar que estamos en main y sin cambios pendientes
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "main" ]; then
    echo "Error: Debes estar en la rama main"
    exit 1
fi

if ! git diff --quiet; then
    echo "Error: Hay cambios sin commitear"
    exit 1
fi

# Compilar para Windows
echo "Compilando para Windows..."
cargo build --release --target x86_64-pc-windows-gnu

# Crear directorio temporal para el release
RELEASE_DIR="release/TrayNarrator-$VERSION"
rm -rf release
mkdir -p "$RELEASE_DIR/piper"

# Copiar binario
cp target/x86_64-pc-windows-gnu/release/tray_narrator.exe "$RELEASE_DIR/"

# Copiar archivos de piper (deben existir localmente)
PIPER_FILES=(
    "piper/piper.exe"
    "piper/espeak-ng.dll"
    "piper/onnxruntime.dll"
    "piper/piper_phonemize.dll"
    "piper/es_ES-sharvard-medium.onnx"
    "piper/es_ES-sharvard-medium.onnx.json"
)

for file in "${PIPER_FILES[@]}"; do
    if [ -f "$file" ]; then
        cp "$file" "$RELEASE_DIR/piper/"
    else
        echo "Advertencia: $file no encontrado"
    fi
done

# Copiar espeak-ng-data
if [ -d "piper/espeak-ng-data" ]; then
    cp -r piper/espeak-ng-data "$RELEASE_DIR/piper/"
fi

# Copiar documentación
cp README.md "$RELEASE_DIR/"
cp LICENSE "$RELEASE_DIR/"

# Crear zip
cd release
zip -r "TrayNarrator-$VERSION-windows-x64.zip" "TrayNarrator-$VERSION"
cd ..

echo ""
echo "Release creado: release/TrayNarrator-$VERSION-windows-x64.zip"
echo ""
echo "Próximos pasos:"
echo "1. git tag -a $VERSION -m \"Release $VERSION\""
echo "2. git push origin $VERSION"
echo "3. Crear release en GitHub y subir el zip"
