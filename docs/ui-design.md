# Quarterly Planning Application
## Visual Design System & UI/UX Specification

---

## 1. Design Philosophy

**Core Principles:**
- **Thoughtful Dark Mode** - Inspired by Apple's WWDC guidance on dark mode with careful attention to color vibrance, contrast, and eye comfort
- **Data-Rich Interface** - Surfaces metrics, trends, and insights that engineering managers value
- **Purposeful Polish** - Rich interactions that serve functional purposes, not decoration
- **Cross-Platform Excellence** - Desktop-first with seamless web compatibility

---

## 2. Color System

### Base Palette (Dark Mode Foundation)

```
Background Hierarchy:
├─ bg-primary:    #1c1c1e  (Main canvas)
├─ bg-secondary:  #2c2c2e  (Elevated surfaces, cards)
├─ bg-tertiary:   #3a3a3c  (Input fields, interactive elements)
└─ bg-overlay:    #48484a  (Modals, popovers)

Borders & Dividers:
├─ border-subtle:   rgba(255, 255, 255, 0.08)  (Soft separators)
├─ border-default:  rgba(255, 255, 255, 0.12)  (Standard borders)
└─ border-emphasis: rgba(255, 255, 255, 0.18)  (Active/hover states)

Text Hierarchy:
├─ text-primary:    rgba(255, 255, 255, 0.95)  (Headings, primary content)
├─ text-secondary:  rgba(255, 255, 255, 0.70)  (Body text, labels)
├─ text-tertiary:   rgba(255, 255, 255, 0.50)  (Subtle text, placeholders)
└─ text-disabled:   rgba(255, 255, 255, 0.30)  (Disabled states)
```

### Semantic Color Palette

**Primary Actions** (Apple Blue family)
```
├─ primary-50:  #0A84FF  (Bright, iOS-style blue - buttons, links)
├─ primary-60:  #0A7AEF  (Hover state)
└─ primary-70:  #0970DF  (Active/pressed state)
```

**Success States** (Vibrant Green)
```
├─ success-50:  #32D74B  (iOS green - properly allocated)
├─ success-60:  #2DC945  (Hover)
└─ success-bg:  rgba(50, 215, 75, 0.12)  (Success background tint)
```

**Warning States** (Punchy Orange)
```
├─ warning-50:  #FF9F0A  (iOS orange - under/over allocated by 10-25%)
├─ warning-60:  #EF9500  (Hover)
└─ warning-bg:  rgba(255, 159, 10, 0.12)  (Warning background tint)
```

**Error States** (Vivid Red)
```
├─ error-50:    #FF453A  (iOS red - conflicts, over-allocated >25%)
├─ error-60:    #EF3A30  (Hover)
└─ error-bg:    rgba(255, 69, 58, 0.12)  (Error background tint)
```

**Oncall Indicator** (Purple for distinctiveness)
```
├─ oncall-50:   #BF5AF2  (iOS purple - oncall weeks)
├─ oncall-60:   #B550E2  (Hover)
└─ oncall-bg:   rgba(191, 90, 242, 0.12)  (Oncall background tint)
```

**Unallocated State** (Muted attention)
```
├─ unallocated-50:  #FF6B6B  (Soft red - not as alarming as error)
├─ unallocated-bg:  rgba(255, 107, 107, 0.10)  (Background tint)
```

**Project Color Palette** (For visual differentiation in grid)
```
├─ project-blue:    #64D2FF  (Bright cyan-blue)
├─ project-green:   #63E6BE  (Mint green)
├─ project-yellow:  #FFD43B  (Warm yellow)
├─ project-orange:  #FFA94D  (Soft orange)
├─ project-red:     #FF8787  (Coral red)
├─ project-purple:  #CC5DE8  (Vibrant purple)
├─ project-pink:    #F783AC  (Soft pink)
├─ project-teal:    #4FD1C5  (Teal)
└─ project-indigo:  #748FFC  (Periwinkle)
```

---

## 3. Typography System

### Font Stack
```css
font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 
             'Segoe UI', system-ui, sans-serif;
```

### Type Scale

```
Display (Page titles):
├─ size: 28px
├─ weight: 600 (Semibold)
├─ line-height: 1.2
└─ letter-spacing: -0.02em

Heading 1 (Section headers):
├─ size: 20px
├─ weight: 600
├─ line-height: 1.3
└─ letter-spacing: -0.01em

Heading 2 (Subsections):
├─ size: 16px
├─ weight: 600
├─ line-height: 1.4
└─ letter-spacing: 0

Body (Default text):
├─ size: 14px
├─ weight: 400 (Regular)
├─ line-height: 1.5
└─ letter-spacing: 0

Body Emphasis:
├─ size: 14px
├─ weight: 500 (Medium)
├─ line-height: 1.5
└─ letter-spacing: 0

Caption (Labels, metadata):
├─ size: 12px
├─ weight: 400
├─ line-height: 1.4
├─ letter-spacing: 0.01em
└─ color: text-secondary

Small (Tiny labels, counts):
├─ size: 11px
├─ weight: 400
├─ line-height: 1.4
├─ letter-spacing: 0.01em
└─ color: text-tertiary

Monospace (Grid alignment, numbers):
├─ font-family: 'SF Mono', 'Consolas', monospace
├─ size: 13px
├─ weight: 400
└─ letter-spacing: 0
```

---

## 4. Spacing System

**8px Base Unit** - All spacing uses multiples of 4px or 8px

```
├─ xs:   4px   (Tight spacing within components)
├─ sm:   8px   (Component padding, small gaps)
├─ md:   16px  (Default component spacing)
├─ lg:   24px  (Section spacing)
├─ xl:   32px  (Major section breaks)
└─ 2xl:  48px  (Page-level spacing)
```

---

## 5. Component Library

### 5.1 Navigation & Layout

**Top Navigation Bar**
```
Height: 56px
Background: bg-secondary
Border-bottom: 1px solid border-default

Contents (left to right):
├─ App icon + "Quarterly Planner" (text-primary, Heading 2)
├─ Quarter selector dropdown "Q1 2025" (primary-50 when active)
├─ View tabs (Roadmap | Technical | Allocation) 
│  └─ Active tab: border-bottom 2px primary-50
├─ [Flexible spacer]
├─ Capacity indicator: "67.5 / 78 weeks allocated" 
│  └─ With mini progress bar (success-50 if healthy)
└─ File menu (☰ icon)
```

**Side Panel** (Technical Projects View only)
```
Width: 320px (collapsible)
Background: bg-primary
Border-right: 1px solid border-default

Shows:
├─ Project list with metrics
├─ Filter/search
└─ Add new project button
```

