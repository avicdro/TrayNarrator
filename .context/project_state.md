# Project State (Memory)

> This file is updated manually so the AI knows where it left off in the last session.
> **Last updated:** 2026-02-10

## Current Context

The project is at **v0.2.0** — a functional MVP with automated CI/CD. Core TTS functionality works. The focus is now on stability improvements, configuration externalization, and potential multi-language support.

## Recently Completed

- [x] Core TTS reading with F8 hotkey (select text → Piper TTS → audio playback)
- [x] Pause/Resume with F9
- [x] Speed control with Ctrl+[ and Ctrl+]
- [x] System tray icon with context menu (velocity display + exit)
- [x] CI pipeline (GitHub Actions: fmt check, clippy, cross-compile build)
- [x] Automated release workflow (tag-triggered, downloads Piper from `assets-v1`)
- [x] Development scripts (`setup.sh`, `build.sh`, `deploy-win.sh`, `lint.sh`, `release.sh`)
- [x] Git hooks for pre-commit (cargo fmt + clippy)
- [x] AI context setup (`.context/` directory via agent-ctx)

## In Progress (Current Sprint)

- [ ] Populate AI context files with actual project data (this bootstrap)
- [ ] Internationalize documentation (README currently in Spanish)

## Coming Up Next

- [ ] External configuration via `.toml` file (replace compile-time constants)
- [ ] Multi-language support (dynamic model switching)
- [ ] Windows toast notifications for status feedback
- [ ] Read history / log of recently read texts
- [ ] Audio confirmation sound on F8 press
- [ ] Audio output device selection
- [ ] Windows installer (MSI or NSIS)
- [ ] Cross-platform support (Linux / macOS)

## Known Bugs / Technical Debt

| Bug/Issue | Priority | Notes |
|-----------|----------|-------|
| All config is compile-time constants | Medium | Must recompile to change paths or speed defaults |
| Single-file architecture (`main.rs` ~636 lines) | Low | Fine for now, but will need module extraction as features grow |
| Clippy warnings in CI have `continue-on-error: true` | Low | Should eventually be strict |
| Two CI workflows exist (`ci.yml` and `release.yml`) with overlapping build logic | Low | Could potentially be unified |
| Spanish-only voice model hardcoded | Medium | No way to switch language without recompiling |
| Piper assets not versioned with code | Low | Stored in a separate `assets-v1` release |

## Important Decisions

- **Single binary, no installer:** Distribute as a simple ZIP to `C:\TrayNarrator\`. Keeps it ultra-lightweight and portable.
- **Piper TTS as external process:** Piper is invoked as a subprocess (not linked) to keep the Rust binary small and avoid ONNX Runtime linking complexity.
- **Spanish naming in code:** All function names, variables, constants, and comments are in Spanish to match the developer's preference.
- **Separate heavy assets release:** Piper binaries (~90 MB) are stored in a GitHub release tagged `assets-v1` to avoid bloating the repository.

## Notes for Next Session

> Write here anything important the AI should know before starting to work:

- The project targets **Windows only** for now, but the code has `#[cfg(not(windows))]` fallback stubs.
- Cross-compilation from WSL2 uses `x86_64-pc-windows-gnu`, while CI release uses `x86_64-pc-windows-msvc`.
- The `piper/` directory is gitignored — it must be present locally for testing, or downloaded at release time.

## Current Restrictions

- Do not modify the `piper/` directory contents — those are external binaries.
- Do not remove Spanish naming conventions without explicit approval.
- Keep the binary size as small as possible (currently ~950 KB).
