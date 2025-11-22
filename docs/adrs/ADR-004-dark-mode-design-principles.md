# ADR-004: Dark Mode Interface Design Principles

**Status:** Accepted
**Date:** 2025-11-22
**Decision Makers:** Design Team
**Tags:** #design-system #dark-mode #color-theory #accessibility

---

## Context

Dark mode interfaces have become a standard expectation in modern applications, particularly for tools used by technical professionals who often work in low-light environments or prefer reduced eye strain during extended sessions. However, implementing dark mode is not simply inverting colors or using dark backgrounds—it requires careful consideration of visual hierarchy, contrast, accessibility, and aesthetic coherence.

This ADR establishes comprehensive principles for designing sophisticated dark mode interfaces with a **modern SaaS aesthetic** (as seen in Asana, Notion, M365), drawing from industry best practices including Apple's seminal WWDC 2019 presentation on dark mode design, Material Design guidelines, and established color theory principles.

**Scope:** This document provides general, reusable principles applicable to any project seeking a sophisticated, layered dark mode interface with vibrant but refined "candy bright" accents. While examples reference this project's implementation, the principles are designed to be portable.

---

## Decision

We adopt a **layered elevation system** with **subtle background progressions** combined with **candy bright accent colors** (60-85% saturation) to create interfaces that are:

1. **Visually sophisticated** through subtle material layering
2. **Aesthetically striking** through strategic use of vibrant accents
3. **Functionally clear** through careful contrast management
4. **Accessible** through WCAG 2.1 AA compliance minimum
5. **Consistent** across browsers and display technologies
6. **Refined and premium** through moderate saturation (avoiding harsh pure neon)

---

## Core Principles

### Principle 1: Elevation Through Subtle Progression

