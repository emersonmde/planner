# Quarterly Planner - Component Reference Guide

This document provides individual component examples for implementation in Dioxus.
Each component includes HTML structure, styling, and behavioral notes.

---

## 1. TOP NAVIGATION COMPONENT

### Structure
```html
<nav class="top-nav">
    <div class="app-title">
        <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="14" height="14" rx="2"/>
            <line x1="3" y1="8" x2="17" y2="8"/>
            <line x1="8" y1="3" x2="8" y2="17"/>
        </svg>
        Quarterly Planner
    </div>
    
    <select class="quarter-selector">
        <option>Q1 2025</option>
        <option>Q2 2025</option>
    </select>
    
    <div class="view-tabs">
        <button class="view-tab active">Roadmap</button>
        <button class="view-tab">Technical</button>
        <button class="view-tab">Allocation</button>
    </div>
    
    <div class="capacity-indicator">
        <span class="capacity-text">67.5 / 78 weeks</span>
        <div class="capacity-bar">
            <div class="capacity-bar-fill success" style="width: 87%"></div>
        </div>
    </div>
    
    <button class="file-menu-btn">☰</button>
</nav>
```

### Styling
```css
.top-nav {
    height: 56px;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-default);
    display: flex;
    align-items: center;
    padding: 0 var(--space-lg);
    gap: var(--space-lg);
    position: sticky;
    top: 0;
    z-index: 100;
}
```

### Dioxus Notes
- Use `rsx!` macro with flexbox layout
- Quarter selector should trigger state update
- View tabs should use conditional CSS class for active state
- Capacity bar width should be reactive based on allocation state
- File menu button triggers dropdown modal

---

## 2. STATUS BADGE COMPONENT

### Variants

#### Success Badge
```html
<span class="status-badge success">
    <svg class="status-icon" viewBox="0 0 12 12">
        <polyline points="2,6 5,9 10,3" fill="none" stroke="currentColor" stroke-width="2"/>
    </svg>
    8
</span>
```

#### Warning Badge
```html
<span class="status-badge warning">
    <svg class="status-icon" viewBox="0 0 12 12">
        <path d="M6,2 L10,10 L2,10 Z" fill="none" stroke="currentColor" stroke-width="2"/>
        <line x1="6" y1="5" x2="6" y2="7" stroke="currentColor" stroke-width="2"/>
    </svg>
    26
</span>
```

#### Error Badge
```html
<span class="status-badge error">
    <svg class="status-icon" viewBox="0 0 12 12">
        <line x1="6" y1="3" x2="6" y2="7" stroke="currentColor" stroke-width="2"/>
        <circle cx="6" cy="9" r="0.5" fill="currentColor"/>
    </svg>
    18
</span>
```

### Styling
```css
.status-badge {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    height: 22px;
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 12px;
    font-family: monospace;
}

.status-badge.success {
    background: rgba(50, 215, 75, 0.12);
    color: #32D74B;
}

.status-badge.warning {
    background: rgba(255, 159, 10, 0.12);
    color: #FF9F0A;
}

.status-badge.error {
    background: rgba(255, 69, 58, 0.12);
    color: #FF453A;
}
```

### Dioxus Notes
- Create enum: `BadgeType { Success, Warning, Error }`
- Calculate type based on allocated vs estimated weeks
- SVG icons can be components or inline
- Number should be reactive

---

## 3. BUTTON COMPONENTS

### Primary Button
```html
<button class="btn btn-primary">
    <svg width="16" height="16" viewBox="0 0 16 16">
        <line x1="8" y1="3" x2="8" y2="13" stroke="currentColor" stroke-width="2"/>
        <line x1="3" y1="8" x2="13" y2="8" stroke="currentColor" stroke-width="2"/>
    </svg>
    Add Project
</button>
```

### Secondary Button
```html
<button class="btn btn-secondary">
    Cancel
</button>
```