### 5.2 Data Tables

**Roadmap Projects Table**
```
Row height: 56px
Header row: bg-tertiary, text-secondary, Caption weight
Data rows: bg-secondary (alternating subtle stripe with bg-primary)
Hover: bg-tertiary

Columns:
├─ Project Name (300px, text-primary, Body Emphasis)
├─ Engineering Est. (80px, monospace, text-secondary)
├─ Science Est. (80px, monospace, text-secondary)
├─ Total Est. (80px, monospace, text-primary, Body Emphasis)
├─ Engineering Alloc. (100px, with status color)
├─ Science Alloc. (100px, with status color)
├─ Total Alloc. (100px, with status color)
├─ Start Date (100px, text-secondary)
├─ Launch Date (100px, text-secondary)
└─ Notes (flexible, text-tertiary, truncated)

Status Colors (Allocated columns):
├─ Match: success-50 text + success-bg background
├─ 10-25% off: warning-50 text + warning-bg background
├─ >25% off: error-50 text + error-bg background
```

**Technical Projects Table**
```
Similar structure to Roadmap table, with additional:
├─ Linked Roadmap Project column (with colored dot indicator)
└─ Progress indicator (allocated / estimated as mini bar)
```

### 5.3 Allocation Grid (Primary View)

**Grid Structure** (Time flows horizontally, following natural left-to-right reading pattern)
```
┌────────────────────────────────────────────────────────────────────────────────┐
│ ALLOCATION GRID                              [Paintbrush Mode: OFF ▼] [⚙]     │
├────────────────────────────────────────────────────────────────────────────────┤
│                │ Sprint 1  │  Sprint 2  │┆ Sprint 3  │┆ Sprint 4  │┆  ...    │
│                │ Jan 3 (W) │ Jan 10     │┆ Jan 17(W) │┆ Jan 24    │┆         │
│                │ Wk 1/13   │ Wk 2/13    │┆ Wk 3/13   │┆ Wk 4/13   │┆         │
├────────────────┼───────────┴────────────┼┴───────────┼┴───────────┼┴─────────┤
│ Alice K        │           │            │┆           │┆           │┆         │
│ Eng • 11.5/12  │  [cell]   │   [cell]   │┆  [cell]   │┆  [cell]   │┆         │
├────────────────┼───────────┼────────────┼┼───────────┼┼───────────┼┼─────────┤
│ Bob Martinez   │           │            │┆           │┆           │┆         │
│ Eng • 12/12    │  [cell]   │   [cell]   │┆  [cell]   │┆  [cell]   │┆         │
├────────────────┼───────────┼────────────┼┼───────────┼┼───────────┼┼─────────┤
│ Carol S        │           │            │┆           │┆           │┆         │
│ Sci • 6/6      │  [cell]   │   [cell]   │┆  [cell]   │┆  [cell]   │┆         │
├────────────────┼───────────┼────────────┼┼───────────┼┼───────────┼┼─────────┤
│ ...            │           │            │┆           │┆           │┆         │
└────────────────┴───────────┴────────────┴┴───────────┴┴───────────┴┴─────────┘

Note: Sprint separators (┆) shown as subtle vertical dashed lines every 2 weeks
```

**Grid Cell Specifications**
```
Cell size: 120px width × 48px height (optimized for horizontal scrolling)
Background: bg-tertiary
Border: 1px solid border-subtle
Border-radius: 6px (standalone) or 0px (when part of multi-week series)

Cell States:

1. Empty (unallocated):
   ├─ Background: bg-tertiary
   ├─ Border: 1px dashed border-default
   ├─ Border-radius: 6px
   ├─ Hover: border-emphasis, cursor: crosshair (in paintbrush mode)
   └─ Subtle icon: "+" in center (text-tertiary)

2. Single Week Project Allocation:
   ├─ Background: project color at 15% opacity (e.g., project-blue with 0.15 alpha)
   ├─ Border: 1px solid project color at 40% opacity
   ├─ Border-radius: 6px (all corners)
   ├─ Text: Project name (Body, text-primary, truncated)
   ├─ Hover: Brighten background to 20% opacity, show tooltip
   └─ Badge: "100%" if full week (top-right, text-tertiary, Small)

3. Multi-Week Project Allocation (CONNECTED VISUAL):
   When the same project spans consecutive weeks for an engineer:

   ┌─ First Cell (left edge):
   ├─ Border-radius: 6px 0 0 6px (rounded left, square right)
   ├─ Border-right: 0px (connects to next cell)
   ├─ Shadow: Subtle inset shadow on right edge

   ┌─ Middle Cells (continuation):
   ├─ Border-radius: 0 (square all sides)
   ├─ Border-left: 0px, Border-right: 0px
   ├─ Visual indicator: Thin 1px connecting line at top/bottom (project color at 60%)
   ├─ Slightly darker background (project color at 18% vs 15% for visual cohesion)

   ┌─ Last Cell (right edge):
   ├─ Border-radius: 0 6px 6px 0 (square left, rounded right)
   ├─ Border-left: 0px
   ├─ Shadow: Subtle inset shadow on left edge
   ├─ Badge: Shows total duration "3w" (Caption, text-primary, top-right)

   Purpose: Creates a visual "timeline bar" showing project duration at a glance
   Interaction: Hovering any cell in series highlights entire connected group

4. Split Allocation (2 projects):
   ├─ Visual: Vertical split with subtle gradient divider
   ├─ Left half: Project A (60%) with name truncated
   ├─ Right half: Project B (40%) with percentage badge
   ├─ Border: 1px solid with both project colors (subtle gradient)
   ├─ Border-radius: 6px
   └─ Hover: Show tooltip with both project details

5. Oncall:
   ├─ Background: oncall-bg
   ├─ Border: 2px solid oncall-50 (thicker for emphasis)
   ├─ Border-radius: 6px
   ├─ Icon: Phone icon (oncall-50, 16px, centered)
   ├─ Text: "Oncall" (Caption, text-primary, below icon)
   └─ Pattern: Subtle diagonal stripes (oncall-50 at 5% opacity)

6. Before Project Start Date:
   ├─ Normal project styling BUT
   ├─ Overlay: Diagonal hash pattern (border-subtle)
   ├─ Opacity: 60% on entire cell
   └─ Warning icon: Small "!" in top-left (warning-50)
```

