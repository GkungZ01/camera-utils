---
name: learn
description: Slash command /learn to capture, clarify, and persist project-specific guidelines, themes, design tokens, and business rules into .agents/doc/.
---

# /learn - Project Context & Rule Persistence

Triggered when the user types `/learn <information>` or instructs AI to learn a new rule/preference.

## Workflow
1. **Clarification Check:** If the provided information is partial, ambiguous, or lacks key technical specs (e.g. missing hex color code or breakpoint):
   - **Halt and ask follow-up questions** in a **numbered bullet list** (one question per bullet).
   - Wait for user answers before persisting.
2. **Categorize Information:**
   - **Design / Theme / Styling:** Save to `.agents/doc/design_system.md`.
   - **Tech Stack / Libraries:** Save to `.agents/doc/stack.md`.
   - **Business Logic / Coding Rules:** Save to `.agents/doc/conventions.md`.
3. **Update Index:** Ensure `.agents/doc/index.md` lists all active documentation files.
4. **Confirm Persistence:** Provide a short receipt confirming what was learned and where it was saved.
