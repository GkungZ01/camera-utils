---
name: design
description: Slash command /design to launch frontend visual design engineering workflow.
---

# /design - Frontend Visual Design Engineering

Triggered when the user types `/design` or requests UI/UX component creation.

## Workflow
1. **Delegate to `frontend-design`:** Load `skills/frontend-design/SKILL.md`.
2. **Declare Design System (CP1):** Define colors (oklch), non-cliché fonts, spacing, radius, and motion before writing code. Wait for approval.
3. **Build v0 Draft (CP2):** Layout skeleton + placeholders.
4. **Full Build & Self-Critique (CP3):** Final components + self-critique rubric score (1-10).
