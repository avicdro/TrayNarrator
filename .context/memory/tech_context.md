---
name: tech_context
description: Technical stack and dependencies - semi-static, updates with major changes
---

# Technical Context

## Stack

| Layer | Technology | Version |
|-------|------------|---------|
| Language | Rust | 2021 edition, MSRV 1.70+ |
| TTS Engine | Piper TTS (external process) | latest |
| Audio | rodio | 0.19 |
| System Tray | tray-icon + muda | 0.19 / 0.15 |
| Event Loop | winit | 0.30 |
| CI/CD | GitHub Actions | N/A |

## Dependencies

### Production

| Crate | Purpose |
|-------|---------|
| `inputbot` 0.6 | Register global keyboard hotkeys (F8, F9, Ctrl+[, Ctrl+]) |
| `arboard` 3.4 | Read text from the system clipboard |
| `enigo` 0.2 | Simulate Ctrl+C keypress to copy selected text |
| `rodio` 0.19 | Decode and play WAV audio through the default output device |
| `lazy_static` 1.5 | Initialize the global audio channel sender at runtime |
| `parking_lot` 0.12 | Efficient Mutex for the channel sender (faster than std) |
| `tray-icon` 0.19 | Cross-platform system tray icon |
| `muda` 0.15 | Cross-platform context menu for the tray |
| `image` 0.25 | PNG decoding for the embedded tray icon |
| `winit` 0.30 | Cross-platform event loop for the system tray |

### Development

| Tool | Purpose |
|------|---------|
| `cargo fmt` | Code formatter (default rustfmt config) |
| `cargo clippy` | Lint checker |
| `mingw-w64` | Cross-compilation toolchain for Windows from Linux/WSL |
| `cargo-dist` | Release packaging configuration (in Cargo.toml metadata) |

## Architecture Decisions

### Multi-module structure

**Context:** The project grew beyond a single file and was refactored into modules for clarity.
**Decision:** Organize code across 9 files in `src/`: `main.rs`, `audio.rs`, `clipboard.rs`, `config.rs`, `hotkeys.rs`, `logging.rs`, `state.rs`, `tray.rs`, `tts.rs`.
**Consequences:** Each module has a clear responsibility; easier to navigate and maintain.

### Cross-platform tray with tray-icon + muda

**Context:** The original Win32 API tray implementation (`windows-sys`) was Windows-only and required `unsafe` code.
**Decision:** Replace with `tray-icon` + `muda` + `winit` for cross-platform tray support. Icon is embedded via `include_bytes!`.
**Consequences:** Portable foundation, no `unsafe` needed, slightly larger dependency tree but cleaner code.

### Piper TTS as external subprocess

**Context:** Linking ONNX Runtime and Piper natively would bloat the binary and add cross-compilation complexity.
**Decision:** Invoke `piper.exe` as a child process, piping text via stdin and writing to a temp WAV file.
**Consequences:** Simple integration, but requires the Piper binary + model to be deployed alongside the app. Adds ~90 MB to the distribution package.

### Spanish naming conventions

**Context:** The developer's primary language is Spanish.
**Decision:** All function names, variables, constants, comments, and doc strings are in Spanish.
**Consequences:** Consistent with the developer's workflow. Contributors should follow the same convention or discuss changes first.

### Atomic types for state management

**Context:** Multiple threads need to read/write playback state and speed without deadlocks.
**Decision:** Use `AtomicU8`, `AtomicU32`, `AtomicBool` for simple state flags; `parking_lot::Mutex` only where needed (channel sender).
**Consequences:** Lock-free reads for state checks, minimal contention.

### Speed control via fixed presets (xN)

**Context:** Increment/decrement speed by arbitrary values caused less predictable UX between tray and hotkeys.
**Decision:** Model speed as fixed contiguous presets (`x0.5`, `x0.75`, `x1`, `x1.25`, `x1.5`, `x2`, `x3`) and move across adjacent presets with `Ctrl+[` / `Ctrl+]`.
**Consequences:** More predictable behavior, easier UI labeling, and stable mapping to Piper `length_scale`.

### Tray speed UI synchronization

**Context:** Speed could be changed by hotkeys while the tray submenu still displayed a stale selected value.
**Decision:** Synchronize tray submenu check state/title from global speed state on each event loop cycle.
**Consequences:** UI remains consistent regardless of whether speed is changed from tray menu or keyboard shortcuts.

## Integration Points

- **Piper TTS:** Invoked via `std::process::Command`, text piped to stdin, output WAV written to disk.
- **System Tray:** `tray-icon` + `muda` for the tray icon and context menu; `winit` event loop on the main thread.
- **System clipboard:** Read-only access via `arboard::Clipboard`.
- **GitHub Actions:** CI builds on Ubuntu (cross-compile), releases build on `windows-latest`.
