#!/bin/bash
# Copiar el programa completo a Windows para probar
# Ejecutar desde WSL

set -e

WINDOWS_PATH="/mnt/c/TrayNarrator"
EXE_PATH="target/x86_64-pc-windows-gnu/release/tray_narrator.exe"

# Verificar que existe el binario
if [ ! -f "$EXE_PATH" ]; then
    echo "Error: No se encontró el binario. Ejecuta primero: ./scripts/build.sh"
    exit 1
fi

# Crear carpetas en Windows
mkdir -p "$WINDOWS_PATH/piper"

# Copiar el ejecutable
echo "Copiando tray_narrator.exe..."
cp "$EXE_PATH" "$WINDOWS_PATH/"

# Copiar carpeta piper si existe
if [ -d "piper" ]; then
    echo "Copiando carpeta piper..."
    cp -r piper/* "$WINDOWS_PATH/piper/"
else
    echo "Advertencia: Carpeta piper no encontrada en el proyecto"
    echo "Asegúrate de tener piper/ con los archivos necesarios:"
    echo "  - piper.exe"
    echo "  - espeak-ng.dll, onnxruntime.dll, piper_phonemize.dll"
    echo "  - es_ES-sharvard-medium.onnx + .json"
    echo "  - espeak-ng-data/"
fi

echo ""
echo "Deploy completado!"
echo "Ejecuta en Windows: C:\\TrayNarrator\\tray_narrator.exe"
