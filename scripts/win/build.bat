@echo off
REM Compilar TrayNarrator en Windows nativo

echo Compilando TrayNarrator...
cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo Error en la compilacion
    exit /b 1
)

echo.
echo Compilacion exitosa!
echo Binario: target\release\tray_narrator.exe
