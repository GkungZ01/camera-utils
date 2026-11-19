---
name: agent-browser
description: >
  Browser automation & web debugging CLI for AI agents using vercel-labs/agent-browser.
  Use for web inspection, UI debugging, E2E testing, visual verification, form filling,
  clicking, element state checks, screenshot capturing, and DOM data extraction.
---

# Agent Browser Skill (Web Debugging & Automation)

This skill positions the AI Agent to control Chrome/Chromium via Vercel's `agent-browser` CLI for page inspection, interactive web debugging, layout testing, and E2E visual QA.

---

## 🛠️ Prerequisite Check & Setup
Before executing browser tasks, ensure the CLI is available:
```bash
agent-browser --version || (npm install -g agent-browser && agent-browser install)
```

---

## 🛑 Checkpoint System
Halt and wait for user confirmation at key stages:
- **🛑 CP1 (Before Launch):** Confirm target URL, scope, and non-destructive action plan.
- **🛑 CP2 (After Snapshot/Initial State):** Present initial page state (screenshot/text summary) before performing form submits or state-changing clicks.
- **🛑 CP3 (After Verification):** Present final findings, console logs, or screenshots before closing the browser session.

---

## ⚡ Core Workflow & Token-Efficient Commands

### 1. Open Page & Navigation
```bash
agent-browser open <url>              # Launch browser & navigate to URL
agent-browser get url                 # Verify current URL
agent-browser get title               # Get page title
```

### 2. Low-Token Element Inspection (Accessibility Tree)
Always prefer `@eN` references over dumping raw HTML:
```bash
agent-browser snapshot                # Get compact accessibility tree with @e1, @e2 refs
```

### 3. Interaction & Input (Ref-based)
```bash
agent-browser click @e1               # Click element by ref
agent-browser fill @e2 "text"         # Clear and fill input by ref
agent-browser press Enter             # Press key
agent-browser select @e3 "option"     # Select dropdown option
agent-browser hover @e4               # Hover element
agent-browser scroll down 300         # Scroll page
```

### 4. Debugging & Visual QA
```bash
agent-browser screenshot page.png               # Capture screenshot
agent-browser screenshot --annotate             # Capture screenshot with numbered element labels
agent-browser get text @e1                      # Extract text from element
agent-browser eval "console.log('debug')"       # Execute custom JavaScript in page context
```

### 5. Session Termination
```bash
agent-browser close                             # Close active browser session
```

---

## 🔒 Safety & Best Practices
- **No Password Exposure:** Never type or store real production passwords or secret keys.
- **Annotated Screenshots:** Use `screenshot --annotate` when diagnosing layout issues or asking user for visual verification.
- **Graceful Failure:** If a click fails due to a modal overlay, snapshot again, dismiss the overlay, and retry.