### Icon Button
```html
<button class="file-menu-btn">
    <svg width="20" height="20" viewBox="0 0 20 20">
        <line x1="3" y1="6" x2="17" y2="6" stroke="currentColor" stroke-width="2"/>
        <line x1="3" y1="10" x2="17" y2="10" stroke="currentColor" stroke-width="2"/>
        <line x1="3" y1="14" x2="17" y2="14" stroke="currentColor" stroke-width="2"/>
    </svg>
</button>
```

### Styling
```css
.btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    height: 36px;
    padding: 0 16px;
    border-radius: 6px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    border: none;
    transition: all 150ms cubic-bezier(0.4, 0, 0.2, 1);
}

.btn-primary {
    background: #0A84FF;
    color: white;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.4);
}

.btn-primary:hover {
    background: #0A7AEF;
}

.btn-primary:active {
    transform: scale(0.98);
}
```

### Dioxus Notes
- Use `onclick` event handler
- Optional icon prop (left or right)
- Disabled state should change opacity and cursor
- Active state uses transform scale

---

## 4. DATA TABLE COMPONENT

### Structure
```html
<div class="data-table">
    <div class="table-header">
        <div class="table-header-cell">Project Name</div>
        <div class="table-header-cell">Eng Est.</div>
        <div class="table-header-cell">Total Est.</div>
        <!-- More headers... -->
    </div>

    <div class="table-row">
        <div class="table-cell emphasis">
            <div class="project-name">
                <span class="project-dot" style="background: #64D2FF"></span>
                Q1 Platform Roadmap
            </div>
        </div>
        <div class="table-cell monospace secondary">24</div>
        <div class="table-cell">
            <span class="status-badge error">18</span>
        </div>
        <!-- More cells... -->
    </div>
</div>
```

### Styling
```css
.data-table {
    background: var(--bg-secondary);
    border-radius: 8px;
    overflow: hidden;
}

.table-header {
    display: grid;
    grid-template-columns: 300px 100px 100px 120px 1fr;
    background: var(--bg-tertiary);
    padding: 16px;
    gap: 16px;
}

.table-row {
    display: grid;
    grid-template-columns: 300px 100px 100px 120px 1fr;
    padding: 16px;
    gap: 16px;
    border-top: 1px solid var(--border-subtle);
    transition: background 150ms;
}

.table-row:hover {
    background: var(--bg-tertiary);
}

.project-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    border: 1px solid rgba(255, 255, 255, 0.2);
}
```

### Dioxus Notes
- Use CSS Grid for column layout
- Map over projects array to generate rows
- Sort and filter should update the mapped data
- Click row to select/edit
- Project dot color comes from project.color field

---

## 5. ALLOCATION GRID CELL COMPONENT

### Empty Cell
```html
<div class="grid-cell empty">
    <!-- Shows + icon via ::before pseudo-element -->
</div>
```

### Allocated Cell
```html
<div class="grid-cell allocated project-blue">
    <div class="cell-content">
        <span class="project-text">Payment API</span>
        <span class="allocation-badge">100%</span>
    </div>
</div>
```

### Split Allocation Cell
```html
<div class="grid-cell split">
    <div class="split-section" style="background: rgba(99, 230, 190, 0.15);">
        <span class="split-text">Pay API</span>
        <span class="split-percentage">60%</span>
    </div>
    <div class="split-section" style="background: rgba(255, 169, 77, 0.15);">
        <span class="split-text">Data</span>
        <span class="split-percentage">40%</span>
    </div>
</div>
```

### Oncall Cell
```html
<div class="grid-cell oncall">
    <div class="oncall-content">
        <svg class="oncall-icon" viewBox="0 0 16 16">
            <path d="M3,3 L5,1 L7,3 M5,1 L5,6 M13,13 L11,15 L9,13 M11,15 L11,10" stroke="currentColor" stroke-width="2" fill="none"/>
            <rect x="5" y="6" width="6" height="4" rx="1" stroke="currentColor" stroke-width="2" fill="none"/>
        </svg>
        <span class="oncall-text">Oncall</span>
    </div>
</div>
```

