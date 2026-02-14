# Changelog

Todos los cambios notables en este proyecto serán documentados en este archivo.

El formato está basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.0.0/),
y este proyecto adhiere a [Semantic Versioning](https://semver.org/lang/es/).

## [0.3.0] - 2026-02-14

### Añadido
- Arquitectura modular: `main.rs` dividido en 9 módulos (`audio`, `clipboard`, `config`, `hotkeys`, `logging`, `state`, `tray`, `tts`)
- System tray cross-platform con `tray-icon` + `muda` + `winit` (reemplaza `windows-sys`)
- Menú contextual del tray: submenú de velocidad, versión, salir
- Icono del tray embebido en el binario (`include_bytes!`)
- Presets de velocidad `xN`: `x0.5`, `x0.75`, `x1`, `x1.25`, `x1.5`, `x2`, `x3`
- Contexto AI para agentes (`.context/`, `AGENTS.md`)

### Cambiado
- Velocidad por defecto: `x1` (antes `x1.25`)
- `Ctrl+[` / `Ctrl+]` navegan entre presets contiguos en vez de incrementar/decrementar arbitrariamente

### Corregido
- El submenú de velocidad del tray se sincroniza cuando la velocidad cambia desde hotkeys

## [0.2.0] - 2026-02-07

### Añadido
- **CI/CD con GitHub Actions** - Releases automáticas para Windows
  - Workflow que compila y empaqueta automáticamente al crear un tag
  - Sistema de assets separados (`assets-v1`) para archivos pesados (Piper TTS, modelos)
  - Soporte para ejecución manual del workflow
- Configuración de `cargo-dist` en `Cargo.toml`
- Scripts de desarrollo para automatizar tareas comunes
  - `build.sh` - Compilar para Windows desde WSL
  - `deploy-win.sh` - Desplegar programa completo a `C:\TrayNarrator`
  - `build-and-deploy-wsl-win.sh` - Build + deploy en un solo paso
  - `lint.sh` - Formateo y linting con cargo fmt/clippy
  - `release.sh` - Crear zip para releases
  - `setup.sh` - Configurar entorno de desarrollo
  - Scripts `.bat` para Windows nativo en `scripts/win/`
- Git hooks para pre-commit (cargo fmt + clippy automático)
- Documentación de scripts en README

### Cambiado
- README actualizado con información de CI/CD y releases automáticas
- Estructura de proyecto mejorada

## [0.1.0] - 2026-02-07

### Añadido
- Lectura de texto seleccionado con F8
- Pausa/Reanudación con F9
- Control de velocidad con Ctrl+[ (rápido) y Ctrl+] (lento)
- Icono en la bandeja del sistema con menú contextual
- Opción "Salir" desde el menú del tray
- Visualización de velocidad actual en el menú
- Logging a archivo para debugging
- Integración con Piper TTS como motor de síntesis
- Soporte para modelos de voz en español
- Compilación cruzada desde WSL2 a Windows

### Características técnicas
- Binario ultraligero (~950KB)
- Arquitectura multi-hilo para audio no bloqueante
- Comunicación entre hilos con canales mpsc
- Estado global thread-safe con AtomicU8/AtomicU32
- System tray usando `windows-sys` (Win32 API)
