---
name: review
description: Slash command /review to audit code changes for quality, bugs, security vulnerabilities, and token efficiency.
---

# /review - Code Quality & Security Auditor

Triggered when the user types `/review` or asks for code review.

## Workflow
1. **Inspect Diff / Files:** Review staged/unstaged changes or target files.
2. **5-Point Audit:**
   - Security (no hardcoded secrets, unsafe queries, or open permissions)
   - Performance & Token Usage
   - Code standards & Clean Code rules
   - Edge-case handling
   - Redundant or unused code
3. **Structured Feedback:** Report findings by Severity (🔴 High, 🟡 Medium, 🔵 Low) with one-line location, problem, and fix.