### Unallocated Cell
```html
<div class="grid-cell unallocated">
    <!-- Shows dashed lines via ::after pseudo-element -->
</div>
```

### Before Start Date Cell
```html
<div class="grid-cell allocated project-yellow before-start">
    <svg class="warning-icon" viewBox="0 0 12 12">
        <path d="M6,2 L10,10 L2,10 Z" fill="none" stroke="currentColor" stroke-width="2"/>
        <line x1="6" y1="5" x2="6" y2="7" stroke="currentColor" stroke-width="2"/>
    </svg>
    <div class="cell-content">
        <span class="project-text">New Feature</span>
        <span class="allocation-badge">100%</span>
    </div>
</div>
```

### Styling
```css
.grid-cell {
    width: 140px;
    height: 40px;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-subtle);
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 150ms;
    cursor: pointer;
    position: relative;
    overflow: hidden;
}

.grid-cell.empty {
    border-style: dashed;
}

.grid-cell.empty::before {
    content: '+';
    font-size: 20px;
    color: var(--text-tertiary);
    opacity: 0.3;
}

.grid-cell.project-blue {
    background: rgba(100, 210, 255, 0.15);
    border-color: rgba(100, 210, 255, 0.4);
}

.grid-cell.oncall {
    background: rgba(191, 90, 242, 0.12);
    border: 2px solid #BF5AF2;
}

.grid-cell.oncall::before {
    content: '';
    position: absolute;
    inset: 0;
    background: repeating-linear-gradient(
        45deg,
        transparent,
        transparent 10px,
        rgba(191, 90, 242, 0.05) 10px,
        rgba(191, 90, 242, 0.05) 20px
    );
}

.grid-cell.before-start {
    opacity: 0.6;
}

.grid-cell.before-start::after {
    content: '';
    position: absolute;
    inset: 0;
    background: repeating-linear-gradient(
        45deg,
        transparent,
        transparent 3px,
        var(--border-subtle) 3px,
        var(--border-subtle) 4px
    );
}
```

### Dioxus Notes
- Cell state enum: `Empty | Allocated | Split | Oncall | Unallocated`
- Click handler should open project selector or split dialog
- Hover shows tooltip with project details
- Drag start/end for paintbrush mode
- Project color comes from lookup table
- Split cells need nested divs with percentage calculation

---

## 6. GRID HEADER COMPONENTS

### Sprint/Week Header
```html
<div class="sprint-cell" style="grid-row: span 2;">1</div>
<div class="week-cell">
    <span class="week-date">Jan 3</span>
    <span class="week-day">(W)</span>
    <span class="week-number">Wk 1/13</span>
</div>
```

### Engineer Header
```html
<div class="grid-header-cell">
    <span class="engineer-name">Alice K</span>
    <span class="role-badge">Eng</span>
    <span class="capacity-text-small">11.5 / 12</span>
</div>
```

### Styling
```css
.sprint-cell {
    background: var(--bg-tertiary);
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 20px;
    font-weight: 600;
}

.week-cell {
    background: var(--bg-tertiary);
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 8px;
    gap: 2px;
}

.grid-header-cell {
    height: 48px;
    background: var(--bg-tertiary);
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 8px;
    gap: 4px;
}

.role-badge {
    padding: 2px 6px;
    background: var(--bg-overlay);
    border-radius: 4px;
    font-size: 11px;
    color: var(--text-secondary);
}

.capacity-text-small {
    font-family: monospace;
    font-size: 11px;
    color: var(--success-50);
}
```

### Dioxus Notes
- Week dates calculated from quarter start + sprint config
- Sprint cells use rowspan (grid-row: span 2)
- Engineer capacity color changes based on utilization
- Tooltip on hover shows detailed capacity breakdown

---

## 7. PAINTBRUSH MODE TOGGLE

