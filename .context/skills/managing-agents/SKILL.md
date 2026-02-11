---
name: managing-agents
description: Configure and auto-sync AGENTS.md with .context/ files. Use when setting up agent context, modifying agent instructions, or after changes to rules/skills.
---

# Managing Agents Context

Configure the AGENTS.md file that orients AI agents to your project.

## When to use

- Setting up AI agent context for a new project
- After adding new rules to `.context/rules/`
- After adding new skills to `.context/skills/`
- After updating architecture or project state
- Changing how agents should behave in your project

## AGENTS.md Purpose

The AGENTS.md file is the **universal entry point** for ALL AI agents:
- Cursor, Copilot, Claude, Gemini, Windsurf all read this file
- It provides a single source of truth for project context
- It indexes all content in `.context/`

## Auto-Sync Trigger

**After modifying any file in `.context/`**, check if AGENTS.md needs updating:

1. Scan `.context/rules/` for all `.md` files
2. Scan `.context/skills/` for all `SKILL.md` files
3. Update the Knowledge Index section if files changed
4. Keep AGENTS.md under 50 lines

## Standard Template

```markdown
# AI Agent Context

You are a senior developer joining this project.
Your goal is to help build, refactor, and maintain code following our architecture.

## Knowledge Index

| Area | Path | Description |
|------|------|-------------|
| Architecture | `.context/architecture.md` | Project structure |
| Rules | `.context/rules/` | Coding standards |
| Skills | `.context/skills/` | Reusable instructions |
| State | `.context/project_state.md` | Current work |
| Memory | `.context/memory/` | Session persistence |

## Active Rules

- [coding-standards](.context/rules/coding-standards.md)

## Available Skills

- [generating-skills](.context/skills/generating-skills/SKILL.md)
- [managing-agents](.context/skills/managing-agents/SKILL.md)
...

## Principles

- Follow all rules in `.context/rules/`
- Check project state before starting work
- Update memory bank at session end
```

## Best Practices

### ✅ Do

- Keep AGENTS.md concise (under 50 lines)
- Point to detailed files instead of duplicating content
- Update when `.context/` structure changes
- Include table of contents to all context areas

### ❌ Avoid

- Putting detailed documentation in AGENTS.md
- Outdated file paths
- Generic instructions without project specifics
- Forgetting to list new rules or skills

## Placement

AGENTS.md should be at the project root:

```
project/
├── AGENTS.md           # AI agent entry point (universal hub)
├── .context/           # Detailed context
│   ├── architecture.md
│   ├── project_state.md
│   ├── rules/
│   ├── skills/
│   └── memory/
└── src/
```
