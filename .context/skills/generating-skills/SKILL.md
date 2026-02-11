---
name: generating-skills
description: Create new agent skills following SKILL.md best practices. Use when creating, validating, or improving skill files for AI agent consumption.
---

# Generating Skills

Create effective skills that AI agents can discover and use.

## When to use

- Creating a new skill from scratch
- Validating existing skill format
- Improving skill discoverability
- Converting documentation to skill format

## Standard Folder Structure

```
skill-name/
├── SKILL.md        # Required: instructions + metadata
├── scripts/        # Optional: executable utilities
├── references/     # Optional: detailed documentation
└── assets/         # Optional: templates, resources
```

For simple skills, a single `SKILL.md` file is sufficient.

## YAML Frontmatter Requirements

```yaml
---
name: skill-name
description: What the skill does and when to use it. Third person. Max 1024 chars.
---
```

### Name Validation

- Maximum 64 characters
- Lowercase letters, numbers, hyphens only
- No consecutive hyphens (`--`)
- Cannot start/end with hyphen
- Pattern: `^[a-z0-9]+(-[a-z0-9]+)*$`

**Good names**: `api-design`, `git-workflow`, `processing-pdfs`
**Bad names**: `APIDesign`, `git--workflow`, `-api-design`

### Description Guidelines

- Write in third person
- Include both what it does AND when to use it
- Be specific with trigger terms
- Max 1024 characters

**Good**: `Design REST APIs with consistent patterns. Use when creating endpoints or handling errors.`
**Bad**: `Helps with APIs`

## SKILL.md Template

```markdown
---
name: skill-name
description: Brief description in third person. Include when to use.
---

# Skill Name

Brief overview.

## When to use

- Trigger condition 1
- Trigger condition 2

## Quick start

Minimal example to get started.

## Implementation steps

1. Step 1
2. Step 2

## Best practices

### ✅ Do
- Good practice

### ❌ Avoid
- Anti-pattern

## References

- [Link](url)
```

## Token Efficiency

- Keep SKILL.md under 500 lines
- Split content into separate files for complex skills
- Use progressive disclosure (reference files loaded on-demand)
- Assume Claude knows common patterns

## Workflow Checklist

Copy and track progress:

```
- [ ] Name follows lowercase-hyphen format
- [ ] Description is specific and includes triggers
- [ ] "When to use" section present
- [ ] Quick start example included
- [ ] Best practices documented
- [ ] Under 500 lines
```
