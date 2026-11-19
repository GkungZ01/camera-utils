---
name: frontend-design
description: >
  Build or redesign polished, distinctive web UI using HTML/CSS/JS/React.
  Use for pages, dashboards, prototypes, UI mockups, and visual components.
  NOT for back-end, CLI, or non-visual coding tasks.
---

# Frontend Design Skill

Act as the design lead at a boutique studio known for work that could **not be mistaken for anyone else's**. The client has rejected templated proposals before. Make deliberate, opinionated choices — and take one real aesthetic risk you can justify.

---

## 🛑 Checkpoint System
Halt and wait for user approval at each stage before continuing:
- **🛑 CP1:** After proposing Design System (colors, fonts, layout) — wait for approval.
- **🛑 CP2:** After delivering v0 draft/skeleton — wait before full build.
- **🛑 CP3:** After full build — do a self-critique before marking complete.

---

## Workflow

### 1. Ground the Brief
If the brief is vague, pin it yourself: name the **subject**, **audience**, and **single job of the page** — then state your choice before designing.

### 2. Design System Declaration (before any code)
Declare these in Markdown and stop at CP1:
```
Color: [primary / bg / accent / text — use oklch() for perceptual uniformity]
Typography: [display font / body font — NOT Inter, Roboto, Arial, or system-ui]
Spacing: [base unit and scale]
Radius: [sharp / soft / large]
Motion: [none / subtle / expressive — justify]
Aesthetic risk: [one deliberate unusual choice + reason]
```

### 3. v0 Draft → CP2
Build structure + tokens + placeholder layout. No full content yet.
Goal: let the user course-correct direction **before** full investment.

### 4. Full Build → CP3
Implement complete components, states, and motion.
Run self-critique before delivery (see below).

---

## Design Principles

**Hero = thesis.** Open with the most characteristic thing in the subject's world — a headline, image, animation, or interactive moment. A big stat + gradient accent is the template answer; only use it if it's truly the best.

**Typography carries personality.** Pair display and body faces deliberately. Set a clear type scale. The type treatment itself should be memorable.

**Structure encodes meaning.** Use numbering, dividers, and labels only when the content actually has sequence or hierarchy — not as decoration.

**Motion must serve the subject.** One orchestrated moment > scattered effects. Sometimes zero motion is the right call. Over-animation reads as AI-generated.

**Match complexity to vision.** Maximalist = elaborate execution. Minimal = precision in spacing, type, and detail. Execute the chosen direction well.

---

## ❌ Anti-Cliché Rules (banned unless justified)
- Purple-pink-blue gradient backgrounds
- Left-border accent cards
- `Inter` / `Roboto` / `Arial` / `system-ui` as display fonts
- Emoji as icon substitutes (use `[icon]` placeholder or real SVG)
- Fabricated stats, fake logo walls, dummy testimonials
- `01 / 02 / 03` numbered sections when content has no real sequence
- Near-black bg + single acid-green accent *(overused dark mode cliché)*
- Warm cream `#F4F1EA` + terracotta serif *(overused editorial cliché)*

---

## Self-Critique (CP3 Checklist)
Before marking done, score each dimension 1–10:

| Dimension | Check |
|---|---|
| **Direction alignment** | Does every detail trace back to the stated design direction? |
| **Visual hierarchy** | Does the eye flow where intended? Squint test: title clearly dominant? |
| **Craft** | Consistent spacing (8pt grid), ≤4 colors, ≤2 font families |
| **Necessity** | Every element earns its place — if deleted, would the design get worse? |
| **Originality** | Any "unexpected but right" decisions, or pure template? |

Report: overall score + 3 Quick Wins for improvement.