**Column Headers (Week Dates + Sprints)**
```
Height: 64px (3 rows of information)
Background: bg-secondary
Border-bottom: 2px solid border-emphasis
Sticky: Yes (remains visible during vertical scroll)

Row 1 - Sprint Numbers:
├─ Height: 24px
├─ Sprint number: "Sprint 1", "Sprint 2" (Body Emphasis, text-primary)
├─ Colspan: 2 columns per sprint
├─ Background: subtle gradient (bg-secondary to bg-tertiary)
├─ Border-bottom: 1px solid border-subtle

Row 2 - Week Dates:
├─ Height: 20px
├─ Date: "Jan 3 (W)" (Body, text-primary)
├─ Day indicator included: "(W)" for Wednesday (text-secondary)

Row 3 - Week Progress:
├─ Height: 20px
├─ Week number: "Wk 1/13" (Caption, text-tertiary)
├─ Shows quarter progress

Sprint Separator (vertical):
├─ Dashed border: 2px dashed border-default (vertical line between sprint groups)
├─ Color: border-subtle
├─ Extra spacing: 4px padding left/right
├─ Spans full grid height
```

**Row Headers (Engineer/Scientist Names)**
```
Width: 180px (fixed, non-scrolling)
Background: bg-secondary
Border-right: 2px solid border-emphasis
Sticky: Yes (remains visible during horizontal scroll)

Content (2 rows per engineer):
Row 1 - Name:
├─ Engineer name (Body Emphasis, text-primary)
├─ Right-aligned role badge: "Eng" or "Sci" (Caption, bg-tertiary, rounded)

Row 2 - Capacity:
├─ Capacity indicator: "11.5 / 12 weeks" (Monospace, Caption)
├─ Color-coded by health:
│  ├─ success-50 if within 0.5 weeks (healthy)
│  ├─ warning-50 if off by 0.5-1 week (slight concern)
│  └─ error-50 if off by >1 week (over/under allocated)
├─ Mini progress bar (80px width, 4px height, shows allocated/capacity ratio)

Divider:
├─ Border-bottom: 1px solid border-default (between engineers)
├─ Hover: bg-tertiary (entire row header for selection feedback)
```

### 5.4 Interactive Components

**Paintbrush Mode Toggle**
```
Position: Top-right of grid
Style: Segmented control

States:
├─ OFF: bg-tertiary, text-secondary
└─ ON: primary-50 background, white text, glow effect

When ON:
├─ Project selector appears (dropdown)
├─ Cursor changes to paintbrush icon
└─ Click-and-drag paints selected project onto cells
```

**Project Selector Dropdown**
```
Trigger: bg-tertiary, border-default, rounded corners
Height: 36px
Width: 240px

Dropdown panel:
├─ Background: bg-overlay with backdrop blur
├─ Shadow: 0 8px 24px rgba(0,0,0,0.4)
├─ Border-radius: 8px
├─ Search box at top (bg-tertiary)
├─ Project list:
│  ├─ Each item: 32px height
│  ├─ Color dot indicator (8px, project color)
│  ├─ Project name (Body, text-primary)
│  ├─ Allocated weeks (Caption, text-tertiary, right-aligned)
│  └─ Hover: bg-tertiary
└─ "Oncall" at bottom (oncall-50 styling, separator above)
```

**Cell Context Menu** (Right-click on cell)
```
Background: bg-overlay with backdrop blur
Shadow: 0 4px 16px rgba(0,0,0,0.5)
Border: 1px solid border-emphasis
Border-radius: 8px
Padding: 4px

Menu items:
├─ "Assign Project..." (with project-blue icon)
├─ "Split Allocation..." (with split icon) 
├─ "Clear Assignment" (with × icon, error-50 when hovered)
├─ [Separator]
├─ "Copy Assignment" (Cmd+C hint)
└─ "Paste Assignment" (Cmd+V hint)

Each item:
├─ Height: 32px
├─ Padding: 8px 12px
├─ Hover: bg-tertiary
├─ Icon: 16px, text-secondary
└─ Text: Body, text-primary
```

**Split Allocation Modal**
```
Overlay: rgba(0, 0, 0, 0.6) with backdrop blur
Modal:
├─ Width: 400px
├─ Background: bg-secondary
├─ Border: 1px solid border-emphasis
├─ Border-radius: 12px
├─ Shadow: 0 16px 48px rgba(0,0,0,0.6)

Header:
├─ "Split Week Allocation" (Heading 1)
├─ Close button (×, top-right)

Content:
├─ Week info: "Week of Jan 3 - Alice K" (Caption, text-secondary)
├─ Project A selector + percentage slider (0-100%)
├─ Visual: Horizontal bar showing split (live preview)
├─ Project B selector + percentage (auto-calculated)
└─ Validation: "Total must equal 100%" if invalid

Footer:
├─ Cancel button (bg-tertiary, text-secondary)
└─ Apply button (primary-50, white text)
```

**Tooltip** (On hover of grid cells, project names, metrics)
```
Background: bg-overlay with backdrop blur
Border: 1px solid border-emphasis
Border-radius: 6px
Padding: 8px 12px
Shadow: 0 2px 8px rgba(0,0,0,0.4)
Max-width: 280px

Content (for grid cell):
├─ Project name (Body Emphasis, text-primary)
├─ Roadmap project link (Caption, primary-50)
├─ Allocation: "100% of week" (Caption, text-secondary)
├─ [Divider]
├─ Total allocated to project: "6.5 / 8 weeks" (Caption)
└─ Status: "On track" with success-50 dot

Arrow pointer: Matches border color, points to hovered element
```

### 5.5 Status Indicators & Metrics

**Capacity Health Badge**
```
Used in: Column headers, project rows, top nav

Visual: Pill shape, 8px border-radius
Height: 22px
Padding: 4px 8px

Variants:
├─ Healthy (within 0.5 weeks):
│  ├─ Background: success-bg
│  ├─ Text: success-50
│  └─ Icon: ✓ checkmark
├─ Warning (0.5-1 week off):
│  ├─ Background: warning-bg
│  ├─ Text: warning-50
│  └─ Icon: ⚠ triangle
└─ Critical (>1 week off):
   ├─ Background: error-bg
   ├─ Text: error-50
   └─ Icon: ✕ x-mark

Text: Monospace, "11.5 / 12" format
```

**Progress Bar** (Mini version for tables)
```
Width: 80px
Height: 6px
Border-radius: 3px
Background: bg-tertiary

Fill:
├─ Height: 6px
├─ Border-radius: 3px
├─ Color: success-50 (if on track)
├─ Color: warning-50 (if 10-25% off)
├─ Color: error-50 (if >25% off)
└─ Animated shimmer on hover
```

**Project Color Dot**
```
Size: 8px diameter
Border-radius: 50%
Background: Assigned project color
Border: 1px solid rgba(255,255,255,0.2) (for definition)

Used in: Dropdowns, tooltips, table rows
```

### 5.6 Buttons & Actions

