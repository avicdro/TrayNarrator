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
| Ctrl+[/] speed adjustment | ✅ Complete | AtomicU32 length_scale, range 50–150 |
| System tray icon + context menu | ✅ Complete | Native Win32 shell API |
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

### Phase 3: Configuration & Multi-language (Planned)

| Task | Status | Notes |
|------|--------|-------|
| Config file (`.toml`) for paths and settings | ⬜ Not Started | Replace compile-time constants |
| Multi-language support | ⬜ Not Started | Dynamic model selection |
| Toast notifications | ⬜ Not Started | Visual feedback on actions |
| Audio device selection | ⬜ Not Started | Choose output device |
| Windows installer | ⬜ Not Started | MSI or NSIS |

## Changelog

### 2026-02-07 (v0.2.0)

- Added: CI/CD with GitHub Actions, dev scripts, git hooks, cargo-dist config
- Changed: README updated with CI/CD and release documentation

### 2026-02-07 (v0.1.0)

- Added: Core TTS reading, pause/resume, speed control, system tray, logging

## Metrics

| Metric | Value | Target |
|--------|-------|--------|
| Binary size (release) | ~950 KB | < 1 MB |
| Source lines of code | ~636 | — |
| Dependencies (direct) | 7 | Keep minimal |

---

*This file tracks overall project progress and is updated by the AI agent.*
