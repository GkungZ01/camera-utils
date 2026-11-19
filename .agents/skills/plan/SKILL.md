---
name: plan
description: Slash command /plan to create implementation plans, architectural designs, and feature specifications with checkpoints.
---

# /plan - Implementation & Architecture Planner

Triggered when the user types `/plan` or requests a technical plan.

## Workflow
1. **Clarify Requirements:** Ask missing information in a numbered bullet list if unclear.
2. **Draft Plan:** Create or update `implementation_plan.md` artifact.
3. **Include Skills Required:** Explicitly include `## 🧰 Skills Required` section.
4. **Trigger Proceed Button:** Set `request_feedback: true` on `implementation_plan.md`.
5. **Halt & Wait:** Stop and wait for user approval before modifying code or running tools.