**Primary Button**
```
Background: primary-50
Color: white
Height: 36px
Padding: 0 16px
Border-radius: 6px
Font: Body Emphasis
Shadow: 0 1px 3px rgba(10, 132, 255, 0.3)

Hover: primary-60, shadow increases
Active: primary-70, shadow decreases
Disabled: bg-tertiary, text-disabled, no shadow
```

**Secondary Button**
```
Background: bg-tertiary
Color: text-primary
Border: 1px solid border-default
Height: 36px
Padding: 0 16px
Border-radius: 6px

Hover: bg-overlay, border-emphasis
Active: bg-tertiary, border-default
```

**Icon Button**
```
Size: 32px × 32px
Background: transparent
Color: text-secondary
Border-radius: 6px

Hover: bg-tertiary, color: text-primary
Active: bg-overlay
```

**Floating Action Button** (For "Add Project")
```
Size: 56px diameter
Background: primary-50
Color: white
Border-radius: 50%
Shadow: 0 4px 16px rgba(10, 132, 255, 0.4)
Icon: + symbol, 24px

Position: Fixed, bottom-right: 32px
Hover: Scale 1.05, shadow increases
Active: Scale 0.98
```

---

## 6. Interaction Patterns

### 6.1 Paintbrush Mode

**Activation:**
1. Toggle paintbrush mode ON (top-right)
2. Project selector appears automatically
3. Select a project from dropdown
4. Cursor changes to paintbrush icon

**Usage:**
```
Single Click: Assign selected project to that cell (100%)
Click + Drag: Paint across multiple cells in same row
Shift + Click: Paint entire column for that engineer
Cmd/Ctrl + Click: Add to existing allocation (creates split)
Esc: Exit paintbrush mode
```

**Visual Feedback:**
- Hover preview: Cell shows ghost image of project (30% opacity)
- Drag path: All cells in path highlight with border-emphasis
- Success: Brief success-50 glow animation on painted cells
- Invalid target: Cell shakes with error-50 border if already full

### 6.2 Drag & Drop Allocation

**Standard Mode** (paintbrush OFF):
```
1. Click cell border: Select cell
2. Drag to another cell: Move allocation
3. Drag to empty space: Delete allocation
4. Drag handle on multi-selected cells: Move batch
```

**Visual States:**
- Dragging: Cell lifts with shadow, semi-transparent
- Drop target valid: Green border pulse
- Drop target invalid: Red border + shake
- Drop complete: Smooth animation to new position

### 6.3 Keyboard Shortcuts

```
Navigation:
├─ Tab: Move between cells (left to right, top to bottom)
├─ Shift + Tab: Move backwards
├─ Arrow keys: Navigate grid
└─ Enter: Open project selector for current cell

Actions:
├─ Cmd/Ctrl + C: Copy cell allocation
├─ Cmd/Ctrl + V: Paste allocation
├─ Cmd/Ctrl + X: Cut allocation
├─ Delete/Backspace: Clear cell
├─ Cmd/Ctrl + Z: Undo
├─ Cmd/Ctrl + Shift + Z: Redo
└─ Esc: Cancel current operation

Views:
├─ Cmd/Ctrl + 1: Roadmap view
├─ Cmd/Ctrl + 2: Technical view
├─ Cmd/Ctrl + 3: Allocation grid
└─ Cmd/Ctrl + S: Save plan
```

### 6.4 Contextual Information

**Hover Behavior:**
- **0-300ms**: No action (prevents flicker)
- **300ms**: Tooltip appears smoothly (fade in 150ms)
- **Tooltip content**: Context-dependent rich information
- **Hover off**: Tooltip fades out (100ms delay)

**Tooltip Content by Element:**

Grid Cell:
```
┌─────────────────────────────────┐
│ Payment Gateway Integration     │
│ Linked to: Q1 Platform Roadmap  │ (clickable)
│ Allocation: 100% of week        │
├─────────────────────────────────┤
│ Project Progress: 4.5 / 8 weeks │
│ Status: On track ✓              │
│                                 │
│ Click to edit • Right-click for options │
└─────────────────────────────────┘
```

Engineer Name Header:
```
┌─────────────────────────────────┐
│ Alice K - Senior Engineer       │
├─────────────────────────────────┤
│ Q1 Capacity: 11.5 / 12 weeks    │
│ Utilization: 96% ✓              │
│                                 │
│ Current Projects:               │
│ • Payment Gateway (4.5 weeks)   │
│ • Data Pipeline (3 weeks)       │
│ • Oncall (2 weeks)              │
│ • Unallocated (2 weeks)         │
└─────────────────────────────────┘
```

Project in Table:
```
┌─────────────────────────────────┐
│ Payment Gateway Integration     │
├─────────────────────────────────┤
│ Engineering: 8 weeks estimated  │
│ Allocated: 4.5 weeks so far     │
│                                 │
│ Assigned to:                    │
│ • Alice K (2.5 weeks)           │
│ • Bob M (2 weeks)               │
│                                 │
│ Start: Jan 3 • Launch: Mar 15   │
└─────────────────────────────────┘
```

### 6.5 Animations & Transitions

**Timing Functions:**
```css
--transition-quick:  150ms cubic-bezier(0.4, 0, 0.2, 1);
--transition-base:   250ms cubic-bezier(0.4, 0, 0.2, 1);
--transition-slow:   400ms cubic-bezier(0.4, 0, 0.2, 1);
--spring:            cubic-bezier(0.34, 1.56, 0.64, 1); /* For playful elements */
```

**Micro-interactions:**
- Button press: Scale 0.98, duration 150ms
- Cell selection: Border grows from 1px to 2px, color shift
- Project assignment: Brief success glow (success-50, 300ms fade)
- Drag start: Lift shadow + slight scale (1.02)
- Dropdown open: Fade + slide down (250ms)
- Modal open: Backdrop fade (300ms) + modal scale (0.95 to 1, 400ms, spring)

---

## 7. Layout Specifications

### 7.1 Roadmap View