### Structure
```html
<button class="paintbrush-toggle">
    <svg width="16" height="16" viewBox="0 0 16 16">
        <path d="M3,13 Q3,10 6,8 L12,2 L14,4 L8,10 Q6,13 3,13Z" fill="none" stroke="currentColor" stroke-width="2"/>
        <circle cx="13" cy="3" r="1.5" fill="currentColor"/>
    </svg>
    <span class="toggle-label">Paintbrush Mode: OFF</span>
</button>

<select class="project-selector">
    <option>Select a project...</option>
    <option>Auth Service Upgrade</option>
    <option>Payment API</option>
    <option>ML Pipeline</option>
    <option>Oncall</option>
</select>
```

### Styling
```css
.paintbrush-toggle {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-default);
    border-radius: 6px;
    cursor: pointer;
    transition: all 150ms;
}

.paintbrush-toggle:hover {
    background: var(--bg-overlay);
}

.paintbrush-toggle.active {
    background: var(--primary-50);
    border-color: var(--primary-50);
    color: white;
}

.project-selector {
    width: 240px;
    padding: 8px 16px;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-default);
    border-radius: 6px;
    color: var(--text-primary);
}
```

### Dioxus Notes
- State: `paintbrush_active: bool`
- State: `selected_project: Option<ProjectId>`
- Toggle changes cursor style globally
- Project selector enables when paintbrush active
- Cell click behavior changes based on mode

---

## 8. TOOLTIP COMPONENT

### Structure
```html
<div class="tooltip visible" style="left: 200px; top: 100px;">
    <div class="tooltip-title">Payment API Integration</div>
    <div class="tooltip-subtitle">Linked to: Payment Gateway ↗</div>
    <div class="tooltip-row">
        <span>Allocation:</span>
        <span>100% of week</span>
    </div>
    <div class="tooltip-divider"></div>
    <div class="tooltip-row">
        <span>Project Progress:</span>
        <span>4.5 / 8 weeks</span>
    </div>
    <div class="tooltip-row">
        <span>Status:</span>
        <span style="color: var(--warning-50)">Needs allocation ⚠</span>
    </div>
    <div class="tooltip-hint">Click to edit • Right-click for options</div>
</div>
```

### Styling
```css
.tooltip {
    position: fixed;
    background: var(--bg-overlay);
    backdrop-filter: blur(20px);
    border: 1px solid var(--border-emphasis);
    border-radius: 6px;
    padding: 12px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.6);
    z-index: 1000;
    max-width: 280px;
    pointer-events: none;
    opacity: 0;
    transition: opacity 150ms;
}

.tooltip.visible {
    opacity: 1;
}

.tooltip-title {
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 4px;
}

.tooltip-subtitle {
    font-size: 12px;
    color: var(--primary-50);
    margin-bottom: 8px;
}

.tooltip-divider {
    height: 1px;
    background: var(--border-subtle);
    margin: 8px 0;
}

.tooltip-row {
    display: flex;
    justify-content: space-between;
    font-size: 12px;
    color: var(--text-secondary);
    margin-bottom: 4px;
}

.tooltip-hint {
    font-size: 11px;
    color: var(--text-tertiary);
    margin-top: 8px;
}
```

### Dioxus Notes
- Portal/overlay component for positioning
- Show after 300ms hover delay
- Hide on mouse leave (100ms delay)
- Position near hovered element
- Content varies by element type (cell, header, badge)

---

## 9. SIDE PANEL COMPONENT (Technical View)

### Structure
```html
<aside class="side-panel">
    <div class="side-panel-section">
        <div class="section-title">Filters</div>
        <div class="filter-option">
            <div class="checkbox checked"></div>
            <span>All Projects</span>
        </div>
        <div class="filter-option">
            <div class="checkbox"></div>
            <span>On Track</span>
        </div>
        <div class="filter-option">
            <div class="checkbox"></div>
            <span>At Risk</span>
        </div>
    </div>

    <div class="side-panel-section">
        <div class="section-title">Sort By</div>
        <div class="filter-option">
            <div class="radio checked"></div>
            <span>Roadmap Project</span>
        </div>
        <div class="filter-option">
            <div class="radio"></div>
            <span>Status</span>
        </div>
    </div>
</aside>
```

