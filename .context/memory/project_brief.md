---
name: project_brief
description: Project vision and goals - static, rarely changes
---

# Project Brief

## Vision

TrayNarrator aims to be the simplest, lightest text-to-speech reader for Windows. Select any text in any application, press a hotkey, and hear it read aloud — with zero UI, zero configuration, and a sub-1 MB binary. It leverages Piper TTS for high-quality, offline neural speech synthesis.

## Goals

- [x] Provide one-key text-to-speech for any selected text on Windows
- [x] Run entirely in the background with only a system tray icon
- [x] Keep the binary ultra-lightweight (< 1 MB)
- [ ] Support multiple voice models and languages
- [ ] Allow runtime configuration without recompilation
- [ ] Provide a polished installer experience

## Success Criteria

1. Users can select text and hear it read within 1–2 seconds of pressing F8.
2. The application uses minimal resources and is invisible when not in use.
3. Speed is adjustable in real time.
4. The binary stays under 1 MB compiled.

## Constraints

- **Platform:** Windows 10/11 (64-bit) only for now.
- **Dependency:** Requires Piper TTS binaries (~90 MB) deployed alongside the binary.
- **Languages:** Currently Spanish only (hardcoded model path).
- **Configuration:** All settings are compile-time constants — no config file yet.
- **Solo developer:** Single maintainer, Spanish-speaking, developing from WSL2.

## Stakeholders

| Role | Name | Responsibility |
|------|------|----------------|
| Owner & Developer | avicdro | Architecture, implementation, releases |