```
┌─────────────────────────────────────────────────────────────────────────┐
│ Top Nav (56px height)                                                   │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─ Roadmap Projects ──────────────────────────────────────────────┐  │
│  │                                                                   │  │
│  │  [Search/Filter bar]          [+ Add Roadmap Project button]    │  │
│  │                                                                   │  │
│  │  ┌──────────────────────────────────────────────────────────┐  │  │
│  │  │ Project Name │ Eng Est │ Sci Est │ Total │ ... │ Notes  │  │  │
│  │  ├──────────────────────────────────────────────────────────┤  │  │
│  │  │ Q1 Platform  │  24 ↓   │   8     │  32   │ ... │  ...   │  │  │
│  │  │ Payment Gtwy │  8 ✓    │   0     │  8    │ ... │  ...   │  │  │
│  │  │ Data Pipeline│  16 ⚠   │   6 ✓   │  22   │ ... │  ...   │  │  │
│  │  │ ...                                                       │  │  │
│  │  └──────────────────────────────────────────────────────────┘  │  │
│  │                                                                   │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  ┌─ Quarter Summary ─────────────────────────────────────────────┐    │
│  │                                                                 │    │
│  │  Total Capacity:    78 weeks (6 Eng × 12 + 1 Sci × 6)        │    │
│  │  Total Allocated:   67.5 weeks                                 │    │
│  │  Utilization:       87% ✓                                      │    │
│  │                                                                 │    │
│  │  [Capacity chart: Horizontal bar with Eng/Sci breakdown]      │    │
│  │                                                                 │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 7.2 Technical Projects View (with Side Panel)

```
┌─────────────────────────────────────────────────────────────────────────┐
│ Top Nav (56px height)                                                   │
├───────────────┬─────────────────────────────────────────────────────────┤
│               │                                                         │
│  Side Panel   │  ┌─ Technical Projects ─────────────────────────┐     │
│  (320px)      │  │                                                │     │
│               │  │  [Search/Filter]    [+ Add Tech Project]      │     │
│  Filters:     │  │                                                │     │
│  ☑ All        │  │  ┌───────────────────────────────────────┐   │     │
│  ☐ On Track   │  │  │ Tech Project │ Roadmap │ Est │ Alloc │   │     │
│  ☐ At Risk    │  │  ├───────────────────────────────────────┤   │     │
│  ☐ No Link    │  │  │ Auth Service │ Q1 Plat │  6  │ 6 ✓  │   │     │
│               │  │  │ Pay API      │ Pay Gtw │  8  │ 4.5⚠ │   │     │
│  Sort by:     │  │  │ ML Pipeline  │ Data... │ 12  │ 8 ↓  │   │     │
│  • Roadmap    │  │  │ ...                                    │   │     │
│  ○ Status     │  │  └───────────────────────────────────────┘   │     │
│  ○ Allocation │  │                                                │     │
│               │  │  ┌─ Project Details (selected) ──────────┐   │     │
│  [Collapse]   │  │  │ Pay API Integration                    │   │     │
│               │  │  │ Linked to: Payment Gateway ↗           │   │     │
│               │  │  │ Est: 8 weeks │ Alloc: 4.5 weeks        │   │     │
│               │  │  │                                         │   │     │
│               │  │  │ Assigned Engineers:                    │   │     │
│               │  │  │ • Alice K (2.5 weeks) [Timeline──■]   │   │     │
│               │  │  │ • Bob M (2 weeks)     [Timeline──■]   │   │     │
│               │  │  │                                         │   │     │
│               │  │  │ Start: Jan 3 • ECD: Feb 14             │   │     │
│               │  │  │ Launch: Mar 15                         │   │     │
│               │  │  │                                         │   │     │
│               │  │  │ Notes: [Expandable text field]         │   │     │
│               │  │  └─────────────────────────────────────────┘   │     │
│               │  │                                                │     │
│               │  └────────────────────────────────────────────────┘     │
│               │                                                         │
└───────────────┴─────────────────────────────────────────────────────────┘
```

### 7.3 Allocation Grid (Primary View)

**Design Rationale:**
This layout follows established UX conventions for timeline-based data:
- **Time on X-axis**: Horizontal time flow matches natural left-to-right reading, Gantt charts, and calendar conventions
- **People on Y-axis**: Vertical scanning of team members is more efficient; allows quick comparison across individuals
- **Connected Multi-week Projects**: Visual continuity shows project duration at a glance, reducing cognitive load
- **Sticky Headers**: Both row (engineers) and column (weeks) headers remain visible during scroll, maintaining context
- **Sprint Separators**: Vertical dashed lines every 2 weeks create natural visual groupings without disrupting horizontal flow

**Time flows horizontally (standard timeline convention)**

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│ Top Nav (56px)                                   [Paintbrush Mode: OFF ▼] [Filter ⚙]    │
├──────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                          │
│  ┌────────────────┬──────────────┬──────────────┬┆┬──────────────┬──────────────┬──┐  │
│  │                │   Sprint 1   │   Sprint 1   │┆│   Sprint 2   │   Sprint 2   │  │  │
│  │                │  Jan 3 (W)   │  Jan 10      │┆│  Jan 17 (W)  │  Jan 24      │→ │  │
│  │                │  Wk 1/13     │  Wk 2/13     │┆│  Wk 3/13     │  Wk 4/13     │  │  │
│  ├────────────────┼──────────────┼──────────────┼┆┼──────────────┼──────────────┼──┤  │
│  │ Alice Kim      │              │              │┆│              │              │  │  │
│  │ Eng • 11.5/12  │ ╔═══════════════════════════╗│┆│ ╔═══Pay══════════════════════╗  │
│  │ ████████░░ 96% │ ║ Payment API (3w)       3w ║│┆│ ║ 60%   │ Data │ 40%   │  │  │
│  ├────────────────┼──────────────┼──────────────┼┆┼──────────────┼──────────────┼──┤  │
│  │ Bob Martinez   │              │              │┆│              │              │  │  │
│  │ Eng • 12/12    │ ╔══ML Pipeline══════════════════════════════════════════════╗  │  │
│  │ ████████████100│ ║ ML Pipeline (4w)                                       4w ║  │  │
│  ├────────────────┼──────────────┼──────────────┼┆┼──────────────┼──────────────┼──┤  │
│  │ Carol Smith    │              │              │┆│              │              │  │  │
│  │ Sci • 6/6      │ ╔═Research═╗│ ╔═Research═╗│┆│              │ ╔═Research═╗│  │  │
│  │ ██████████ 100%│ ║ Research ║│ ║ Research ║│┆│ ┌──────────┐ │ ║ Research ║│  │  │
│  │                │ ╚═══════════╝│ ╚═══════════╝│┆│ │Unallocat.│ │ ╚═══════════╝│  │  │
│  ├────────────────┼──────────────┼──────────────┼┆┼──────────────┼──────────────┼──┤  │
│  │ Dave Roberts   │              │              │┆│              │              │  │  │
│  │ Eng • 12/12    │ ╔═Auth Service═════════════════╗│ ┌───────────┐ ┌──────────┐│  │  │
│  │ ████████████100│ ║ Auth Service (3w)          3w ║│ │  Oncall   │ │   ...    ││  │  │
│  ├────────────────┼──────────────┼──────────────┼┆┼──────────────┼──────────────┼──┤  │
│  │ ...            │              │              │┆│              │              │  │  │
│  └────────────────┴──────────────┴──────────────┴┆┴──────────────┴──────────────┴──┘  │
│                                                   ┆← Sprint separator (vertical)        │
│                                                                                          │
│  Legend: ╔═════════╗ = Multi-week project (connected cells with total duration)       │
│          ┌─────────┐ = Single week allocation                                          │
│          [Oncall] = Oncall week (purple, striped pattern)                              │
│          Pay 60% | Data 40% = Split allocation (vertical divider)                      │
│          ████████░░ = Capacity utilization bar (green/orange/red)                      │
│                                                                                          │
└──────────────────────────────────────────────────────────────────────────────────────────┘
```

