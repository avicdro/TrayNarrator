---
name: creating-rules
description: Define coding rules and standards documentation. Use when establishing or updating coding conventions, linter rules, or team standards.
---

# Creating Coding Rules

Define and document coding standards that AI agents and developers must follow.

## When to use

- Establishing new coding conventions
- Documenting existing team standards
- Adding linter or formatter rules
- Clarifying ambiguous code patterns

## Rule Document Structure

```markdown
# Rule: [Rule Name]

> Brief description of the rule

## Why this rule exists

[Explain the problem this prevents or benefit it provides]

## Requirements

- Requirement 1
- Requirement 2

## ✅ Correct Examples

[Code showing correct usage]

## ❌ Incorrect Examples

[Code showing what to avoid]

## Exceptions

[Valid exceptions or "No exceptions"]

## Enforcement

- [ ] Linter rule: [rule name]
- [ ] Code review checklist
- [ ] Automated test
```

## Common Rule Categories

### Code Style

- Naming conventions (camelCase, PascalCase)
- File organization
- Import ordering
- Comment standards

### Architecture

- Component structure
- State management patterns
- API design patterns
- Error handling

### Quality

- Test coverage requirements
- Performance thresholds
- Security practices
- Accessibility standards

## Writing Effective Rules

### ✅ Do

- Provide concrete examples
- Explain the "why" not just the "what"
- Include both correct and incorrect examples
- Make rules enforceable (linter, tests)

### ❌ Avoid

- Vague requirements
- Rules without examples
- Conflicting rules
- Overly strict exceptions

## File Organization

```
.context/
└── rules/
    ├── coding-standards.md    # Main standards
    ├── naming-conventions.md  # Naming rules
    ├── testing-rules.md       # Test requirements
    └── security-rules.md      # Security practices
```
