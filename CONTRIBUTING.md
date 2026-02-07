# Contribuir a TrayNarrator

¡Gracias por tu interés en contribuir! 🎉

## Cómo Contribuir

### Reportar Bugs

1. Verifica que el bug no haya sido reportado antes en [Issues](../../issues)
2. Abre un nuevo issue con:
   - Descripción clara del problema
   - Pasos para reproducirlo
   - Comportamiento esperado vs actual
   - Contenido de `log.txt` si es relevante
   - Versión de Windows

### Sugerir Mejoras

1. Abre un issue describiendo:
   - La funcionalidad que te gustaría
   - Por qué sería útil
   - Posibles implementaciones

### Pull Requests

1. **Fork** el repositorio
2. Crea una **rama** para tu feature (`git checkout -b feature/mi-feature`)
3. Haz **commit** de tus cambios (`git commit -am 'Añade mi feature'`)
4. **Push** a la rama (`git push origin feature/mi-feature`)
5. Abre un **Pull Request**

## Configuración del Entorno de Desarrollo

### Requisitos

- Rust 1.70+ (recomendado: última estable)
- Para cross-compilation a Windows:
  - WSL2 o Linux
  - mingw-w64 (`sudo apt install mingw-w64`)
  - Target Windows (`rustup target add x86_64-pc-windows-gnu`)

### Compilar

```bash
# Debug (rápido, más grande)
cargo build

# Release (optimizado, pequeño)
cargo build --release --target x86_64-pc-windows-gnu
```

### Probar

```bash
# Ejecutar en Windows
cargo run --release --target x86_64-pc-windows-gnu
```

## Guía de Estilo

- Sigue las convenciones de Rust (usa `cargo fmt`)
- Ejecuta `cargo clippy` antes de hacer PR
- Documenta funciones públicas con `///`
- Mantén los comentarios en español (igual que el resto del código)
- Usa nombres descriptivos en español para variables y funciones

## Estructura del Proyecto

```
src/
└── main.rs          # Todo el código (modular interno)
    ├── Configuración (constantes)
    ├── Estado global (atomics, canales)
    ├── Logging
    ├── System Tray (módulo tray)
    ├── Funciones auxiliares (copiar, portapapeles, Piper)
    ├── Hilo de audio
    ├── Manejadores de teclas
    └── main()
```

## Ideas para Contribuir

- [ ] Soporte para más idiomas (cambiar modelo dinámicamente)
- [ ] Configuración desde archivo .toml
- [ ] Notificaciones toast en Windows
- [ ] Historial de textos leídos
- [ ] Sonido de confirmación al presionar F8
- [ ] Opción de seleccionar dispositivo de audio
- [ ] Instalador MSI/NSIS
- [ ] Soporte para Linux/macOS

## CI/CD y Releases

Este proyecto usa GitHub Actions para releases automáticas:

1. **Assets pesados** se almacenan en una release separada (`assets-v1`)
2. Al crear un tag `v*`, el workflow automáticamente:
   - Descarga los assets de `assets-v1`
   - Compila el binario
   - Crea y publica la release

### Para crear una nueva release:

```bash
# 1. Actualiza versión en Cargo.toml
# 2. Actualiza CHANGELOG.md
# 3. Commit y tag
git add .
git commit -m "release: vX.Y.Z"
git tag vX.Y.Z
git push && git push --tags
```

## Código de Conducta

- Sé respetuoso y constructivo
- Acepta críticas constructivas
- Enfócate en lo mejor para el proyecto y la comunidad

---

¿Preguntas? Abre un issue o contacta al mantenedor.
