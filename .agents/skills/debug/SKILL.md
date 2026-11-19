---
name: debug
description: Slash command /debug for systematic bug diagnosis, log inspection, and live browser testing with agent-browser.
---

# /debug - Systematic Bug Diagnosis & Browser QA

Triggered when the user types `/debug` or reports a bug/error.

## Workflow
1. **Fetch Empirical Evidence:** Inspect full error logs or stack trace silently. Never guess.
2. **Isolate Root Cause:** Trace upstream data provider / function signature.
3. **Live Web Inspection (if Web/UI):** Delegate to `agent-browser` skill (`agent-browser open <url>`, `snapshot`, `screenshot --annotate`).
4. **Minimal Fix:** Apply minimal root-cause patch (no swallowing errors or dummy fallbacks).
5. **Verify:** Re-run build/test or re-inspect browser state to prove clean success.
