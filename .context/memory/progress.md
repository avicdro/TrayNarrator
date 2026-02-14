---
name: progress
description: Development progress tracking - dynamic, updates with each milestone
---

# Progress

## Milestones

### Phase 1: Core MVP (v0.1.0 — Complete)

| Task | Status | Notes |
|------|--------|-------|
| F8 hotkey → select text → read aloud | ✅ Complete | Uses Ctrl+C simulation + clipboard + Piper |
| F9 pause/resume playback | ✅ Complete | Toggle via `ComandoAudio::TogglePausa` |
| Ctrl+[/] speed adjustment | ✅ Complete | Presets `x0.5..x3` con pasos contiguos |
| System tray icon + context menu | ✅ Complete | Original implementation with windows-sys |
| File-based logging | ✅ Complete | Timestamped log to `C:\TrayNarrator\log.txt` |
| Multi-threaded audio playback | ✅ Complete | mpsc channel + rodio Sink |
| Cross-compilation from WSL2 | ✅ Complete | `x86_64-pc-windows-gnu` target |

### Phase 2: DevOps & Tooling (v0.2.0 — Complete)

| Task | Status | Notes |
|------|--------|-------|
| GitHub Actions CI (fmt + clippy + build) | ✅ Complete | `ci.yml` on Ubuntu with cross-compile |
| Automated release workflow | ✅ Complete | `release.yml` on Windows, tag-triggered |
| Heavy assets management (`assets-v1`) | ✅ Complete | Piper binaries stored in separate GH release |
| Development scripts | ✅ Complete | `setup.sh`, `build.sh`, `deploy-win.sh`, `lint.sh`, `release.sh` |
| Git hooks (pre-commit) | ✅ Complete | cargo fmt + clippy via `.githooks/` |
| AI context setup (`agent-ctx`) | ✅ Complete | `.context/` directory populated |

### Phase 3: Architecture, Tray & Speed (v0.3.0 — Complete)

| Task | Status | Notes |
|------|--------|-------|
| Multi-module refactoring | ✅ Complete | 9 source files: main, audio, clipboard, config, hotkeys, logging, state, tray, tts |
| Cross-platform tray (tray-icon + muda + winit) | ✅ Complete | Replaced windows-sys with portable crates |
| Enhanced tray context menu | ✅ Complete | Speed controls, version display, exit |
| Tray/hotkeys speed sync | ✅ Complete | El menú refleja cambios de velocidad desde hotkeys |
| Embedded tray icon (include_bytes!) | ✅ Complete | No runtime file dependency |
| Speed presets xN | ✅ Complete | x0.5..x3 with contiguous stepping |
| Windows manual validation | ✅ Complete | Hotkeys + tray + speed sync validated |
| Documentation overhaul | ✅ Complete | All docs updated to reflect current architecture |

### Phase 4: Configuration & Multi-language (Planned)

| Task | Status | Notes |
|------|--------|-------|
| Config file (`.toml`) for paths and settings | ⬜ Not Started | Replace compile-time constants |
| Multi-language support | ⬜ Not Started | Dynamic model selection |
| Toast notifications | ⬜ Not Started | Visual feedback on actions |
| Audio device selection | ⬜ Not Started | Choose output device |
| Windows installer | ⬜ Not Started | MSI or NSIS |

## Changelog

### 2026-02-14

- Enhanced: System tray with cross-platform crates (tray-icon, muda, winit)
- Added: Tray context menu with speed controls and version info
- Changed: Replaced windows-sys with tray-icon/muda/winit
- Changed: Speed control standardized to contiguous `xN` presets with default `x1`
- Fixed: Tray speed submenu synchronization when speed changes from hotkeys
- Changed: Comprehensive documentation review and update
- Done: Manual Windows testing completed before release

### 2026-02-10

- Refactored: Single-file architecture into 9 modules
- Added: AI context files populated with project data

### 2026-02-07 (v0.2.0)

- Added: CI/CD with GitHub Actions, dev scripts, git hooks, cargo-dist config
- Changed: README updated with CI/CD and release documentation

### 2026-02-07 (v0.1.0)

- Added: Core TTS reading, pause/resume, speed control, system tray, logging

## Metrics

| Metric | Value | Target |
|--------|-------|--------|
| Binary size (release) | ~950 KB | < 1 MB |
| Source files | 9 | — |
| Dependencies (direct) | 10 | Keep minimal |

---

*This file tracks overall project progress and is updated by the AI agent.*