**Grid Cell Visual Examples:**

```
Empty Cell:
┌──────────────┐
│              │  (bg-tertiary, dashed border)
│      +       │  (subtle + icon)
│              │
└──────────────┘

Single Week Allocation:
┌──────────────┐
│ Payment API  │  (project-blue bg at 15%, solid border, rounded corners)
│          100%│  (allocation badge)
└──────────────┘

Multi-Week Project (Connected Cells):
╔══════════════════════════════════════════╗
║ Payment API (3 weeks)                 3w ║  ← Shows total duration on last cell
╚══════════════════════════════════════════╝
 ↑              ↑                        ↑
 First cell     Middle cell(s)           Last cell
 (rounded left) (square sides)           (rounded right)

Visual breakdown:
╔═══════════╦═══════════╦═══════════╗
║ Pay API   ║ Pay API   ║ Pay API 3w║  (same project color, connected borders)
╚═══════════╩═══════════╩═══════════╝
 Week 1       Week 2       Week 3

Split Cell (Vertical Division):
┌──────────────┐
│ Pay     60%  │  (project-blue bg, left portion)
│      ┃       │  (vertical divider)
│ Data    40%  │  (project-green bg, right portion)
└──────────────┘

Oncall Cell:
┌──────────────┐
│ ╱╱╱╱╱╱╱╱╱╱╱ │  (diagonal stripes, oncall-bg)
│ 📞 Oncall    │  (oncall-50 icon + text)
│ ╱╱╱╱╱╱╱╱╱╱╱ │
└──────────────┘

Before Start Date:
┌──────────────┐
│ ░░░░░░░░░░░░ │  (hash overlay at 60% opacity)
│ ⚠ Pay API    │  (warning icon, muted)
│ ░░░░░░░░░░░░ │
└──────────────┘

Unallocated (Warning):
┌──────────────┐
│              │  (unallocated-bg, subtle red tint)
│   ⎯ ⎯ ⎯      │  (horizontal lines suggesting empty)
│              │
└──────────────┘
```

---

## 8. Data Visualization & Metrics

### 8.1 Capacity Dashboard (Top Nav)

```
┌────────────────────────────────────────────┐
│ Q1 2025 Capacity                           │
│                                            │
│ ████████████████████░░░░░░  67.5 / 78 wks │
│                                            │
│ Engineering: 58.5 / 72 weeks (81%)         │
│ Science:      9.0 / 6 weeks (150%) ⚠      │
└────────────────────────────────────────────┘

Colors:
├─ Filled portion: success-50 (if <95%)
├─ Filled portion: warning-50 (if 95-105%)
├─ Filled portion: error-50 (if >105%)
└─ Unfilled: bg-tertiary
```

### 8.2 Project Health Indicators

**Traffic Light System:**
```
✓ On Track (success-50):
  - Allocated weeks within ±5% of estimate
  - Start date confirmed
  - No blocking issues

⚠ At Risk (warning-50):
  - Allocated 10-25% off estimate
  - Start date uncertain
  - Some engineers unassigned

✕ Critical (error-50):
  - Allocated >25% off estimate
  - Missing start date or launch date
  - No engineers assigned yet
```

### 8.3 Engineer Utilization Chart

**In Technical Projects view side panel:**
```
Engineer Utilization:
┌──────────────────────────────────────┐
│ Alice K    ████████████░░ 96%  ✓    │
│ Bob M      ██████████████ 100% ✓    │
│ Carol S    ██████████████ 100% ✓    │
│ Dave R     ████████░░░░░░ 67%  ⚠    │
│ Eve T      ██████░░░░░░░░ 50%  ⚠    │
│ Frank L    ████████████░░ 92%  ✓    │
└──────────────────────────────────────┘
```

### 8.4 Sprint Burn-up/Burn-down

**Optional feature for Technical Projects detail:**
```
Project Timeline Visualization:
Week 1   Week 2   Week 3   Week 4   Week 5   Week 6
┌────────────────────────────────────────────────┐
│     ▄▄▄▄                                       │  Estimated
│   ▄▀    ▀▀▀▄▄▄▄                                │  (dotted line)
│ ▄▀              ▀▀▀▄▄▄▄                        │
│▀                      ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀ │
│                                                │
│ ████▓▓▓▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ │  Actual allocation
│ Alice  Bob    (unallocated)                    │  (by engineer)
└────────────────────────────────────────────────┘
```

---

## 9. File Management & Persistence

### 9.1 File Menu Dropdown

```
Trigger: ☰ icon (top-right nav)

Dropdown:
┌─────────────────────────────────┐
│ 📄 New Plan           Cmd+N     │
│ 📂 Open Plan...       Cmd+O     │
│ 💾 Save Plan          Cmd+S     │
│ 💾 Save Plan As...    Cmd+Sh+S  │
├─────────────────────────────────┤
│ 📤 Export to CSV                │
│ 📤 Export to Excel              │
│ 📥 Import from CSV              │
├─────────────────────────────────┤
│ ⚙️  Preferences...              │
│ ℹ️  About                        │
└─────────────────────────────────┘
```

### 9.2 File Format (JSON)

```json
{
  "version": "1.0",
  "quarter": "Q1 2025",
  "startDate": "2025-01-01",
  "sprintStartDay": "wednesday",
  "sprintLengthWeeks": 2,
  
  "engineers": [
    {
      "id": "eng_001",
      "name": "Alice K",
      "role": "engineer",
      "capacity": 12
    }
  ],
  
  "roadmapProjects": [
    {
      "id": "roadmap_001",
      "name": "Payment Gateway Integration",
      "engineeringEstimate": 8,
      "scienceEstimate": 0,
      "startDate": "2025-01-03",
      "launchDate": "2025-03-15",
      "notes": "Priority 1 - Q1 commitment",
      "color": "project-blue"
    }
  ],
  
  "technicalProjects": [
    {
      "id": "tech_001",
      "name": "Payment API",
      "roadmapProjectId": "roadmap_001",
      "engineeringEstimate": 8,
      "scienceEstimate": 0,
      "startDate": "2025-01-03",
      "color": "project-blue"
    }
  ],
  
  "allocations": [
    {
      "engineerId": "eng_001",
      "weekStartDate": "2025-01-03",
      "assignments": [
        {
          "technicalProjectId": "tech_001",
          "percentage": 100
        }
      ]
    }
  ]
}
```

