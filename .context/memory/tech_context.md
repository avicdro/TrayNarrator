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
| Platform API | windows-sys | 0.59 |
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
| `windows-sys` 0.59 | Low-level Win32 API bindings for shell notification icons, window management |

### Development

| Tool | Purpose |
|------|---------|
| `cargo fmt` | Code formatter (default rustfmt config) |
| `cargo clippy` | Lint checker |
| `mingw-w64` | Cross-compilation toolchain for Windows from Linux/WSL |
| `cargo-dist` | Release packaging configuration (in Cargo.toml metadata) |

## Architecture Decisions

### Single-file binary

**Context:** The project is small (~636 lines) and unlikely to exceed 1–2 KLOC soon.
**Decision:** Keep everything in `src/main.rs` with clear section separators.
**Consequences:** Fast to navigate; will need refactoring into modules if the feature set grows significantly.

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

## Integration Points

- **Piper TTS:** Invoked via `std::process::Command`, text piped to stdin, output WAV written to disk.
- **Windows Shell API:** `Shell_NotifyIconW` for tray icon, `TrackPopupMenu` for context menu.
- **System clipboard:** Read-only access via `arboard::Clipboard`.
- **GitHub Actions:** CI builds on Ubuntu (cross-compile), releases build on `windows-latest`.
