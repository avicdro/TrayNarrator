@echo off
REM Lint y formato en Windows nativo

echo === Formateando codigo ===
cargo fmt

echo.
echo === Ejecutando Clippy ===
cargo clippy -- -D warnings

if %ERRORLEVEL% NEQ 0 (
    echo Clippy encontro errores
    exit /b 1
)

echo.
echo Todo OK!