### 9.3 Preferences Modal

```
┌─────────────────────────────────────────┐
│ Preferences                          ✕  │
├─────────────────────────────────────────┤
│                                         │
│ General                                 │
│ ├─ Sprint Start Day: [Wednesday ▼]     │
│ ├─ Sprint Length: [2 weeks ▼]          │
│ └─ Default Capacity: [12 weeks]        │
│                                         │
│ Theme                                   │
│ ├─ ● Dark Mode                          │
│ └─ ○ Light Mode (not yet supported)    │
│                                         │
│ Grid                                    │
│ ├─ Cell Width: [140px]                 │
│ ├─ Show Sprint Separators: ☑           │
│ └─ Highlight Weekends: ☐               │
│                                         │
│ Notifications                           │
│ ├─ Warn on over-allocation: ☑           │
│ └─ Show capacity alerts: ☑              │
│                                         │
│         [Cancel]  [Save Preferences]    │
└─────────────────────────────────────────┘
```

---

## 10. Responsive Behavior

### 10.1 Viewport Breakpoints

```
Desktop (Primary): 1440px+
├─ All features available
├─ Side panels visible
└─ Full grid width

Laptop: 1024px - 1439px
├─ Side panel collapsible
├─ Reduced grid column width (120px)
└─ Horizontal scroll for >6 engineers

Tablet: 768px - 1023px
├─ Side panel overlay (not inline)
├─ Simplified table views
└─ Grid becomes primary focus

Mobile: <768px
├─ Not primary target
├─ Read-only mode recommended
└─ Consider "View on Desktop" message
```

### 10.2 Horizontal Scroll (for many engineers)

```
When >8 engineers:
├─ Freeze left columns (Sprint, Week)
├─ Horizontal scrollbar at bottom
├─ Scroll position indicator: "Showing 4-8 of 12 engineers"
└─ Keyboard: Shift + Arrow keys to scroll
```

---

## 11. Accessibility

### 11.1 Keyboard Navigation

```
Full keyboard support:
├─ All interactive elements focusable
├─ Focus indicators: 2px outline, primary-50
├─ Skip links for large grids
└─ Logical tab order
```

### 11.2 Screen Reader Support

```
ARIA labels:
├─ Grid cells: "Week of January 3, Alice K, Payment API, 100% allocated"
├─ Status badges: "On track, 11.5 of 12 weeks allocated"
├─ Buttons: Clear action labels
└─ Tables: Proper header associations
```

### 11.3 Color Contrast

```
All text meets WCAG AA standards:
├─ text-primary on bg-primary: 12.5:1
├─ text-secondary on bg-primary: 8.2:1
├─ primary-50 on bg-primary: 5.8:1
└─ success/warning/error on bg-primary: >4.5:1

Pattern overlays for colorblind users:
├─ Oncall: Diagonal stripes + icon
├─ Unallocated: Horizontal lines
└─ Split allocation: Vertical divider
```

---

## 12. Assets for Development

### 12.1 Design Tokens (CSS Variables)

```css
:root {
  /* Colors - Backgrounds */
  --bg-primary: #1c1c1e;
  --bg-secondary: #2c2c2e;
  --bg-tertiary: #3a3a3c;
  --bg-overlay: #48484a;
  
  /* Colors - Borders */
  --border-subtle: rgba(255, 255, 255, 0.08);
  --border-default: rgba(255, 255, 255, 0.12);
  --border-emphasis: rgba(255, 255, 255, 0.18);
  
  /* Colors - Text */
  --text-primary: rgba(255, 255, 255, 0.95);
  --text-secondary: rgba(255, 255, 255, 0.70);
  --text-tertiary: rgba(255, 255, 255, 0.50);
  --text-disabled: rgba(255, 255, 255, 0.30);
  
  /* Colors - Semantic */
  --primary-50: #0A84FF;
  --primary-60: #0A7AEF;
  --primary-70: #0970DF;
  
  --success-50: #32D74B;
  --success-60: #2DC945;
  --success-bg: rgba(50, 215, 75, 0.12);
  
  --warning-50: #FF9F0A;
  --warning-60: #EF9500;
  --warning-bg: rgba(255, 159, 10, 0.12);
  
  --error-50: #FF453A;
  --error-60: #EF3A30;
  --error-bg: rgba(255, 69, 58, 0.12);
  
  --oncall-50: #BF5AF2;
  --oncall-60: #B550E2;
  --oncall-bg: rgba(191, 90, 242, 0.12);
  
  --unallocated-50: #FF6B6B;
  --unallocated-bg: rgba(255, 107, 107, 0.10);
  
  /* Colors - Projects */
  --project-blue: #64D2FF;
  --project-green: #63E6BE;
  --project-yellow: #FFD43B;
  --project-orange: #FFA94D;
  --project-red: #FF8787;
  --project-purple: #CC5DE8;
  --project-pink: #F783AC;
  --project-teal: #4FD1C5;
  --project-indigo: #748FFC;
  
  /* Spacing */
  --space-xs: 4px;
  --space-sm: 8px;
  --space-md: 16px;
  --space-lg: 24px;
  --space-xl: 32px;
  --space-2xl: 48px;
  
  /* Typography */
  --font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Segoe UI', system-ui, sans-serif;
  --font-mono: 'SF Mono', 'Consolas', monospace;
  
  --font-size-display: 28px;
  --font-size-h1: 20px;
  --font-size-h2: 16px;
  --font-size-body: 14px;
  --font-size-caption: 12px;
  --font-size-small: 11px;
  
  /* Transitions */
  --transition-quick: 150ms cubic-bezier(0.4, 0, 0.2, 1);
  --transition-base: 250ms cubic-bezier(0.4, 0, 0.2, 1);
  --transition-slow: 400ms cubic-bezier(0.4, 0, 0.2, 1);
  --spring: cubic-bezier(0.34, 1.56, 0.64, 1);
  
  /* Shadows */
  --shadow-sm: 0 1px 3px rgba(0, 0, 0, 0.4);
  --shadow-md: 0 4px 16px rgba(0, 0, 0, 0.5);
  --shadow-lg: 0 8px 24px rgba(0, 0, 0, 0.6);
  --shadow-xl: 0 16px 48px rgba(0, 0, 0, 0.6);
  
  /* Border Radius */
  --radius-sm: 4px;
  --radius-md: 6px;
  --radius-lg: 8px;
  --radius-xl: 12px;
  --radius-full: 9999px;
  
  /* Grid */
  --grid-cell-width: 140px;
  --grid-cell-height: 40px;
  --grid-header-height: 48px;
}
```

