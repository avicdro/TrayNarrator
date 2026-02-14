# Project State (Memory)

> This file is updated manually so the AI knows where it left off in the last session.
> **Last updated:** 2026-02-14

## Current Context

The project is in **v0.3.0 release preparation** — modular architecture is complete, tray/hotkeys speed flow is stabilized with contiguous `xN` presets, and manual Windows validation is finished.

## Recently Completed

- [x] Core TTS reading with F8 hotkey (select text → Piper TTS → audio playback)
- [x] Pause/Resume with F9
- [x] Speed control with Ctrl+[ and Ctrl+] using contiguous `xN` presets
- [x] Multi-module refactoring (9 source files)
- [x] Enhanced system tray icon with context menu (velocity controls, version, exit)
- [x] Cross-platform tray migration (windows-sys → tray-icon/muda/winit)
- [x] Embedded tray icon via `include_bytes!`
- [x] Tray speed submenu synchronized when speed changes via hotkeys
- [x] CI pipeline (GitHub Actions: fmt check, clippy, cross-compile build)
- [x] Automated release workflow (tag-triggered, downloads Piper from `assets-v1`)
- [x] Development scripts (`setup.sh`, `build.sh`, `deploy-win.sh`, `lint.sh`, `release.sh`)
- [x] Git hooks for pre-commit (cargo fmt + clippy)
- [x] AI context setup (`.context/` directory via agent-ctx)
- [x] Comprehensive documentation review and update

## In Progress (Current Sprint)

- [x] Manual testing on Windows (tray icon, context menu, hotkeys)
- [x] Prepare next version changelog notes
- [ ] Create tag and publish release `v0.3.0`

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
| Clippy warnings in CI have `continue-on-error: true` | Low | Should eventually be strict |
| Two CI workflows exist (`ci.yml` and `release.yml`) with overlapping build logic | Low | Could potentially be unified |
| Spanish-only voice model hardcoded | Medium | No way to switch language without recompiling |
| Piper assets not versioned with code | Low | Stored in a separate `assets-v1` release |

## Important Decisions

- **Single binary, no installer:** Distribute as a simple ZIP to `C:\TrayNarrator\`. Keeps it ultra-lightweight and portable.
- **Piper TTS as external process:** Piper is invoked as a subprocess (not linked) to keep the Rust binary small and avoid ONNX Runtime linking complexity.
- **Spanish naming in code:** All function names, variables, constants, and comments are in Spanish to match the developer's preference.
- **Separate heavy assets release:** Piper binaries (~90 MB) are stored in a GitHub release tagged `assets-v1` to avoid bloating the repository.
- **Cross-platform tray:** Replaced Windows-only `windows-sys` with `tray-icon`/`muda`/`winit` for portable system tray support.

## Notes for Next Session

> Write here anything important the AI should know before starting to work:

- The system tray now uses `tray-icon` + `muda` + `winit` — no `unsafe` code or `windows-sys`.
- The tray icon is embedded via `include_bytes!("../assets/traynarrator-icon.png")`.
- Speed uses fixed presets (`x0.5`, `x0.75`, `x1`, `x1.25`, `x1.5`, `x2`, `x3`) with default `x1`.
- Ctrl+[ advances to next faster preset; Ctrl+] moves to next slower preset.
- Tray speed submenu now stays in sync when speed changes via hotkeys.
- Changelog has been consolidated for `v0.3.0` release notes.
- Cross-compilation from WSL2 uses `x86_64-pc-windows-gnu`, while CI release uses `x86_64-pc-windows-msvc`.
- The `piper/` directory is gitignored — it must be present locally for testing, or downloaded at release time.

## Current Restrictions

- Do not modify the `piper/` directory contents — those are external binaries.
- Do not remove Spanish naming conventions without explicit approval.
- Keep the binary size as small as possible (currently ~950 KB).
