---
name: tracking-project-state
description: Manage project state documentation for AI agent context. Use when updating current work status, priorities, or project phase.
---

# Tracking Project State

Maintain a living document of current project status for AI agents.

## When to use

- Starting or completing major tasks
- Changing project priorities
- Blocking issues arise
- Sprint or phase transitions

## Project State Structure

```markdown
# Project State

## Current Phase
[Development / Testing / Maintenance / etc.]

## Active Work

### In Progress
- [ ] Feature X - [brief status]
- [ ] Bug fix Y - [brief status]

### Blocked
- Issue Z - [reason] - [owner]

## Recent Completions
- ✅ Feature A (date)
- ✅ Bug fix B (date)

## Next Priorities
1. Priority task 1
2. Priority task 2

## Known Issues
- Issue description - [severity]
```

## Update Frequency

Update project_state.md when:
- Starting significant work
- Completing features or fixes
- Encountering blockers
- Changing priorities
- Beginning new sprint/phase

## Best Practices

### ✅ Do

- Keep it current (update same day)
- Be specific about status
- Include blockers and owners
- Date recent completions
- Limit to top priorities

### ❌ Avoid

- Stale information (>1 week old)
- Too much detail (link to issues instead)
- Historical log (keep only recent)
- Vague status updates

## Integration with AI Agents

AI agents check project_state.md to:
- Understand what's being worked on
- Avoid conflicting changes
- Prioritize their suggestions
- Provide contextual help

## File Location

```
project/
└── .context/
    └── project_state.md  # Current project state
```

## Quick Update Template

When updating, use this format:

```markdown
## Update [Date]

**Completed**: [what was finished]
**In Progress**: [what's being worked on]
**Next**: [upcoming priority]
**Blocked**: [any blockers] (if applicable)
```