### Styling
```css
.side-panel {
    width: 320px;
    background: var(--bg-primary);
    border-right: 1px solid var(--border-default);
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 24px;
}

.side-panel-section {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.section-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

.filter-option {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px;
    cursor: pointer;
    border-radius: 6px;
    transition: background 150ms;
}

.filter-option:hover {
    background: var(--bg-tertiary);
}

.checkbox {
    width: 16px;
    height: 16px;
    border-radius: 4px;
    border: 1px solid var(--border-default);
    background: var(--bg-tertiary);
}

.checkbox.checked {
    background: var(--primary-50);
    border-color: var(--primary-50);
}
```

### Dioxus Notes
- State: `filters: HashSet<FilterType>`
- State: `sort_by: SortOption`
- Checkboxes toggle filter set
- Radio buttons set sort option
- Changes trigger table re-render

---

## 10. SEARCH INPUT COMPONENT

### Structure
```html
<input 
    type="text" 
    class="search-input" 
    placeholder="Search projects..."
>
```

### Styling
```css
.search-input {
    width: 300px;
    height: 36px;
    padding: 0 16px;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-default);
    border-radius: 6px;
    color: var(--text-primary);
    font-size: 14px;
    transition: all 150ms;
}

.search-input:focus {
    outline: none;
    border-color: var(--primary-50);
    box-shadow: 0 0 0 3px rgba(10, 132, 255, 0.1);
}

.search-input::placeholder {
    color: var(--text-tertiary);
}
```

### Dioxus Notes
- Use `oninput` for reactive filtering
- Debounce input (300ms) for performance
- Clear button appears when text present
- Escape key clears input

---

## 11. PROJECT COLOR SYSTEM

### Color Mapping
```rust
pub enum ProjectColor {
    Blue,
    Green,
    Yellow,
    Orange,
    Red,
    Purple,
    Pink,
    Teal,
    Indigo,
}

impl ProjectColor {
    pub fn to_css_class(&self) -> &'static str {
        match self {
            Self::Blue => "project-blue",
            Self::Green => "project-green",
            Self::Yellow => "project-yellow",
            Self::Orange => "project-orange",
            Self::Red => "project-red",
            Self::Purple => "project-purple",
            Self::Pink => "project-pink",
            Self::Teal => "project-teal",
            Self::Indigo => "project-indigo",
        }
    }
    
    pub fn to_hex(&self) -> &'static str {
        match self {
            Self::Blue => "#64D2FF",
            Self::Green => "#63E6BE",
            Self::Yellow => "#FFD43B",
            Self::Orange => "#FFA94D",
            Self::Red => "#FF8787",
            Self::Purple => "#CC5DE8",
            Self::Pink => "#F783AC",
            Self::Teal => "#4FD1C5",
            Self::Indigo => "#748FFC",
        }
    }
}
```

### CSS Color Classes
```css
.project-blue {
    background: rgba(100, 210, 255, 0.15);
    border-color: rgba(100, 210, 255, 0.4);
}

.project-green {
    background: rgba(99, 230, 190, 0.15);
    border-color: rgba(99, 230, 190, 0.4);
}

/* ...etc for all colors */
```

---

## 12. GRID LAYOUT STRUCTURE

### Full Grid Layout
```html
<div class="allocation-grid">
    <div class="grid-container">
        <!-- Header Row -->
        <div class="grid-header-row">
            <div class="grid-header-cell" style="grid-column: 1 / 3;">Sprint / Week</div>
            <div class="grid-header-cell">Engineer 1</div>
            <div class="grid-header-cell">Engineer 2</div>
            <!-- ...more engineers -->
        </div>

        <!-- Sprint 1, Week 1 -->
        <div class="grid-row">
            <div class="sprint-cell" style="grid-row: span 2;">1</div>
            <div class="week-cell">Jan 3 (W)</div>
            <div class="grid-cell"><!-- cell content --></div>
            <!-- ...more cells -->
        </div>

        <!-- Sprint 1, Week 2 -->
        <div class="grid-row">
            <div class="week-cell">Jan 10</div>
            <div class="grid-cell"><!-- cell content --></div>
            <!-- ...more cells -->
        </div>

        <!-- Sprint 2 (with separator) -->
        <div class="grid-row sprint-separator">
            <div class="sprint-cell" style="grid-row: span 2;">2</div>
            <!-- ...rest of row -->
        </div>
    </div>
</div>
```

