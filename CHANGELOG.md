# Changelog

Todos los cambios notables en este proyecto serán documentados en este archivo.

El formato está basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.0.0/),
y este proyecto adhiere a [Semantic Versioning](https://semver.org/lang/es/).

## [0.2.0] - 2026-02-07

### Añadido
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
- README más profesional, sin emojis excesivos

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
- System tray nativo usando windows-sys
