---
name: managing-memory
description: Maintain the Memory Bank files in .context/memory/. Use when updating project state, tracking progress, or persisting context between sessions.
---

# Managing Memory Bank

Keep the Memory Bank files updated to maintain context persistence across AI sessions.

## When to use

- Starting a new development session (read memory files first)
- Completing a significant task (update progress)
- Changing project focus (update active context)
- Major architectural decisions (update tech context)

## Memory Bank Structure

```
.context/memory/
├── project_brief.md    # Vision and goals (rarely changes)
├── tech_context.md     # Stack and patterns (updates with tech changes)
├── active_context.md   # Current focus (updates frequently)
└── progress.md         # Milestones and changelog (updates per task)
```

## File Update Frequency

| File | Update Frequency | Trigger |
|------|------------------|---------|
| `project_brief.md` | Rarely | Major goal changes |
| `tech_context.md` | Occasionally | New tech, patterns |
| `active_context.md` | Every session | Focus changes |
| `progress.md` | Per task | Task completion |

## Update Guidelines

### active_context.md

Update at the START and END of each session:

```markdown
## Current Focus
[What you're working on NOW]

## Recent Changes
- [Most recent first]

## Next Steps
1. [ ] Immediate next action
```

### progress.md

Update when completing significant work:

```markdown
### [DATE]
- Added: [new features]
- Changed: [modifications]
- Fixed: [bug fixes]
```

### tech_context.md

Update when adding new dependencies or patterns:

```markdown
## Stack
| Technology | Version | Purpose |
|------------|---------|---------|
| [New tech] | [ver]   | [why]   |
```

## Best Practices

### ✅ Do

- Read ALL memory files at session start
- Update `active_context.md` before ending work
- Keep entries concise and scannable
- Use tables for structured data
- Include dates on progress entries

### ❌ Avoid

- Duplicating information across files
- Leaving outdated "Current Focus"
- Forgetting to update progress after completing tasks
- Writing paragraphs instead of bullet points

## Session Workflow

1. **Start**: Read all memory files for context
2. **Work**: Focus on current task
3. **Complete**: Update progress.md with changes
4. **End**: Update active_context.md with next steps