### 12.2 Component Classes

```css
/* Button */
.btn-primary {
  background: var(--primary-50);
  color: white;
  height: 36px;
  padding: 0 var(--space-md);
  border-radius: var(--radius-md);
  font-weight: 500;
  box-shadow: var(--shadow-sm);
  transition: all var(--transition-quick);
}

.btn-primary:hover {
  background: var(--primary-60);
  box-shadow: var(--shadow-md);
}

/* Grid Cell */
.grid-cell {
  width: var(--grid-cell-width);
  height: var(--grid-cell-height);
  background: var(--bg-tertiary);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: var(--font-size-body);
  color: var(--text-primary);
  transition: all var(--transition-quick);
}

.grid-cell.empty {
  border-style: dashed;
  color: var(--text-tertiary);
}

.grid-cell.allocated {
  border-style: solid;
}

.grid-cell:hover {
  border-color: var(--border-emphasis);
}

/* Status Badge */
.status-badge {
  display: inline-flex;
  align-items: center;
  gap: var(--space-xs);
  height: 22px;
  padding: 4px var(--space-sm);
  border-radius: var(--radius-sm);
  font-size: var(--font-size-caption);
  font-family: var(--font-mono);
}

.status-badge.success {
  background: var(--success-bg);
  color: var(--success-50);
}

.status-badge.warning {
  background: var(--warning-bg);
  color: var(--warning-50);
}

.status-badge.error {
  background: var(--error-bg);
  color: var(--error-50);
}
```

### 12.3 Icon Library

**Recommended: Lucide Icons or SF Symbols**

Required icons:
```
Actions:
├─ plus (add)
├─ save (floppy disk)
├─ folder-open (open file)
├─ upload (import)
├─ download (export)
├─ settings (preferences)
├─ menu (hamburger)
├─ x (close)
├─ check (success)
├─ alert-triangle (warning)
├─ alert-circle (error)
└─ info (information)

Navigation:
├─ chevron-down (dropdown)
├─ chevron-right (expand)
├─ arrow-left (back)
└─ external-link (open in new)

Grid:
├─ paintbrush (paintbrush mode)
├─ split (split allocation)
├─ copy (duplicate)
├─ trash (delete)
└─ phone (oncall)

Status:
├─ circle (dot indicator)
├─ circle-check (success)
├─ circle-alert (warning)
└─ circle-x (error)
```

### 12.4 Wireframe Mockups

I'll create high-fidelity mockups for:
1. Roadmap View (full page)
2. Technical Projects View with side panel
3. Allocation Grid (primary view)
4. Grid cell states (all variants)
5. Modal dialogs (split allocation, preferences)
6. Dropdown menus
7. Tooltips

Would you like me to generate these as actual image mockups, or proceed with implementation-ready specifications?

---

## 13. Implementation Notes for Claude Code

### 13.1 Technology Stack Recommendation

```
Framework: React 18+ with TypeScript
├─ State Management: Zustand or Context API
├─ Styling: Tailwind CSS (with custom theme)
├─ UI Components: Radix UI (headless components)
├─ Drag & Drop: @dnd-kit
├─ Data Grid: TanStack Table or custom
├─ File I/O: File System Access API (web) / Electron fs (desktop)
└─ Build: Vite

Desktop Wrapper: Electron (optional)
├─ electron-builder for packaging
└─ electron-store for preferences
```

### 13.2 File Structure

```
src/
├─ components/
│  ├─ Layout/
│  │  ├─ TopNav.tsx
│  │  └─ SidePanel.tsx
│  ├─ Views/
│  │  ├─ RoadmapView.tsx
│  │  ├─ TechnicalProjectsView.tsx
│  │  └─ AllocationGrid/
│  │     ├─ Grid.tsx
│  │     ├─ GridCell.tsx
│  │     ├─ GridHeader.tsx
│  │     └─ PaintbrushMode.tsx
│  ├─ Tables/
│  │  ├─ ProjectTable.tsx
│  │  └─ ProjectRow.tsx
│  ├─ Modals/
│  │  ├─ SplitAllocation.tsx
│  │  └─ Preferences.tsx
│  └─ UI/
│     ├─ Button.tsx
│     ├─ Badge.tsx
│     ├─ Dropdown.tsx
│     └─ Tooltip.tsx
├─ hooks/
│  ├─ useFileManagement.ts
│  ├─ useAllocation.ts
│  └─ useCapacityCalculation.ts
├─ stores/
│  ├─ planStore.ts
│  └─ uiStore.ts
├─ types/
│  ├─ plan.types.ts
│  └─ allocation.types.ts
├─ utils/
│  ├─ dateHelpers.ts
│  ├─ capacityCalculations.ts
│  └─ validators.ts
└─ styles/
   ├─ globals.css
   └─ theme.css
```

### 13.3 Key Implementation Details

**Allocation Grid Performance:**
- Virtualize rows if >20 weeks visible
- Memoize cell components with React.memo
- Use CSS Grid for layout (not table)
- Debounce drag operations

**State Management:**
```typescript
// Core data structure
interface PlanState {
  quarter: string;
  engineers: Engineer[];
  roadmapProjects: RoadmapProject[];
  technicalProjects: TechnicalProject[];
  allocations: Map<string, Allocation>; // key: "engineerId-weekDate"
  
  // Actions
  addAllocation: (allocation: Allocation) => void;
  updateAllocation: (key: string, allocation: Partial<Allocation>) => void;
  removeAllocation: (key: string) => void;
  
  // Computed
  getProjectAllocated: (projectId: string) => number;
  getEngineerUtilization: (engineerId: string) => number;
}
```

**Paintbrush Mode Logic:**
```typescript
const [paintbrushActive, setPaintbrushActive] = useState(false);
const [selectedProject, setSelectedProject] = useState<string | null>(null);
const [paintStart, setPaintStart] = useState<GridCell | null>(null);

function handleCellMouseDown(cell: GridCell) {
  if (!paintbrushActive || !selectedProject) return;
  setPaintStart(cell);
}

function handleCellMouseEnter(cell: GridCell) {
  if (!paintStart) return;
  // Show preview
  previewAllocation(paintStart, cell, selectedProject);
}

function handleMouseUp() {
  if (!paintStart) return;
  // Commit allocation
  commitAllocation(paintStart, currentCell, selectedProject);
  setPaintStart(null);
}
```