### Grid CSS
```css
.grid-container {
    display: grid;
    gap: 8px;
    min-width: fit-content;
}

.grid-header-row {
    display: grid;
    grid-template-columns: 60px 120px repeat(6, 140px);
    gap: 8px;
}

.grid-row {
    display: grid;
    grid-template-columns: 60px 120px repeat(6, 140px);
    gap: 8px;
}

.grid-row.sprint-separator {
    margin-top: 16px;
    padding-top: 16px;
    border-top: 2px dashed var(--border-default);
}
```

### Dioxus Notes
- Calculate grid columns dynamically: `60px 120px repeat({engineers.len()}, 140px)`
- Sprint cells use `grid-row: span 2` for rowspan effect
- Week dates calculated from quarter start
- Sprint separators every 2 weeks (configurable)
- Horizontal scroll container if many engineers

---

## IMPLEMENTATION CHECKLIST FOR DIOXUS

### Phase 1: Foundation
- [ ] Set up Dioxus project with CSS
- [ ] Implement design tokens as CSS variables
- [ ] Create layout structure (TopNav + MainContent)
- [ ] Build view switching logic

### Phase 2: Core Components
- [ ] Button component (primary, secondary, icon)
- [ ] Badge component (success, warning, error)
- [ ] Input component (search, text)
- [ ] Dropdown/Select component

### Phase 3: Data Display
- [ ] Data table component
- [ ] Table row component with hover
- [ ] Status indicators
- [ ] Project dot component

### Phase 4: Allocation Grid
- [ ] Grid container with dynamic columns
- [ ] Grid cell component (all variants)
- [ ] Sprint/week headers
- [ ] Engineer headers with capacity

### Phase 5: Interactions
- [ ] Paintbrush mode toggle
- [ ] Click-to-allocate
- [ ] Drag-to-paint
- [ ] Split allocation dialog
- [ ] Tooltip system

### Phase 6: State Management
- [ ] Project store
- [ ] Allocation store
- [ ] Capacity calculations
- [ ] Validation logic

### Phase 7: File Operations
- [ ] Save to JSON
- [ ] Load from JSON
- [ ] Export to CSV
- [ ] Import from CSV

### Phase 8: Polish
- [ ] Hover states
- [ ] Transitions/animations
- [ ] Keyboard shortcuts
- [ ] Accessibility (ARIA labels)

---

## RESPONSIVE CONSIDERATIONS

### Breakpoint Behavior
- Desktop (1440px+): Full layout, all features
- Laptop (1024-1439px): Collapsible side panel, smaller cells
- Tablet (768-1023px): Side panel as overlay, simplified tables
- Mobile (<768px): Read-only mode recommended

### Horizontal Scroll for Many Engineers
```css
.allocation-grid {
    overflow-x: auto;
}

.grid-container {
    min-width: fit-content;
}
```

When >8 engineers:
- Freeze left columns (Sprint, Week) using `position: sticky`
- Add scroll position indicator
- Keyboard shortcuts: Shift + Arrow keys to scroll

---

## PERFORMANCE NOTES

### Virtualization
For grids with >20 weeks (>40 rows), consider:
- Virtual scrolling (only render visible rows)
- Memoization of cell components
- Debounced drag operations
- Lazy loading of project details

### State Updates
- Batch allocation updates
- Use immutable data structures
- Compute derived values (capacity, utilization) on demand
- Cache tooltip content

### CSS Performance
- Use `transform` for animations (GPU accelerated)
- Avoid layout thrashing
- Use `will-change` sparingly
- Minimize box-shadow complexity

---

This component reference provides everything needed to implement the Quarterly Planner in Dioxus while maintaining the exact look and feel of the HTML mockup.
