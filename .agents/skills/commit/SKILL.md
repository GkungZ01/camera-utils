---
name: commit
description: Slash command /commit to generate concise Conventional Commit messages.
---

# /commit - Ultra-Compressed Git Commit Generator

Triggered when the user types `/commit` or asks for commit message generation.

## Workflow
1. **Inspect Staged Changes:** Check `git status` and `git diff --cached`.
2. **Format Conventional Commit:**
   - `<type>(<scope>): <subject>` (Subject ≤50 chars, imperative mood, lowercase)
   - Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
3. **No Noise:** Body only when rationale ("why") is non-obvious.