**The Problem with Pure Black:**
Pure black backgrounds (#000000) create several issues:
- **Visual fatigue**: Extreme contrast between pure white text and pure black causes eye strain
- **Loss of depth**: No ability to create layered surfaces or elevation hierarchy
- **Harsh aesthetic**: Feels flat and unpolished
- **Reduced accessibility**: Maximum contrast isn't always optimal contrast

**The Solution:**
Use a carefully calibrated progression of dark grays that increase in lightness with elevation. Each "surface" in your interface sits at a different elevation level.

**Implementation Guidelines:**

```css
/* Example: 4-level elevation system */
--bg-primary: #0f0f11;     /* Base canvas - darkest */
--bg-secondary: #18181a;   /* Lifted surfaces (+9 points) */
--bg-tertiary: #212125;    /* Cards/panels (+18 points) */
--bg-overlay: #2a2a2e;     /* Modals/menus (+27 points) */
```

**Key Metrics:**
- **Minimum increment:** 8-10 hex points between levels (e.g., #0f → #18 = +9)
- **Maximum levels:** 4-5 elevation levels (more creates visual confusion)
- **Progression pattern:** Can be linear or slightly accelerating
- **Base darkness:** #0a-#12 range provides optimal foundation

**Why This Works:**
- Creates **implicit hierarchy** without explicit borders
- Enables **floating UI elements** (modals, dropdowns) to feel elevated
- Maintains **visual interest** through subtle variation
- Prevents **monotony** in large interface areas

**Reference:** Apple's WWDC 2019 "Implementing Dark Mode on iOS" explicitly recommends against pure black (#000000) and demonstrates multi-level gray systems.

---

### Principle 2: Candy Bright Accents, Not Neon Everything

**The Modern Dark Mode Aesthetic:**
The modern dark mode aesthetic (as seen in Asana, Notion, M365) uses **candy bright colors**—vibrant and punchy but with moderate saturation (60-85%) and higher luminosity. This creates energetic visual interest without the harshness of pure neon (100% saturation).

**Why Candy Bright Over Pure Neon:**
- **Less harsh** on the eyes during extended use
- **Better readability** when used at low opacity (backgrounds)
- **More refined** and premium feeling (SaaS aesthetic vs gamer RGB)
- **Superior distinction** between colors at 25% opacity
- **Accessible** text contrast even on colored backgrounds

**What Gets Candy Bright Treatment:**
- ✅ **Accent colors**: Primary actions, key UI elements
- ✅ **Data visualization**: Charts, graphs, project colors
- ✅ **Interactive elements**: Hover states, focus indicators
- ✅ **Status indicators**: Success/warning/error states
- ✅ **Content highlights**: Selected items, active states

**What Stays Subtle:**
- ❌ **Backgrounds**: Keep sophisticated gray progressions
- ❌ **Body text**: Pure white or high-alpha white
- ❌ **Borders**: Subtle, translucent whites/grays
- ❌ **Secondary text**: 50-75% white opacity
- ❌ **Disabled states**: Low-alpha grays

**Implementation:**

```css
/* Candy bright accent palette - moderate saturation with high luminosity */
--primary-50: #00B8FF;      /* Electric cyan-blue */
--success-50: #00FF41;      /* Matrix green */
--warning-50: #FFB800;      /* Amber alert */
--error-50: #FF0055;        /* Hot red */

/* Background system - sophisticated grays */
--bg-primary: #0f0f11;      /* NOT colored */
--bg-secondary: #18181a;    /* NOT colored */
```

**Saturation Guidelines:**
- **Candy bright accents:** 60-85% saturation (vibrant but refined)
- **Pure neon semantic colors:** 90-100% saturation (success/error states only)
- **Backgrounds:** 0-10% saturation (near-grayscale)
- **Borders:** 0% saturation (pure grayscale with alpha)

---

### Principle 3: Text Hierarchy Through Opacity

In dark mode, text hierarchy is achieved through **white at varying opacities** rather than different shades of gray. This approach is more flexible and maintains consistency across different background levels.

**Standard Text Hierarchy:**

```css
--text-primary: #FFFFFF;                      /* 100% - Headlines, key content */
--text-secondary: rgba(255, 255, 255, 0.75);  /* 75% - Body text, labels */
--text-tertiary: rgba(255, 255, 255, 0.50);   /* 50% - Metadata, captions */
--text-disabled: rgba(255, 255, 255, 0.30);   /* 30% - Disabled states */
```

**Why Opacity Over Fixed Colors:**
- **Adapts to background:** Text automatically adjusts when background changes
- **Consistent hierarchy:** Same semantic meaning across all surfaces
- **Simpler system:** Fewer color tokens to manage
- **Better blending:** Natural anti-aliasing at edges

**Exceptions:**
- **Neon text:** Use solid hex colors for accent text (e.g., success messages)
- **Monospace code:** May use slightly higher opacity for readability
- **Small text:** May need +5-10% opacity boost for legibility

**Minimum Contrast Ratios (WCAG 2.1 AA):**
- **Large text (18pt+):** 3:1 minimum
- **Normal text:** 4.5:1 minimum
- **Interactive elements:** 3:1 minimum

---

### Principle 4: Borders as Subtle Guides

Borders in dark mode should be **barely perceptible guides**, not strong dividers. Overuse of borders creates visual clutter and competes with elevation hierarchy.

**Border System:**

```css
--border-subtle: rgba(255, 255, 255, 0.08);    /* Hairline separators */
--border-default: rgba(255, 255, 255, 0.12);   /* Standard borders */
--border-emphasis: rgba(255, 255, 255, 0.18);  /* Strong separation */
```

**Usage Guidelines:**
- **Prefer elevation over borders:** Use background progression first
- **Translucent only:** Never use solid gray borders
- **Consistent opacity:** Use semantic tokens, not arbitrary values
- **Neon borders for accents:** Use full-saturation colors for interactive elements

**When to Use Each:**
- `border-subtle`: Table rows, card internal divisions, subtle separators
- `border-default`: Input fields (default state), card containers, panels
- `border-emphasis`: Sticky headers, strong section breaks, dragged items

**Anti-pattern:**
Avoid thick (2px+) solid borders in neutral gray—they create harsh lines that break visual flow.

---

### Principle 5: Strategic Color Saturation

Not all colors should be equally saturated. Create visual hierarchy through **strategic saturation levels**.

**Saturation Hierarchy:**

| Element Type | Saturation | Example |
|--------------|------------|---------|
| Data visualization | 60-85% | Project colors, charts (candy bright) |
| Primary actions | 70-90% | Buttons, links, active tabs |
| Success/Error states | 90-100% | Status indicators, alerts (pure neon for urgency) |
| Backgrounds | 0-10% | Surface colors, canvas |
| Borders | 0% | Separators, outlines |
| Text | 0% (white) | Body copy, headings |

**Implementation Example:**

```css
/* Candy bright - vibrant but refined (60-85% saturation) */
--project-cyan: #5AC8FA;      /* 75% saturation - iOS blue */
--project-green: #4ADE80;     /* 75% saturation - fresh green */
--project-purple: #A78BFA;    /* 70% saturation - soft purple */

/* Pure neon - urgent states only (90-100% saturation) */
--success-50: #00FF41;        /* 100% saturation - immediate recognition */
--error-50: #FF0055;          /* 100% saturation - demands attention */

/* Low saturation - recedes into background */
--bg-primary: #0f0f11;        /* ~2% saturation */
--border-default: rgba(255, 255, 255, 0.12);  /* 0% saturation */
```

**The 90/10 Rule:**
Approximately 90% of your interface should use low-saturation colors (backgrounds, text, borders), with 10% using candy bright colors (accents, data, interactions).

---

### Principle 6: Candy Bright Color Palette Design

When creating a candy bright color palette for modern dark mode interfaces, follow these guidelines for optimal vibrancy and readability.

**Spectral Distribution:**
Distribute colors across the spectrum to ensure visual distinction:

```
Cyan:    #5AC8FA  (200° hue, 75% sat, 98% lum) - iOS blue
Green:   #4ADE80  (142° hue, 75% sat, 87% lum) - Fresh green
Purple:  #A78BFA  (258° hue, 70% sat, 98% lum) - Soft purple
Orange:  #FB923C  (25° hue, 80% sat, 98% lum)  - Warm orange
Pink:    #F472B6  (330° hue, 75% sat, 98% lum) - Vibrant pink
Yellow:  #FBBF24  (45° hue, 85% sat, 98% lum)  - Amber gold
Magenta: #E879F9  (295° hue, 80% sat, 98% lum) - Bright magenta
Teal:    #2DD4BF  (172° hue, 75% sat, 84% lum) - Aqua
Indigo:  #818CF8  (235° hue, 75% sat, 99% lum) - Periwinkle
```

**Design Rules:**
1. **Minimum 30° hue separation** between adjacent colors
2. **Target 60-85% saturation** for vibrant but refined feel
3. **High luminosity (80-99%)** to ensure brightness and readability
4. **Test at low opacity** (20-30%) for background usage
5. **Verify distinctness** for colorblind users (use simulators)
6. **Avoid pure channel values** (00, FF) except for semantic colors

**Why This Saturation Range:**
- **60-70%:** Soft, elegant (purples, blues)
- **70-80%:** Balanced vibrancy (greens, pinks, teals)
- **80-85%:** Maximum pop without harshness (yellows, oranges)

**Background Opacity for Project Colors:**

When using candy bright colors as backgrounds (e.g., allocated cells), use **20-30% opacity**:

```css
.allocated-cell {
  background: color-mix(in srgb, var(--project-color) 25%, transparent);
  border: 1px solid color-mix(in srgb, var(--project-color) 40%, transparent);
}
```

**Why This Opacity Range:**
- **20-30%:** Provides clear color identification without overwhelming
- **Higher luminosity colors** remain distinguishable at lower opacity
- **Better text contrast** for labels on colored backgrounds
- **Reduced visual fatigue** compared to pure neon at same opacity

**Contrast with Pure Neon:**
Candy bright colors work better than 100% saturated neon because:
- At 25% opacity, #5AC8FA (candy) remains identifiable
- At 25% opacity, #00F0FF (neon) can wash out or become indistinct
- Higher luminosity = better visibility at low opacity

---

### Principle 7: Glow and Luminosity Effects

Neon colors can be enhanced with **subtle glow effects** to reinforce the cyberpunk aesthetic, but use sparingly.

**Effective Glow Usage:**

```css
/* Hover state with subtle glow */
.button-primary:hover {
  box-shadow: 0 0 12px rgba(0, 184, 255, 0.4);
}

/* Focus indicator with pronounced glow */
.input:focus {
  box-shadow: 0 0 0 3px rgba(0, 184, 255, 0.3),
              0 0 16px rgba(0, 184, 255, 0.2);
}

/* Active selection with layered glow */
.grid-cell.selected {
  box-shadow: 0 0 0 2px var(--primary-50),
              0 0 12px rgba(0, 184, 255, 0.4),
              0 4px 16px rgba(0, 0, 0, 0.3);
}
```

**Glow Guidelines:**
- **Blur radius:** 8-16px for subtle, 20-32px for dramatic
- **Color opacity:** 20-40% of base neon color
- **Layer count:** 2-3 shadows maximum
- **Performance:** Use `will-change: box-shadow` for animated glows
- **Context:** Reserve for interactive/active states, not static UI

**Anti-pattern:**
Don't add glows to every colored element—this creates visual fatigue and loses the special feeling.

---

### Principle 8: Transitions and State Changes

Smooth transitions between states help users understand the interface without relying solely on color.

**Transition Timing:**

```css
--transition-quick: 150ms cubic-bezier(0.4, 0, 0.2, 1);   /* Hovers, clicks */
--transition-base: 250ms cubic-bezier(0.4, 0, 0.2, 1);    /* State changes */
--transition-slow: 400ms cubic-bezier(0.4, 0, 0.2, 1);    /* Panels, modals */
```

**What to Animate:**
- ✅ Background colors
- ✅ Border colors
- ✅ Box shadows (glows)
- ✅ Opacity
- ✅ Transform (scale, translate)

**What NOT to Animate:**
- ❌ Width/height (causes reflow)
- ❌ Font size (causes text reflow)
- ❌ Padding/margin (causes reflow)

**Easing Function:**
Use `cubic-bezier(0.4, 0, 0.2, 1)` (ease-out) for most transitions—it creates a responsive, natural feel.

---

### Principle 9: Accessibility in Dark Mode

Dark mode must meet **WCAG 2.1 Level AA** minimum standards for contrast ratios.

**Contrast Requirements:**

| Element | Minimum Contrast | Example |
|---------|------------------|---------|
| Normal text (14px) | 4.5:1 | White on #0f0f11 = 18:1 ✅ |
| Large text (18px+) | 3:1 | rgba(255,255,255,0.75) on #18181a = 12:1 ✅ |
| UI components | 3:1 | Borders, icons, interactive elements |
| Non-text contrast | 3:1 | Charts, graphs, status indicators |

**Testing Tools:**
- [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)
- Chrome DevTools: Lighthouse accessibility audit
- [Stark](https://www.getstark.co/) for Figma/Sketch
- [Colorblindly](https://github.com/oftheheadland/Colorblindly) browser extension

**Color Blind Considerations:**
- Test palette with **deuteranopia** (red-green, most common) and **protanopia** simulators
- Don't rely on color alone—use **icons, labels, patterns** for status
- Ensure **hue separation** of 30°+ between critical colors
- Verify distinction at **15-25% opacity** (background usage)

**High Contrast Mode:**
Ensure your interface remains usable in OS-level high contrast modes:
- Test on Windows High Contrast themes
- Verify forced-colors media query compliance
- Ensure borders remain visible when `forced-colors: active`

---

### Principle 10: Typography in Dark Mode

Typography requires special consideration in dark mode to maintain readability.

**Font Weight Adjustments:**

In dark mode, white text on dark backgrounds can appear **thicker** than dark text on light backgrounds (halation effect). Compensate by:

```css
body {
  font-weight: 400;           /* Normal for light mode */
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* Optional: Reduce weight slightly in dark mode */
@media (prefers-color-scheme: dark) {
  body {
    font-weight: 350;  /* Slightly thinner if your font supports variable weights */
  }
}
```

**Font Rendering:**
- Always use `-webkit-font-smoothing: antialiased` for macOS/iOS
- Use `-moz-osx-font-smoothing: grayscale` for Firefox on macOS
- Avoid `text-rendering: optimizeLegibility` (performance cost)

**Font Size Considerations:**
- **Minimum body text:** 14px for comfortable reading
- **Minimum UI text:** 12px (captions, labels)
- **Code/monospace:** May need 13-14px for clarity

**Line Height:**
- **Body text:** 1.5-1.6
- **Headlines:** 1.2-1.3
- **UI labels:** 1.3-1.4

---

## Implementation Checklist

When implementing these principles in a new project:

### Phase 1: Foundation
- [ ] Define 4-level background elevation system (#0a-#12 base)
- [ ] Create text hierarchy (100%, 75%, 50%, 30% white)
- [ ] Establish 3-level border system (8%, 12%, 18% white)
- [ ] Set up transition timing tokens

### Phase 2: Color System
- [ ] Design neon accent palette (6-9 colors, 30°+ hue separation)
- [ ] Define semantic colors (primary, success, warning, error)
- [ ] Test background opacity for neon colors (20-30% range)
- [ ] Verify color distinction for colorblind users

### Phase 3: Accessibility
- [ ] Run WCAG contrast audits (aim for AA minimum)
- [ ] Test with colorblind simulators
- [ ] Verify keyboard focus indicators
- [ ] Test in high contrast mode

### Phase 4: Refinement
- [ ] Add strategic glow effects to interactive elements
- [ ] Fine-tune transition durations
- [ ] Optimize font rendering settings
- [ ] Cross-browser testing (Chrome, Firefox, Safari)

### Phase 5: Documentation
- [ ] Document color tokens and usage
- [ ] Create component examples
- [ ] Write migration guide (if updating existing system)
- [ ] Share with design/engineering teams

---

## Common Pitfalls to Avoid

### ❌ Pure Black Backgrounds
**Problem:** Creates harsh contrast, loses elevation system
**Solution:** Use #0a-#12 range as darkest base

### ❌ Pure Neon Colors for Data Visualization
**Problem:** 100% saturated colors (#00FF00, #FF00FF) are harsh and wash out at low opacity
**Solution:** Use candy bright palette (60-85% saturation) for project colors and data viz

### ❌ Insufficient Elevation Steps
**Problem:** All surfaces blend together
**Solution:** Maintain 8-10 hex point increments between levels

### ❌ Over-using Bright Colors
**Problem:** Visual fatigue, loss of hierarchy, "Christmas tree" effect
**Solution:** Follow 90/10 rule—90% subtle (grays/whites), 10% candy bright (accents/data)

### ❌ Solid Gray Borders
**Problem:** Creates harsh lines, competes with elevation
**Solution:** Use translucent white (rgba) at 8-18% opacity

### ❌ Ignoring Colorblind Users
**Problem:** Color-coded information becomes inaccessible
**Solution:** Use icons, labels, patterns in addition to color

### ❌ Inconsistent Opacity Values
**Problem:** Text hierarchy breaks down across surfaces
**Solution:** Use semantic tokens (--text-primary, --text-secondary)

### ❌ Glows on Everything
**Problem:** Effect loses special feeling, causes performance issues
**Solution:** Reserve glows for hover/focus/active states only

### ❌ Skipping Contrast Testing
**Problem:** Accessibility violations, poor readability
**Solution:** Test every color combination with contrast checker

### ❌ Browser-Specific Colors
**Problem:** Inconsistent appearance across browsers
**Solution:** Specify colors in hex/rgba, test color-mix() carefully

---

## Testing Strategy

### Visual Regression Testing
1. **Capture baseline screenshots** in Chrome, Firefox, Safari
2. **Test at multiple viewport sizes** (1280px, 1920px, 2560px)
3. **Verify color consistency** across browsers (color-mix can vary)
4. **Check at different zoom levels** (100%, 125%, 150%)

### Accessibility Audits
1. **Automated:** Lighthouse, axe DevTools, WAVE
2. **Manual:** Keyboard navigation, screen reader testing
3. **Simulators:** Colorblind modes, high contrast mode
4. **Real users:** Test with users who have visual impairments

### Performance Monitoring
1. **CSS file size:** Keep under 50KB for design tokens
2. **Animation performance:** Monitor frame rates during transitions
3. **Paint times:** Avoid expensive box-shadow on large elements
4. **Rendering cost:** Profile with Chrome DevTools Performance tab

---

## Maintenance and Evolution

### When to Update Colors
- **User feedback:** Accessibility complaints, readability issues
- **Brand evolution:** Logo/brand color changes
- **Platform updates:** OS dark mode guideline changes
- **Technology shifts:** New CSS features (e.g., color-mix improvements)

### Versioning Strategy
- **Major version:** Complete palette overhaul
- **Minor version:** New colors added, no removals
- **Patch version:** Contrast fixes, accessibility improvements

### Documentation Updates
- Maintain **live style guide** with interactive examples
- Document **decision rationale** for future reference
- Update **this ADR** as principles evolve
- Create **migration guides** for breaking changes

---

## References and Further Reading

### Industry Standards
- [Apple WWDC 2019: Implementing Dark Mode on iOS](https://developer.apple.com/videos/play/wwdc2019/214/)
- [Material Design: Dark Theme](https://m3.material.io/styles/color/dark-mode/overview)
- [WCAG 2.1 Contrast Guidelines](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html)

### Color Theory
- [Color and Contrast: Dark Mode Edition](https://www.smashingmagazine.com/2020/07/color-contrast-dark-mode/)
- [Designing for Colorblindness](https://webaim.org/articles/visual/colorblind)
- [HSL vs RGB: Understanding Color Models](https://hslpicker.com/)

### Modern SaaS Color Palettes
- [Tailwind CSS Color Palette](https://tailwindcss.com/docs/customizing-colors) - Excellent candy bright colors
- [Radix UI Colors](https://www.radix-ui.com/colors) - Designed for dark mode interfaces
- [Open Color](https://yeun.github.io/open-color/) - Optimized color schemes for UI design
- [Asana Design System](https://asana.com/) - Example of refined candy bright in production

### Tools
- [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)
- [Coolors Palette Generator](https://coolors.co/)
- [Colorblindly Simulator](https://github.com/oftheheadland/Colorblindly)
- [Chrome DevTools Color Picker](https://developer.chrome.com/docs/devtools/css/)

---

## Appendix: Quick Reference

### Background System Template
```css
--bg-primary: #0f0f11;     /* Base canvas */
--bg-secondary: #18181a;   /* Lifted surfaces */
--bg-tertiary: #212125;    /* Cards/panels */
--bg-overlay: #2a2a2e;     /* Modals/menus */
```

### Text Hierarchy Template
```css
--text-primary: #FFFFFF;
--text-secondary: rgba(255, 255, 255, 0.75);
--text-tertiary: rgba(255, 255, 255, 0.50);
--text-disabled: rgba(255, 255, 255, 0.30);
```

### Border System Template
```css
--border-subtle: rgba(255, 255, 255, 0.08);
--border-default: rgba(255, 255, 255, 0.12);
--border-emphasis: rgba(255, 255, 255, 0.18);
```

### Candy Bright Project Colors Template
```css
/* Data visualization - 60-85% saturation */
--project-cyan: #5AC8FA;      /* iOS blue */
--project-green: #4ADE80;     /* Fresh green */
--project-purple: #A78BFA;    /* Soft purple */
--project-orange: #FB923C;    /* Warm orange */
--project-pink: #F472B6;      /* Vibrant pink */
--project-yellow: #FBBF24;    /* Amber gold */
--project-magenta: #E879F9;   /* Bright magenta */
--project-teal: #2DD4BF;      /* Aqua */
--project-indigo: #818CF8;    /* Periwinkle */
```

### Semantic Colors Template
```css
/* Urgent states - 90-100% saturation for immediate recognition */
--primary-50: #00B8FF;      /* Electric cyan-blue */
--success-50: #00FF41;      /* Matrix green */
--warning-50: #FFB800;      /* Amber alert */
--error-50: #FF0055;        /* Hot red */
```

### Transition Timing Template
```css
--transition-quick: 150ms cubic-bezier(0.4, 0, 0.2, 1);
--transition-base: 250ms cubic-bezier(0.4, 0, 0.2, 1);
--transition-slow: 400ms cubic-bezier(0.4, 0, 0.2, 1);
```

---

## Conclusion

Sophisticated dark mode design balances **subtle layering** with **strategic vibrancy**. The background system creates depth through careful elevation, while candy bright accents provide energetic visual interest without harshness. By following these principles, you create interfaces that are both **aesthetically striking** and **functionally excellent**—the hallmark of great design.

Remember: **Good dark mode is invisible**. Users should feel comfortable, focused, and energized—not overwhelmed by darkness or fatigued by excessive saturation. The candy bright approach (60-85% saturation) strikes this balance perfectly, delivering the modern SaaS aesthetic seen in apps like Asana, Notion, and M365. Test thoroughly, verify accessibility, and iterate based on real usage.

---

**Last Updated:** 2025-11-22 (revised to candy bright palette from pure neon)
**Next Review:** When starting new dark mode project or after significant platform updates

**Revision History:**
- 2025-11-22: Initial version with pure neon colors (100% saturation)
- 2025-11-22: Revised to candy bright palette (60-85% saturation) based on modern SaaS aesthetic (Asana, M365, Notion)
