# Coding Standards and Rules

## 1. General Principles

- **DRY (Don't Repeat Yourself):** Extract repeated logic into helper functions.
- **KISS (Keep It Simple):** Prefer the most readable solution over the clever one.
- **Multi-module architecture:** Code is organized across multiple files in `src/` (`main.rs`, `audio.rs`, `clipboard.rs`, `config.rs`, `hotkeys.rs`, `logging.rs`, `state.rs`, `tray.rs`, `tts.rs`). Create new modules when logic warrants separation.
- **No `unsafe` code:** The tray module now uses `tray-icon`/`muda` instead of raw Win32 API. Avoid introducing `unsafe` blocks.

## 2. Rust & Typing

- **Edition:** Rust 2021 (`edition = "2021"` in `Cargo.toml`).
- **MSRV:** 1.70+ (declared via `rust-version`).
- **Toolchain:** Stable channel, pinned in `rust-toolchain.toml`.
- **No `unwrap()` in production paths:** Use `map_err` + `?` or `if let` to handle errors gracefully. Failures are logged via the `log()` function.
- **Atomic types for global state:** Use `AtomicU8`, `AtomicU32`, `AtomicBool` for thread-safe state; avoid `Mutex` where atomics suffice.

## 3. Naming Conventions

- **Functions / Variables:** `snake_case` in **Spanish** (e.g., `manejar_f8`, `leer_portapapeles`, `generar_audio_piper`).
- **Constants:** `UPPER_SNAKE_CASE` in **Spanish** (e.g., `RUTA_PIPER`, `VELOCIDAD_INICIAL`, `ESTADO_IDLE`).
- **Enum variants:** `PascalCase` in **Spanish** (e.g., `ComandoAudio::Reproducir`, `ComandoAudio::TogglePausa`).
- **Module names:** `snake_case` in **English** when generic (e.g., `tray`, `audio`, `config`).
- **Doc comments:** `///` in **Spanish** for all public and significant private functions.

## 4. Code Organization

The project uses a multi-module architecture under `src/`:

| Module | Responsibility |
|--------|---------------|
| `main.rs` | Entry point: spawns threads, launches tray event loop |
| `config.rs` | Compile-time constants (paths, speeds, version) |
| `state.rs` | Global atomics, enums (`ComandoAudio`), speed adjustment |
| `logging.rs` | Timestamped file logging (`log()` function) |
| `audio.rs` | Audio thread, `rodio::Sink` playback |
| `clipboard.rs` | Ctrl+C simulation + clipboard read |
| `tts.rs` | Piper TTS subprocess invocation |
| `hotkeys.rs` | Global hotkey registration and handlers |
| `tray.rs` | System tray icon + context menu (`tray-icon`/`muda`/`winit`) |

## 5. Comments & Documentation

- All comments and doc strings are written in **Spanish**.
- Use `///` doc comments for functions explaining what they do.
- Use `//` inline comments for non-obvious implementation details.
- Section headers within modules use box-drawing separators:
  ```rust
  // ═══════════════════════════════════════════════════════════════
  // SECTION NAME
  // ═══════════════════════════════════════════════════════════════
  ```

## 6. Formatting & Linting

- **Formatter:** `cargo fmt` (default rustfmt settings).
- **Linter:** `cargo clippy` (warnings treated as errors in CI on best-effort basis: `continue-on-error: true`).
- **Pre-commit hook:** Runs `cargo fmt` and `cargo clippy` automatically via `.githooks/`.
- **Indentation:** 4 spaces (Rust default).

## 7. Error Handling

- Functions that can fail return `Result<T, String>`.
- Errors are logged with the `log()` function before propagating.
- The `log()` function writes timestamped entries to `RUTA_LOG`.
- Never silently swallow errors — at minimum, log them.

## 8. Platform Considerations

- The tray module uses cross-platform crates (`tray-icon`, `muda`, `winit`), but the primary target is Windows.
- The `#![windows_subsystem = "windows"]` attribute hides the console window.
- When spawning Piper, use `CREATE_NO_WINDOW` creation flag to hide its console.

## 9. Build & Release

- **Cross-compilation target:** `x86_64-pc-windows-gnu` (from WSL/Linux).
- **Native target:** `x86_64-pc-windows-msvc` (from Windows / CI).
- **Release profile:** `opt-level = "z"`, LTO, single codegen unit, panic=abort, stripped — optimized for minimum binary size.
- **Release process:** Tag-triggered GitHub Actions workflow (`release.yml`).
