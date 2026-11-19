---
name: init
description: Slash command /init to scan, inspect, and summarize workspace rules and project documentation (.agents/ and .agents/doc/) at session start.
---

# /init - Workspace & Project Context Initializer

Triggered when the user types `/init` or starts a new session.

## Workflow
1. **Read Core Guidelines:** Read `AGENTS.md` to refresh workspace behavior rules, checkpoints, and skills registry.
2. **Inspect Project Documentation:** Read `.agents/doc/index.md`, `.agents/doc/stack.md`, `.agents/doc/design_system.md`, `.agents/doc/structure.md`, and `.agents/doc/conventions.md`.
3. **Auto-Generate if Missing:** If `.agents/doc/` files do not exist, scan `package.json` and directory structure to create initial drafts.
4. **Output Readiness Summary:** Present a short, concise summary (in Caveman Thai) confirming active rules, tech stack, and design system status.
