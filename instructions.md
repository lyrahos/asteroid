# Lightweight Linux Browser - Technical Specification
**Project Name:** Asteroid Browser
**Version:** 1.0 Specification
**Date:** February 2026

## Build Instructions for AI Agent

This specification describes a complete, production-ready lightweight web browser for Linux. Build ALL components described in this document:

**What to Build:**
1. Complete Rust + GTK4 application following the architecture in this spec
2. Gecko engine integration with optimizations
3. All memory management systems (tab suspension, monitoring, trimming)
4. Content blocking engine with filter list support
5. Video hardware acceleration (VA-API)
6. Complete UI (minimal chrome, keyboard shortcuts, settings page)
7. Tab management with vertical sidebar option
8. Engine abstraction layer for future Servo support
9. Test suite and benchmarking tools
10. Build scripts for .deb, .rpm, and Flatpak packaging
11. **GitHub repository with manual Actions workflows for:**
    - Icon generation from SVG (manual trigger, choose environment)
    - Installation package building (manual trigger, choose env + package type)
    - NO automatic triggers on push/pull requests
12. **Auto-update mechanism that checks GitHub releases**

**Key Constraints:**
- RAM usage MUST meet targets: <150MB idle, <300MB for 5 tabs
- Use Gecko engine (NOT Chromium, NOT WebKit)
- Clean abstraction layer to enable future Servo migration
- Minimal UI - keyboard-first, no bloat
- Built-in ad/tracker blocking
- Hardware video acceleration mandatory

**GitHub Integration:**
- Set up repository with proper structure
- **GitHub Actions workflows are MANUAL ONLY** (workflow_dispatch)
  - Icon generation workflow (choose: main/test/dev)
  - Package building workflow (choose: environment, package type)
  - NO automatic builds on push/pull request
- Workflows include icon generation from SVG logos
- Automated release artifacts with 90-day retention
- Update checker that pulls from GitHub releases API

Start with the directory structure in the "Engine Abstraction Details" section and implement all components described throughout this document.

## Executive Summary

A minimal-RAM, high-performance browser for Linux that maintains independence from corporate control while supporting modern web standards. Uses Gecko (Firefox engine) initially with a clean abstraction layer enabling future migration to Servo or other engines.

---

## Branding & Logo

### Logo Design

**Concept:** 

*Animated Version:* Asteroids orbiting in a circular ring at varying speeds - dynamic asteroid belt representing continuous browsing

*Static Version:* **Small, friendly asteroid with gentle motion trail** - Simple, approachable, cosmic - quiet exploration through the web

**Files:**
- `resources/logo.svg` OR `resources/asteroid-logo-animated.svg` - Animated orbital ring (for loading screens, web, splash)
- `resources/logo-static.svg` OR `resources/asteroid-logo-icon.svg` - **Official friendly asteroid icon** (for icons, app launchers, branding, favicons)
- Generated from static logo: 16x16, 32x32, 48x48, 128x128, 256x256 PNG icons

**Design Philosophy:**

*Animated Version (for loading/web):*
- Circular asteroid ring (16 asteroids total)
- Variable speeds creating natural, organic motion
- Random spacing for realistic belt effect
- Perfect for loading screens and dynamic displays

*Static Version (FRIENDLY ICON):*
- 🪨 **Small, irregular asteroid** - organic, asymmetrical shape with soft curves
- **Gentle motion trail** - single subtle arc suggesting calm movement
- **Light, friendly colors** - soft pale gray/blue asteroid (#d4dce6) with muted cyan trail (#5a9fc8)
- **Minimal texture** - smooth surface with subtle highlights, no obvious craters
- **Flat design** - modern, clean, approachable
- **Non-intimidating** - feels like quiet exploration, not impact or aggression
- **Scales perfectly** - works beautifully as favicon and large icon
- Represents: **Lightweight browsing, calm navigation, friendly exploration**

**Color Palette:**

*Static Icon (official):*
- Background: `#01111E` (very dark blue-black)
- Asteroid body: `#E6F0F3` (very light blue-gray/off-white)
- Shading: `#AAC9DC` (light blue-gray for depth)
- Motion trail: `#7DC6DA` (bright cyan/turquoise)
- Accent dots: White and light blue
- Creates soft, friendly, approachable aesthetic

*Animated Version:*
- Background: `#0a0e1a` (deep space black)
- Asteroids: `#6b7b9e`, `#7a8aad`, `#8a9ac0` (blue-gray tones)
- Highlights: `#99aad1`, `#aabae6`
- Stars: White with varying opacity

**What Makes the Static Logo Work:**
- **Simple and friendly** - non-threatening, approachable design
- **Unique shape** - irregular asteroid, not a perfect circle
- **Motion without aggression** - gentle arc suggests calm exploration
- **Modern and lightweight** - flat design, minimal details
- **Universal appeal** - suitable for all ages
- **Distinctive without copying** - sits comfortably alongside Firefox/Safari
- **Excellent scalability** - clear and recognizable at any size
- **Quiet confidence** - understated, not flashy

**Animation Details (animated version):**
- All asteroids orbit clockwise around the center
- Speed variety creates dynamic, natural movement:
  - Tiny asteroids: 8-11 seconds (fast)
  - Small asteroids: 12-14 seconds (medium-fast)
  - Medium asteroids: 16-22 seconds (medium)
  - Large asteroids: 24-28 seconds (slow)
- Random starting positions prevent uniform spacing

**Usage:**
- **Static (friendly icon):** App icons, desktop launchers, package managers, favicons, taskbar
- **Animated (orbital ring):** Loading screens, splash screens, about dialogs, website headers
- Both maintain brand consistency - cosmic theme, different expressions

The logos represent:
- **Friendly asteroid** = Approachable browsing, lightweight and simple
- **Gentle motion trail** = Calm navigation, smooth experience
- **Orbital ring** (animated) = Continuous navigation, ecosystem, multitasking
- **Cosmic theme** = Exploration, discovery, going beyond
- **Minimal design** = Lightweight browser philosophy
- **Soft colors** = Non-intimidating, modern, accessible
- **Quiet exploration** = Thoughtful browsing without aggression

### Logo SVG Files

**Create these exact SVG files in `resources/` directory:**

#### Static Logo: `resources/logo-static.svg` or `resources/asteroid-logo-icon.svg`

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="1024" height="1024" viewBox="0 0 1024 1024">
<g>
<path d="M 0.00 0.00 L 1024.00 0.00 L 1024.00 1024.00 L 0.00 1024.00 ZM 350.50 624.40 C357.93,625.15 391.42,623.89 403.50,622.40 C423.22,619.99 448.49,614.94 478.21,607.47 C490.23,604.45 497.57,603.08 499.63,603.46 C501.36,603.78 507.43,605.80 513.13,607.95 C533.61,615.67 547.88,618.44 567.00,618.43 C582.18,618.43 590.92,616.92 608.50,611.28 C629.29,604.60 650.62,593.03 662.38,582.05 C669.05,575.82 674.58,567.71 681.96,553.33 C687.81,541.92 688.81,540.60 698.07,532.04 C714.95,516.42 726.56,498.56 731.16,481.12 C733.56,472.01 734.04,454.51 732.14,445.50 C728.44,427.98 718.58,407.53 706.62,392.60 C699.64,383.88 677.97,363.12 670.10,357.61 C651.49,344.61 641.84,341.53 622.01,342.26 C610.31,342.69 606.85,343.24 596.19,346.37 C589.42,348.36 577.72,352.52 570.19,355.62 C552.73,362.81 545.76,364.52 528.50,365.85 C512.81,367.06 504.75,369.06 494.50,374.29 C482.31,380.50 472.62,390.29 457.88,411.26 C447.52,426.00 440.55,434.66 433.54,441.50 C410.95,463.54 398.72,494.66 402.00,521.75 C403.97,538.01 408.75,548.48 418.97,558.90 C427.55,567.66 434.68,572.41 452.18,581.05 C460.05,584.93 466.62,588.44 466.76,588.84 C466.91,589.24 459.48,590.35 450.26,591.30 C379.03,598.65 329.31,594.26 305.59,578.52 C294.03,570.84 288.86,560.09 290.77,547.73 C294.39,524.32 320.38,495.96 364.00,467.82 C374.29,461.18 396.09,448.35 400.00,446.64 C400.91,446.24 400.72,446.15 399.50,446.41 C396.00,447.17 376.90,455.52 365.00,461.49 C345.57,471.24 313.00,491.71 313.00,494.18 C313.00,494.63 312.49,495.00 311.86,495.00 C310.01,495.00 297.14,505.76 288.00,514.96 C267.98,535.10 258.00,554.13 258.00,572.17 C258.00,582.71 260.85,589.05 269.41,597.54 C279.91,607.96 293.37,614.31 315.79,619.43 C325.09,621.55 332.11,622.56 350.50,624.40 ZM 709.05 608.56 C709.71,609.35 711.48,610.00 713.00,610.00 C716.47,610.00 718.24,607.92 717.82,604.33 C717.44,601.01 713.02,598.87 710.20,600.64 C708.11,601.95 707.43,606.61 709.05,608.56 ZM 342.65 407.17 C345.76,410.60 351.00,408.37 351.00,403.63 C351.00,400.68 349.19,399.00 346.00,399.00 C341.20,399.00 339.35,403.52 342.65,407.17 Z" fill="rgb(1,17,30)"/>
<path d="M 350.50 624.40 C332.11,622.56 325.09,621.55 315.79,619.43 C293.37,614.31 279.91,607.96 269.41,597.54 C260.85,589.05 258.00,582.71 258.00,572.17 C258.00,554.13 267.98,535.10 288.00,514.96 C297.14,505.76 310.01,495.00 311.86,495.00 C312.49,495.00 313.00,494.63 313.00,494.18 C313.00,491.71 345.57,471.24 365.00,461.49 C376.90,455.52 396.00,447.17 399.50,446.41 C400.28,446.24 400.64,446.22 400.56,446.33 C398.37,446.03 380.04,454.25 380.70,455.33 C381.06,455.91 380.70,456.10 379.83,455.77 C378.98,455.44 378.33,455.75 378.33,456.47 C378.33,457.89 381.71,456.73 388.50,452.98 C380.40,457.68 370.15,463.85 364.00,467.82 C320.38,495.96 294.39,524.32 290.77,547.73 C288.86,560.09 294.03,570.84 305.59,578.52 C329.31,594.26 379.03,598.65 450.26,591.30 C459.48,590.35 466.91,589.24 466.76,588.84 C466.66,588.57 463.58,586.85 459.13,584.55 C462.57,586.27 464.99,587.39 467.54,587.97 C472.55,589.12 478.03,588.20 492.71,585.76 C495.02,585.37 497.56,584.95 500.35,584.49 C526.75,580.16 550.79,575.22 571.50,569.86 C591.58,564.67 592.01,564.58 591.96,565.63 C591.91,567.00 583.53,572.00 571.00,578.15 C558.60,584.23 541.04,590.48 518.00,597.00 C510.02,599.26 502.63,601.36 501.57,601.68 C499.77,602.21 499.81,602.34 502.07,603.51 C503.37,604.18 504.92,604.89 506.65,605.62 C503.40,604.50 500.69,603.66 499.63,603.46 C497.57,603.08 490.23,604.45 478.21,607.47 C448.49,614.94 423.22,619.99 403.50,622.40 C391.42,623.89 357.93,625.15 350.50,624.40 ZM 709.05 608.56 C707.43,606.61 708.11,601.95 710.20,600.64 C713.02,598.87 717.44,601.01 717.82,604.33 C718.24,607.92 716.47,610.00 713.00,610.00 C711.48,610.00 709.71,609.35 709.05,608.56 ZM 649.00 590.70 C648.17,591.38 647.28,591.92 647.00,591.89 C642.16,591.43 643.46,583.92 648.72,581.91 C652.10,580.63 653.91,582.18 651.91,584.65 C650.85,585.96 650.83,586.19 651.82,585.59 C652.92,584.93 652.92,585.19 651.82,587.13 C651.10,588.41 649.83,590.02 649.00,590.70 ZM 571.35 618.38 C578.83,617.92 586.97,617.03 591.50,616.01 C602.39,613.56 615.10,609.25 626.06,604.56 C620.31,607.11 614.39,609.39 608.50,611.28 C592.67,616.36 584.01,618.09 571.35,618.38 ZM 520.64 610.65 C529.56,613.47 539.23,615.93 546.00,616.91 C547.74,617.16 549.54,617.43 551.15,617.67 C541.16,616.62 531.71,614.42 520.64,610.65 ZM 645.38 594.53 C646.15,594.00 646.86,593.49 647.52,592.99 C650.36,590.79 653.13,589.00 653.67,589.00 C653.72,589.00 653.76,588.99 653.82,588.98 C651.20,590.86 648.37,592.71 645.38,594.53 ZM 660.03 584.14 C663.20,581.17 666.86,577.32 669.48,574.12 C667.17,577.17 664.85,579.74 662.38,582.05 C661.63,582.75 660.85,583.45 660.03,584.14 ZM 674.71 566.39 C677.02,562.52 679.76,557.61 681.96,553.33 C679.35,558.40 676.98,562.69 674.71,566.39 Z" fill="rgb(125,198,218)"/>
<path d="M 571.35 618.38 C569.96,618.42 568.51,618.43 567.00,618.43 C561.39,618.44 556.20,618.20 551.15,617.67 C549.54,617.43 547.74,617.16 546.00,616.91 C539.23,615.93 529.56,613.47 520.64,610.65 C518.22,609.83 515.73,608.93 513.13,607.95 C510.94,607.13 508.69,606.32 506.65,605.62 C504.92,604.89 503.37,604.18 502.07,603.51 C499.81,602.34 499.77,602.21 501.57,601.68 C502.63,601.36 510.02,599.26 518.00,597.00 C541.04,590.48 558.60,584.23 571.00,578.15 C583.53,572.00 591.91,567.00 591.96,565.63 C592.01,564.58 591.58,564.67 571.50,569.86 C550.79,575.22 526.75,580.16 500.35,584.49 C497.56,584.95 495.02,585.37 492.71,585.76 C478.03,588.20 472.55,589.12 467.54,587.97 C464.99,587.39 462.57,586.27 459.13,584.55 C457.05,583.48 454.68,582.28 452.18,581.05 C445.78,577.89 440.76,575.25 436.56,572.74 L 436.54 572.73 C436.44,572.67 436.34,572.61 436.25,572.55 C429.84,568.71 425.32,565.16 420.57,560.50 C420.04,559.98 419.51,559.45 418.97,558.90 L 418.97 558.90 C409.39,549.12 404.59,539.31 402.40,524.70 C403.30,528.29 405.19,530.16 409.08,533.29 C416.03,538.90 431.71,547.19 439.80,549.55 C442.93,550.45 449.76,551.81 454.99,552.57 C466.89,554.28 484.42,553.68 497.64,551.11 C503.06,550.06 513.80,546.86 521.50,544.00 C529.20,541.15 540.90,537.27 547.50,535.38 C578.68,526.46 596.51,519.31 616.50,507.69 C626.87,501.66 651.00,483.33 651.00,481.49 C651.00,481.25 649.63,480.34 647.96,479.48 C635.41,472.99 640.73,450.75 658.05,437.30 C666.24,430.94 673.06,428.09 680.21,428.04 C686.94,427.99 686.99,427.93 689.48,416.58 C691.35,408.04 691.90,392.07 690.65,382.65 C690.03,377.96 690.14,376.96 691.19,377.58 C692.25,378.21 689.25,374.78 685.74,371.09 C693.96,378.81 702.66,387.65 706.62,392.60 C718.58,407.53 728.44,427.98 732.14,445.50 C734.04,454.51 733.56,472.01 731.16,481.12 C726.56,498.56 714.95,516.42 698.07,532.04 C688.81,540.60 687.81,541.92 681.96,553.33 C679.76,557.61 677.02,562.52 674.71,566.39 L 674.71 566.39 C672.93,569.30 671.21,571.84 669.48,574.12 C666.86,577.32 663.20,581.17 660.03,584.14 C658.14,585.76 656.06,587.38 653.82,588.98 C653.76,588.99 653.72,589.00 653.67,589.00 C653.13,589.00 650.36,590.79 647.52,592.99 C646.86,593.49 646.15,594.00 645.38,594.53 C639.46,598.11 632.89,601.52 626.06,604.56 C615.10,609.25 602.39,613.56 591.50,616.01 C586.97,617.03 578.83,617.92 571.35,618.38 ZM 452.29 503.30 C452.63,504.21 451.76,504.64 449.46,504.70 C446.05,504.80 445.00,504.35 445.00,502.77 C445.00,502.28 445.52,502.20 446.15,502.60 C446.95,503.08 447.03,502.87 446.43,501.90 C445.95,501.13 445.69,499.66 445.86,498.64 C446.07,497.37 445.77,497.02 444.91,497.55 C444.02,498.11 443.85,497.82 444.33,496.58 C444.70,495.62 445.00,494.39 445.00,493.83 C445.00,491.87 447.46,493.69 448.01,496.07 C448.55,498.37 449.03,498.52 456.79,498.79 C467.91,499.18 467.92,502.00 456.81,502.00 C453.09,502.00 451.92,502.34 452.29,503.30 ZM 649.00 590.70 C649.83,590.02 651.10,588.41 651.82,587.13 C652.92,585.19 652.92,584.93 651.82,585.59 C650.83,586.19 650.85,585.96 651.91,584.65 C653.91,582.18 652.10,580.63 648.72,581.91 C643.46,583.92 642.16,591.43 647.00,591.89 C647.28,591.92 648.17,591.38 649.00,590.70 ZM 400.00 446.64 C398.43,447.33 393.95,449.82 388.50,452.98 C381.71,456.73 378.33,457.89 378.33,456.47 C378.33,455.75 378.98,455.44 379.83,455.77 C380.70,456.10 381.06,455.91 380.70,455.33 C380.04,454.25 398.37,446.03 400.56,446.33 C400.51,446.39 400.33,446.49 400.00,446.64 ZM 401.72 518.98 C401.05,510.70 401.78,502.09 403.77,493.51 C402.43,499.38 401.71,505.25 401.66,511.00 C401.63,514.22 401.64,516.83 401.72,518.98 ZM 668.76 356.68 C664.03,353.46 658.97,350.40 654.95,348.41 C652.22,347.06 649.80,345.97 647.47,345.10 C654.01,347.33 660.51,351.00 668.76,356.68 ZM 403.77 493.51 C408.03,474.80 418.56,456.13 433.50,441.54 C418.54,456.15 408.13,474.75 403.77,493.51 Z" fill="rgb(170,201,220)"/>
<path d="M 454.99 552.57 C449.76,551.81 442.93,550.45 439.80,549.55 C431.71,547.19 416.03,538.90 409.08,533.29 C402.04,527.62 401.53,526.08 401.66,511.00 C401.87,486.89 413.89,460.67 433.54,441.50 C440.55,434.66 447.52,426.00 457.88,411.26 C472.62,390.29 482.31,380.50 494.50,374.29 C504.75,369.06 512.81,367.06 528.50,365.85 C545.76,364.52 552.73,362.81 570.19,355.62 C577.72,352.52 589.42,348.36 596.19,346.37 C606.85,343.24 610.31,342.69 622.01,342.26 C637.57,341.68 643.64,342.82 654.95,348.41 C662.27,352.02 673.00,359.19 679.00,364.47 C683.38,368.32 692.96,378.63 691.19,377.58 C690.14,376.96 690.03,377.96 690.65,382.65 C691.90,392.07 691.35,408.04 689.48,416.58 C686.99,427.93 686.94,427.99 680.21,428.04 C673.06,428.09 666.24,430.94 658.05,437.30 C640.73,450.75 635.41,472.99 647.96,479.48 C649.63,480.34 651.00,481.25 651.00,481.49 C651.00,483.33 626.87,501.66 616.50,507.69 C596.51,519.31 578.68,526.46 547.50,535.38 C540.90,537.27 529.20,541.15 521.50,544.00 C513.80,546.86 503.06,550.06 497.64,551.11 C484.42,553.68 466.89,554.28 454.99,552.57 ZM 452.29 503.30 C451.92,502.34 453.09,502.00 456.81,502.00 C467.92,502.00 467.91,499.18 456.79,498.79 C449.03,498.52 448.55,498.37 448.01,496.07 C447.46,493.69 445.00,491.87 445.00,493.83 C445.00,494.39 444.70,495.62 444.33,496.58 C443.85,497.82 444.02,498.11 444.91,497.55 C445.77,497.02 446.07,497.37 445.86,498.64 C445.69,499.66 445.95,501.13 446.43,501.90 C447.03,502.87 446.95,503.08 446.15,502.60 C445.52,502.20 445.00,502.28 445.00,502.77 C445.00,504.35 446.05,504.80 449.46,504.70 C451.76,504.64 452.63,504.21 452.29,503.30 ZM 342.65 407.17 C339.35,403.52 341.20,399.00 346.00,399.00 C349.19,399.00 351.00,400.68 351.00,403.63 C351.00,408.37 345.76,410.60 342.65,407.17 Z" fill="rgb(230,240,243)"/>
</g>
</svg>
```

#### Animated Logo: `resources/logo.svg` or `resources/asteroid-logo-animated.svg`

```svg
<svg width="256" height="256" viewBox="0 0 256 256" xmlns="http://www.w3.org/2000/svg">
  <!-- Dark space theme with circular asteroid ring -->
  <!-- Asteroids orbit at different speeds and random positions -->
  
  <!-- Dark background circle -->
  <circle cx="128" cy="128" r="128" fill="#0a0e1a"/>
  
  <!-- Ring guide path (subtle) -->
  <circle cx="128" cy="128" r="70" 
          fill="none" 
          stroke="#2a3347" 
          stroke-width="40" 
          opacity="0.2"/>
  
  <!-- Asteroids orbiting at different speeds and random positions -->
  
  <!-- Large asteroid 1 - slow rotation (25s) -->
  <g transform="rotate(15 128 128)">
    <g transform="translate(0 58)">
      <path d="M 128,-12 L 136,-8 L 138,2 L 132,10 L 122,8 L 118,0 L 122,-10 Z" 
            fill="#6b7b9e" 
            stroke="#8a9ac0" 
            stroke-width="1.5"/>
      <circle cx="126" cy="-3" r="2.5" fill="#4a5a7d" opacity="0.6"/>
      <circle cx="131" cy="2" r="1.8" fill="#4a5a7d" opacity="0.6"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="15 128 128"
      to="375 128 128"
      dur="25s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Medium asteroid 1 - medium speed (18s) -->
  <g transform="rotate(85 128 128)">
    <g transform="translate(0 70)">
      <path d="M 128,-8 L 134,-5 L 135,1 L 131,7 L 124,6 L 121,0 L 124,-7 Z" 
            fill="#7a8aad" 
            stroke="#99aad1" 
            stroke-width="1.5"/>
      <circle cx="127" cy="-2" r="1.5" fill="#5a6a8d" opacity="0.6"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="85 128 128"
      to="445 128 128"
      dur="18s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Small asteroid 1 - fast rotation (12s) -->
  <g transform="rotate(140 128 128)">
    <g transform="translate(0 68)">
      <circle cx="128" cy="0" r="3" fill="#8a9ac0" opacity="0.8"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="140 128 128"
      to="500 128 128"
      dur="12s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Large asteroid 2 - slow rotation (28s) -->
  <g transform="rotate(195 128 128)">
    <g transform="translate(0 62)">
      <path d="M 128,-10 L 135,-7 L 137,1 L 132,9 L 123,7 L 119,0 L 123,-9 Z" 
            fill="#6b7b9e" 
            stroke="#8a9ac0" 
            stroke-width="1.5"/>
      <circle cx="127" cy="-2" r="2.2" fill="#4a5a7d" opacity="0.6"/>
      <circle cx="131" cy="3" r="1.6" fill="#4a5a7d" opacity="0.6"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="195 128 128"
      to="555 128 128"
      dur="28s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Tiny asteroid 1 - very fast (10s) -->
  <g transform="rotate(245 128 128)">
    <g transform="translate(0 73)">
      <circle cx="128" cy="0" r="2.2" fill="#7a8aad" opacity="0.7"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="245 128 128"
      to="605 128 128"
      dur="10s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Medium asteroid 2 - medium-slow (22s) -->
  <g transform="rotate(280 128 128)">
    <g transform="translate(0 66)">
      <path d="M 128,-7 L 134,-4 L 135,2 L 131,8 L 124,6 L 121,-1 L 124,-6 Z" 
            fill="#8a9ac0" 
            stroke="#aabae6" 
            stroke-width="1.5"/>
      <circle cx="128" cy="1" r="1.4" fill="#6a7a9d" opacity="0.6"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="280 128 128"
      to="640 128 128"
      dur="22s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Small asteroid 2 - fast (14s) -->
  <g transform="rotate(320 128 128)">
    <g transform="translate(0 71)">
      <circle cx="128" cy="0" r="2.8" fill="#6a7a9d" opacity="0.75"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="320 128 128"
      to="680 128 128"
      dur="14s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Large asteroid 3 - medium (20s) -->
  <g transform="rotate(10 128 128)">
    <g transform="translate(0 60)">
      <path d="M 128,-11 L 136,-7 L 138,3 L 133,10 L 123,8 L 119,1 L 122,-9 Z" 
            fill="#7a8aad" 
            stroke="#99aad1" 
            stroke-width="1.5"/>
      <circle cx="127" cy="-1" r="2.3" fill="#5a6a8d" opacity="0.6"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="10 128 128"
      to="370 128 128"
      dur="20s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Tiny asteroid 2 - very fast (9s) -->
  <g transform="rotate(55 128 128)">
    <g transform="translate(0 74)">
      <circle cx="128" cy="0" r="2" fill="#8a9ac0" opacity="0.65"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="55 128 128"
      to="415 128 128"
      dur="9s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Medium asteroid 3 - medium-fast (16s) -->
  <g transform="rotate(110 128 128)">
    <g transform="translate(0 67)">
      <path d="M 128,-6 L 133,-4 L 134,1 L 131,6 L 125,5 L 122,0 L 125,-5 Z" 
            fill="#6b7b9e" 
            stroke="#8a9ac0" 
            stroke-width="1.5"/>
      <circle cx="128" cy="0" r="1.3" fill="#4a5a7d" opacity="0.6"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="110 128 128"
      to="470 128 128"
      dur="16s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Small asteroid 3 - fast (13s) -->
  <g transform="rotate(165 128 128)">
    <g transform="translate(0 72)">
      <circle cx="128" cy="0" r="2.5" fill="#7a8aad" opacity="0.7"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="165 128 128"
      to="525 128 128"
      dur="13s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Medium asteroid 4 - slow (24s) -->
  <g transform="rotate(220 128 128)">
    <g transform="translate(0 64)">
      <path d="M 128,-8 L 135,-5 L 136,2 L 132,8 L 124,6 L 121,-1 L 124,-7 Z" 
            fill="#8a9ac0" 
            stroke="#aabae6" 
            stroke-width="1.5"/>
      <circle cx="129" cy="0" r="1.5" fill="#6a7a9d" opacity="0.6"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="220 128 128"
      to="580 128 128"
      dur="24s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Tiny asteroid 3 - super fast (8s) -->
  <g transform="rotate(265 128 128)">
    <g transform="translate(0 75)">
      <circle cx="128" cy="0" r="1.8" fill="#6a7a9d" opacity="0.6"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="265 128 128"
      to="625 128 128"
      dur="8s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Small asteroid 4 - medium (17s) -->
  <g transform="rotate(305 128 128)">
    <g transform="translate(0 69)">
      <circle cx="128" cy="0" r="2.6" fill="#8a9ac0" opacity="0.75"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="305 128 128"
      to="665 128 128"
      dur="17s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Medium asteroid 5 - medium-slow (21s) -->
  <g transform="rotate(340 128 128)">
    <g transform="translate(0 65)">
      <path d="M 128,-7 L 134,-4 L 135,2 L 131,7 L 124,5 L 121,0 L 124,-6 Z" 
            fill="#7a8aad" 
            stroke="#99aad1" 
            stroke-width="1.5"/>
      <circle cx="128" cy="-1" r="1.6" fill="#5a6a8d" opacity="0.6"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="340 128 128"
      to="700 128 128"
      dur="21s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Tiny asteroid 4 - fast (11s) -->
  <g transform="rotate(30 128 128)">
    <g transform="translate(0 73)">
      <circle cx="128" cy="0" r="2.1" fill="#7a8aad" opacity="0.65"/>
    </g>
    <animateTransform
      attributeName="transform"
      type="rotate"
      from="30 128 128"
      to="390 128 128"
      dur="11s"
      repeatCount="indefinite"/>
  </g>
  
  <!-- Stars (background - static) -->
  <circle cx="40" cy="40" r="1.5" fill="#ffffff" opacity="0.6"/>
  <circle cx="216" cy="40" r="1" fill="#ffffff" opacity="0.5"/>
  <circle cx="230" cy="220" r="1.2" fill="#ffffff" opacity="0.7"/>
  <circle cx="30" cy="200" r="0.8" fill="#ffffff" opacity="0.4"/>
  <circle cx="220" cy="128" r="1" fill="#ffffff" opacity="0.5"/>
  <circle cx="36" cy="128" r="1.3" fill="#ffffff" opacity="0.6"/>
  <circle cx="180" cy="210" r="0.9" fill="#ffffff" opacity="0.5"/>
  <circle cx="50" cy="160" r="1.1" fill="#ffffff" opacity="0.6"/>
</svg>
```

**IMPORTANT:** Copy these SVG files exactly as shown. The static logo is the primary branding icon. The animated logo is for loading screens and dynamic displays.

## Core Design Principles

1. **Engine Independence** - Clean separation between UI and rendering engine
2. **Minimal Resource Footprint** - Target <300MB RAM for basic browsing
3. **Aggressive Optimization** - Every byte and CPU cycle counts
4. **Modern Web Support** - Must handle video, JavaScript-heavy sites
5. **Corporate Independence** - No telemetry, tracking, or vendor lock-in
6. **Keyboard-First UX** - Minimal chrome, maximum content

---

## Architecture Overview

### Three-Layer Architecture

```
┌─────────────────────────────────────┐
│   UI Layer (Rust + GTK4)            │
│   - Window management               │
│   - Tab bar, address bar            │
│   - Keyboard shortcuts              │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   Abstraction Layer (Rust)          │
│   - Engine-agnostic API             │
│   - Resource management             │
│   - IPC coordination                │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   Engine Layer (Gecko/Servo)        │
│   - Web rendering                   │
│   - JavaScript execution            │
│   - Network stack                   │
└─────────────────────────────────────┘
```

### Component Breakdown

#### 1. UI Layer
- **Technology:** Rust + GTK4 (native Linux, lightweight)
- **Responsibilities:**
  - Minimal window chrome (address bar, tab switcher)
  - Keyboard shortcut handling
  - Tab lifecycle management
  - Settings/preferences UI
  
**Key Features:**
- Vertical tab sidebar option (saves horizontal space)
- Minimal toolbar (auto-hide on idle)
- No bloat: no bookmarks toolbar, no extension buttons initially
- Status overlay instead of permanent status bar

#### 2. Abstraction Layer (Engine API)

**Purpose:** Provide a stable, engine-agnostic interface

```rust
trait BrowserEngine {
    fn create_view(&mut self, view_id: ViewId) -> Result<()>;
    fn load_url(&mut self, view_id: ViewId, url: &str) -> Result<()>;
    fn execute_script(&mut self, view_id: ViewId, script: &str) -> Result<Value>;
    fn suspend_view(&mut self, view_id: ViewId) -> Result<()>;
    fn resume_view(&mut self, view_id: ViewId) -> Result<()>;
    fn destroy_view(&mut self, view_id: ViewId) -> Result<()>;
    
    // Video optimization hooks
    fn set_video_decoder(&mut self, decoder: VideoDecoder) -> Result<()>;
    fn enable_hardware_acceleration(&mut self, enabled: bool) -> Result<()>;
    
    // Memory management
    fn get_memory_usage(&self) -> MemoryStats;
    fn trim_memory(&mut self, level: TrimLevel) -> Result<()>;
}
```

**Implementations:**
- `GeckoEngine` - Initial implementation using Mozilla's Gecko
- `ServoEngine` - Future implementation (stubbed initially)
- Test mocks for development

#### 3. Engine Layer - Gecko Implementation

**Base:** GeckoView or embedded Gecko (NOT full Firefox)

**Optimizations:**

1. **Compile-time removals:**
   - Disable Pocket integration
   - Remove Firefox Sync
   - Strip telemetry/crash reporting
   - Remove Firefox Accounts
   - Disable built-in extensions (Pocket, Screenshots, etc.)
   - Remove Safe Browsing lookups (can use local filters)

2. **Custom preferences (`prefs.js`):**
```javascript
// Memory optimizations
user_pref("browser.sessionhistory.max_total_viewers", 0);
user_pref("browser.sessionstore.interval", 60000);
user_pref("browser.cache.memory.capacity", 51200); // 50MB max
user_pref("media.memory_cache_max_size", 32768); // 32MB
user_pref("browser.tabs.unloadOnLowMemory", true);

// Aggressive tab discarding
user_pref("browser.tabs.min_inactive_duration_before_unload", 300000); // 5min

// Disable animations/transitions
user_pref("browser.tabs.animate", false);
user_pref("browser.fullscreen.animate", false);

// Video optimizations
user_pref("media.hardware-video-decoding.enabled", true);
user_pref("media.ffmpeg.vaapi.enabled", true); // Linux hardware decode
user_pref("layers.acceleration.force-enabled", true);
user_pref("gfx.webrender.all", true); // GPU rendering
```

---

## Memory Optimization Strategy

### Target RAM Usage
- **Idle (1 tab):** <150MB
- **Light use (5 tabs):** <300MB  
- **Heavy use (10 tabs, video):** <600MB

### Techniques

#### 1. Aggressive Tab Suspension
```rust
struct TabManager {
    active_tab: ViewId,
    suspended_tabs: HashMap<ViewId, SuspendedState>,
    suspension_timer: Duration,
}

impl TabManager {
    fn on_tab_inactive(&mut self, view_id: ViewId) {
        // After 5 minutes of inactivity, suspend tab
        schedule_suspension(view_id, Duration::from_secs(300));
    }
    
    fn suspend_tab(&mut self, view_id: ViewId) {
        // Serialize DOM state, screenshots
        let state = self.engine.serialize_view(view_id);
        self.suspended_tabs.insert(view_id, state);
        self.engine.destroy_view(view_id);
        // Shows screenshot placeholder in tab
    }
}
```

#### 2. Single Process Model (Initially)
- Unlike Chrome/Firefox multi-process, use single process
- Reduces IPC overhead and memory duplication
- Trade security isolation for RAM savings
- Can add sandboxing via Linux namespaces/seccomp

#### 3. Memory Pressure Monitoring
```rust
fn monitor_memory_pressure() {
    loop {
        let mem_info = get_system_memory();
        
        if mem_info.available < CRITICAL_THRESHOLD {
            // Suspend all inactive tabs immediately
            tab_manager.suspend_all_inactive();
            engine.trim_memory(TrimLevel::Aggressive);
            
        } else if mem_info.available < LOW_THRESHOLD {
            // Suspend oldest inactive tabs
            tab_manager.suspend_oldest_inactive(3);
            engine.trim_memory(TrimLevel::Moderate);
        }
        
        sleep(Duration::from_secs(10));
    }
}
```

#### 4. Efficient Cache Strategy
- **Disk cache:** 100MB max (SSD-optimized)
- **Memory cache:** 50MB max
- **Image cache:** Aggressive compression, lazy decode
- **HTTP cache:** Prioritize revalidation over storage

#### 5. Content Blocking (Saves RAM + Bandwidth)
- Built-in ad/tracker blocking (uBlock Origin-style filters)
- Blocks ads/trackers before they load → saves parsing, DOM, memory
- Load filter lists at startup (EasyList, EasyPrivacy)

---

## Video Optimization

### Critical for Low-End Hardware

#### 1. Hardware Acceleration (VA-API on Linux)
```rust
fn initialize_video_decoder() -> Result<()> {
    // Probe for VA-API support
    if vaapi_available() {
        engine.set_video_decoder(VideoDecoder::VAAPI);
        engine.enable_hardware_acceleration(true);
    } else {
        // Fallback to optimized software decode
        engine.set_video_decoder(VideoDecoder::FFmpegOptimized);
    }
    Ok(())
}
```

**Ensure Gecko builds with:**
- `--enable-vaapi` flag
- FFmpeg with VA-API enabled
- WebRender for GPU compositing

#### 2. Adaptive Quality
- Hook into video elements via JS injection
- Monitor CPU/RAM usage during playback
- Auto-reduce quality on struggling systems
```javascript
// Injected into video-heavy pages
if (navigator.hardwareConcurrency <= 2) {
    // Low-end CPU detected
    const videos = document.querySelectorAll('video');
    videos.forEach(v => {
        // Prefer lower resolutions
        v.addEventListener('loadedmetadata', () => {
            if (v.videoHeight > 480) {
                // Signal to player to prefer 480p
            }
        });
    });
}
```

#### 3. Preload Control
```javascript
user_pref("media.autoplay.default", 5); // Block autoplay
user_pref("media.autoplay.block-webaudio", true);
// User must click to play = saves RAM until needed
```

---

## User Interface Design

### Minimal Chrome Philosophy

```
┌─────────────────────────────────────────────┐
│ [←][→][⟳] example.com          [☰]         │ ← Minimal toolbar
├─────────────────────────────────────────────┤
│                                             │
│                                             │
│          Web Content Here                   │
│                                             │
│                                             │
│                                             │
└─────────────────────────────────────────────┘
 Status: "Loading..." (appears bottom-left only when needed)
```

**Features:**
- **Address bar:** Combined search + URL (omnibox style)
- **Navigation:** Back, Forward, Reload only
- **Menu:** Single hamburger menu (☰)
- **No bookmarks bar** - Access via Ctrl+B or search
- **No tabs bar initially** - Vertical sidebar toggle or Ctrl+Tab switcher

### Keyboard-First Interaction

**Essential Shortcuts:**
```
Ctrl+L       - Focus address bar
Ctrl+T       - New tab
Ctrl+W       - Close tab
Ctrl+Tab     - Tab switcher overlay
Ctrl+F       - Find in page
Ctrl+Shift+H - History
F11          - Fullscreen
/            - Quick find (like vim)
```

**Vim-style hints for links:**
- Press `f` to show numbered hints over all clickable elements
- Type number to click (saves mouse usage)

### Tab Management

**Vertical Tab Sidebar (Optional):**
```
┌──────┬────────────────────────┐
│ [1]  │ ←→⟳ example.com   ☰   │
│ [2]  │                        │
│ [3]  │   Web Content          │
│      │                        │
│ [+]  │                        │
└──────┴────────────────────────┘
```

**Benefits:**
- Visible tab list for monitors (horizontal space)
- Easy scanning of many tabs
- Toggle with F1 or Ctrl+Shift+T

---

## Technical Stack

### Core Technologies

**Language:** Rust
- Memory safe
- Zero-cost abstractions
- Excellent Linux ecosystem
- Can interface with Gecko C++ via FFI

**UI Framework:** GTK4
- Native Linux look/feel
- Lightweight
- Good Wayland support
- Mature, stable

**Engine:** Gecko (Mozilla SpiderMonkey + Gecko layout)
- Use `mozjs` crate for SpiderMonkey bindings
- GeckoView embedder API (Android uses this, adaptable)
- OR use servo/components parts if easier

**Build System:** Cargo + Meson (for Gecko integration)

**Dependencies:**
```toml
[dependencies]
gtk4 = "0.8"
webkit2gtk = false  # Explicitly not using WebKit
mozjs = "0.15"      # SpiderMonkey bindings
serde = "1.0"
tokio = "1.0"       # Async runtime
reqwest = "0.11"    # HTTP client (supplementary)
adblock = "0.8"     # Filter list engine
```

---

## Core Requirements

### Essential Features to Implement

**Browser Core:**
- Rust + GTK4 project structure
- Minimal window with address bar
- Gecko engine integration (embedding)
- URL loading and page rendering
- Navigation (back/forward/reload)
- Multi-tab support with suspension
- Keyboard shortcuts (see UI Design section)

**Memory Management:**
- Tab suspension/restoration system
- Memory pressure monitoring daemon
- Aggressive tab unloading on low memory
- Tab switcher UI (Ctrl+Tab overlay)
- Memory trim triggers

**Performance:**
- Video hardware acceleration (VA-API)
- Content blocking engine (ad/tracker filters)
- Gecko compile-time optimizations
- Custom preference tuning
- Performance monitoring/metrics

**User Features:**
- Settings UI (HTML-based, minimal)
- History and bookmarks
- Download manager
- Find in page
- Vertical tab sidebar (optional)
- Vim-style link hints

**Engine Abstraction:**
- Clean abstraction layer (BrowserEngine trait)
- Gecko engine implementation
- Servo engine stub (for future)
- Compile-time engine selection
- Test infrastructure

**Distribution:**
- .deb packaging
- .rpm packaging  
- Flatpak manifest
- Build scripts

---

## Engine Abstraction Details

### Directory Structure
```
asteroid-browser/
├── src/
│   ├── main.rs
│   ├── ui/              # GTK UI layer
│   ├── core/            # Abstraction layer
│   │   ├── engine.rs    # BrowserEngine trait
│   │   ├── tab.rs       # Tab management
│   │   ├── memory.rs    # Memory monitoring
│   │   └── updater.rs   # GitHub update checker
│   ├── engines/
│   │   ├── gecko/       # Gecko implementation
│   │   │   ├── mod.rs
│   │   │   ├── prefs.rs
│   │   │   └── ffi.rs
│   │   └── servo/       # Servo stub (future)
│   │       └── mod.rs
├── resources/           # UI resources, icons, logo
│   ├── logo.svg         # Animated logo
│   ├── logo-static.svg  # Static logo
│   ├── icons/           # Generated PNG icons
│   │   ├── 16x16/
│   │   ├── 32x32/
│   │   ├── 48x48/
│   │   ├── 128x128/
│   │   └── 256x256/
│   ├── ui/              # UI resource files
│   └── filters/         # Ad-block filter lists
├── gecko-config/        # Gecko build configs
├── scripts/             # Build scripts
│   ├── build-deb.sh
│   ├── build-rpm.sh
│   ├── build-flatpak.sh
│   └── generate-icons.sh
├── .github/
│   └── workflows/       # GitHub Actions
├── test-pages/          # Test HTML pages
└── Cargo.toml
```

### Build-Time Engine Selection
```toml
[features]
default = ["gecko-engine"]
gecko-engine = ["mozjs"]
servo-engine = ["servo/components"]

[dependencies]
mozjs = { version = "0.15", optional = true }
# servo = { git = "...", optional = true }
```

```rust
// src/engines/mod.rs
#[cfg(feature = "gecko-engine")]
pub use gecko::GeckoEngine as DefaultEngine;

#[cfg(feature = "servo-engine")]
pub use servo::ServoEngine as DefaultEngine;
```

### Migration Path to Servo

**When Servo is ready:**
1. Implement `BrowserEngine` trait for Servo
2. Test with both engines in parallel
3. Compare performance/compatibility
4. Switch default feature flag
5. Maintain Gecko as fallback

**Servo readiness checklist:**
- [ ] CSS Grid support
- [ ] WebGL support
- [ ] Service Workers
- [ ] 95%+ Web Platform Tests pass rate
- [ ] Production-ready stability

---

## Performance Targets & Validation

### Required Benchmarks

**Startup Time:**
- Cold start: <2 seconds
- Warm start: <0.5 seconds

**Memory Usage (Must Meet):**
| Scenario | Target | Maximum |
|----------|--------|---------|
| Idle (1 tab, blank) | 80MB | 150MB |
| Simple page (Wikipedia) | 120MB | 180MB |
| 5 tabs (news sites) | 250MB | 300MB |
| 10 tabs + video | 500MB | 600MB |

**Video Playback:**
- 1080p YouTube: <5% dropped frames on 4-core CPU
- Hardware decode latency: <100ms
- CPU usage during playback: <15% on low-end CPU

**Page Load Times:**
- Simple page: <1 second to interactive
- Heavy JS site: <3 seconds to interactive
- With content blocking: 30% faster than Firefox baseline

### Validation Test Suite

Build these automated tests:

```bash
# Memory benchmarks
cargo run --release --bin bench-memory

# Video performance
cargo run --release --bin bench-video

# Page load times
cargo run --release --bin bench-pageload -- https://example.com
```

Test pages needed:
- `test-pages/simple.html` - Basic HTML
- `test-pages/heavy-js.html` - React app simulation
- `test-pages/video.html` - HTML5 video embed
- `test-pages/ads.html` - Ad-heavy page for blocker testing

---

## Content Blocking (Ad/Tracker Blocking)

### Why Built-In?

1. **Memory savings:** Ads/trackers never load → less DOM, JS, RAM
2. **Speed:** Pages load faster without ad network requests
3. **Privacy:** Independent from corporate interests
4. **Bandwidth:** Saves data on metered connections

### Implementation

Use **adblock crate** (Rust implementation of uBlock Origin filters)

```rust
use adblock::{Engine, FilterFormat};

struct ContentBlocker {
    engine: Engine,
}

impl ContentBlocker {
    fn new() -> Self {
        let mut engine = Engine::new(true); // Enable optimizations
        
        // Load filter lists
        engine.add_filter_list(EASYLIST, FilterFormat::Standard);
        engine.add_filter_list(EASYPRIVACY, FilterFormat::Standard);
        
        Self { engine }
    }
    
    fn should_block(&self, url: &str, source_url: &str, resource_type: &str) -> bool {
        self.engine.check_network_urls(url, source_url, resource_type)
            .matched
    }
}
```

**Integration:**
- Hook into Gecko's request pipeline
- Block requests before network fetch
- Saves DNS, TCP, TLS, HTTP overhead

**Filter lists to include:**
- EasyList (ads)
- EasyPrivacy (trackers)
- Annoyances (cookie notices, etc.)
- Regional lists (optional, user choice)

---

## Configuration & Preferences

### User-Accessible Settings

**Minimal UI (Settings page HTML + CSS):**

```
General:
  [ ] Enable tab suspension (5 min inactive)
  [ ] Show vertical tab sidebar
  [ ] Vim-style link hints
  
Performance:
  [x] Hardware video acceleration
  [ ] Aggressive memory trimming
  Cache size: [100] MB
  
Privacy:
  [x] Block ads and trackers
  [ ] Send Do Not Track header
  [ ] Delete cookies on close
  
Advanced:
  Engine: [Gecko v124] (Servo when available)
  [ ] Enable developer tools
```

### Config File

**Location:** `~/.config/asteroid-browser/config.toml`

```toml
[general]
tab_suspension_enabled = true
tab_suspension_delay = 300  # seconds
vertical_tabs = false

[performance]
hardware_acceleration = true
memory_trim_level = "moderate"  # off, moderate, aggressive
cache_size_mb = 100

[privacy]
block_ads = true
block_trackers = true
send_dnt = false
clear_cookies_on_close = false

[engine]
current = "gecko"
# future: current = "servo"
```

---

## Security Considerations

### Trade-offs for Minimal RAM

**Single-process model risks:**
- No site isolation
- One compromised site can affect others
- Mitigations:
  - Linux namespaces for basic sandboxing
  - Seccomp-bpf to restrict syscalls
  - Drop unnecessary capabilities
  
**Future:** Consider lightweight process isolation
- Use Linux user namespaces
- One process per security domain (origin)
- Still lighter than full Chrome isolation

### Content Security

- HTTPS-only mode (warn on HTTP)
- Certificate validation
- Safe Browsing alternative (local phishing filter)
- Auto-update mechanism for security patches

---

## Development Tools

### Debug Build Features

```bash
# Run with debug logging
RUST_LOG=flux=debug cargo run

# Memory profiling
heaptrack cargo run --release
# Then: heaptrack_gui heaptrack.flux.PID.gz

# Performance profiling
perf record -g cargo run --release
perf report
```

### Test Pages

Create test suite of pages for benchmarking:
- `test-pages/simple.html` - Basic HTML
- `test-pages/heavy-js.html` - React app simulation
- `test-pages/video.html` - HTML5 video embed
- `test-pages/ads.html` - Ad-heavy page for blocker testing

---

## GitHub Actions & CI/CD

### Repository Structure

```
asteroid-browser/
├── .github/
│   └── workflows/
│       ├── generate-icons.yml     # Generate PNG icons from SVG (manual)
│       └── build-package.yml      # Build installation packages (manual)
├── scripts/
│   ├── build-deb.sh
│   ├── build-rpm.sh
│   ├── build-flatpak.sh
│   └── generate-icons.sh
├── src/
├── Cargo.toml
├── README.md
└── LICENSE
```

### GitHub Actions Workflows

**IMPORTANT:** All workflows are **MANUAL ONLY** - they use `workflow_dispatch` and will NOT run automatically on push or pull requests. You must manually trigger them from the GitHub Actions tab.

#### 1. Icon Generation Workflow (`.github/workflows/generate-icons.yml`)

Generates PNG icons from SVG logo files. **Manual trigger only.**

```yaml
name: Generate Icons

on:
  workflow_dispatch:
    inputs:
      environment:
        description: 'Target environment'
        required: true
        type: choice
        options:
          - main
          - test
          - dev
        default: 'dev'

jobs:
  generate-icons:
    runs-on: ubuntu-24.04
    
    steps:
    - uses: actions/checkout@v4
      with:
        ref: ${{ github.event.inputs.environment }}
    
    - name: Install dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y inkscape imagemagick
    
    - name: Generate PNG icons from SVG
      run: |
        chmod +x ./scripts/generate-icons.sh
        ./scripts/generate-icons.sh
    
    - name: Commit generated icons
      run: |
        git config --local user.email "action@github.com"
        git config --local user.name "GitHub Action"
        git add resources/icons/
        git diff --quiet && git diff --staged --quiet || git commit -m "chore: generate PNG icons for ${{ github.event.inputs.environment }}"
    
    - name: Push changes
      uses: ad-m/github-push-action@master
      with:
        github_token: ${{ secrets.GITHUB_TOKEN }}
        branch: ${{ github.event.inputs.environment }}
    
    - name: Upload icons as artifacts
      uses: actions/upload-artifact@v3
      with:
        name: asteroid-icons-${{ github.event.inputs.environment }}
        path: resources/icons/
        retention-days: 30
```

#### 2. Build Package Workflow (`.github/workflows/build-package.yml`)

Builds installation packages (.deb, .rpm, Flatpak). **Manual trigger only.**

```yaml
name: Build Installation Package

on:
  workflow_dispatch:
    inputs:
      environment:
        description: 'Target environment'
        required: true
        type: choice
        options:
          - main
          - test
          - dev
        default: 'dev'
      package_type:
        description: 'Package type to build'
        required: true
        type: choice
        options:
          - all
          - deb
          - rpm
          - flatpak
        default: 'all'
      version:
        description: 'Version string (e.g., 1.0.0-dev)'
        required: false
        type: string

jobs:
  setup:
    runs-on: ubuntu-24.04
    outputs:
      version: ${{ steps.set-version.outputs.version }}
      build_deb: ${{ steps.set-packages.outputs.build_deb }}
      build_rpm: ${{ steps.set-packages.outputs.build_rpm }}
      build_flatpak: ${{ steps.set-packages.outputs.build_flatpak }}
    
    steps:
    - name: Set version
      id: set-version
      run: |
        if [ -z "${{ github.event.inputs.version }}" ]; then
          VERSION="1.0.0-${{ github.event.inputs.environment }}-$(date +%Y%m%d)"
        else
          VERSION="${{ github.event.inputs.version }}"
        fi
        echo "version=$VERSION" >> $GITHUB_OUTPUT
    
    - name: Determine packages to build
      id: set-packages
      run: |
        if [ "${{ github.event.inputs.package_type }}" = "all" ]; then
          echo "build_deb=true" >> $GITHUB_OUTPUT
          echo "build_rpm=true" >> $GITHUB_OUTPUT
          echo "build_flatpak=true" >> $GITHUB_OUTPUT
        elif [ "${{ github.event.inputs.package_type }}" = "deb" ]; then
          echo "build_deb=true" >> $GITHUB_OUTPUT
          echo "build_rpm=false" >> $GITHUB_OUTPUT
          echo "build_flatpak=false" >> $GITHUB_OUTPUT
        elif [ "${{ github.event.inputs.package_type }}" = "rpm" ]; then
          echo "build_deb=false" >> $GITHUB_OUTPUT
          echo "build_rpm=true" >> $GITHUB_OUTPUT
          echo "build_flatpak=false" >> $GITHUB_OUTPUT
        elif [ "${{ github.event.inputs.package_type }}" = "flatpak" ]; then
          echo "build_deb=false" >> $GITHUB_OUTPUT
          echo "build_rpm=false" >> $GITHUB_OUTPUT
          echo "build_flatpak=true" >> $GITHUB_OUTPUT
        fi

  build-deb:
    needs: setup
    if: needs.setup.outputs.build_deb == 'true'
    runs-on: ubuntu-24.04
    
    steps:
    - uses: actions/checkout@v4
      with:
        ref: ${{ github.event.inputs.environment }}
    
    - name: Install dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y libgtk-4-dev libva-dev build-essential \
          libglib2.0-dev libcairo2-dev libpango1.0-dev libgdk-pixbuf2.0-dev \
          dpkg-dev inkscape
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: nightly
        override: true
    
    - name: Generate icons if needed
      run: |
        if [ ! -d "resources/icons/16x16" ]; then
          chmod +x ./scripts/generate-icons.sh
          ./scripts/generate-icons.sh
        fi
    
    - name: Build DEB package
      run: |
        chmod +x ./scripts/build-deb.sh
        VERSION=${{ needs.setup.outputs.version }} ./scripts/build-deb.sh
    
    - name: Generate checksums
      run: |
        cd dist
        sha256sum *.deb > asteroid-browser-${{ github.event.inputs.environment }}.deb.sha256
    
    - name: Upload DEB package
      uses: actions/upload-artifact@v3
      with:
        name: asteroid-browser-${{ github.event.inputs.environment }}-deb
        path: |
          dist/*.deb
          dist/*.sha256
        retention-days: 90

  build-rpm:
    needs: setup
    if: needs.setup.outputs.build_rpm == 'true'
    runs-on: ubuntu-24.04
    container: fedora:latest
    
    steps:
    - uses: actions/checkout@v4
      with:
        ref: ${{ github.event.inputs.environment }}
    
    - name: Install dependencies
      run: |
        dnf install -y gtk4-devel libva-devel gcc gcc-c++ \
          glib2-devel cairo-devel pango-devel gdk-pixbuf2-devel \
          rpm-build rpmdevtools inkscape
    
    - name: Install Rust
      run: |
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
        rustup default nightly
    
    - name: Generate icons if needed
      run: |
        if [ ! -d "resources/icons/16x16" ]; then
          chmod +x ./scripts/generate-icons.sh
          ./scripts/generate-icons.sh
        fi
    
    - name: Build RPM package
      run: |
        source $HOME/.cargo/env
        chmod +x ./scripts/build-rpm.sh
        VERSION=${{ needs.setup.outputs.version }} ./scripts/build-rpm.sh
    
    - name: Generate checksums
      run: |
        cd dist
        sha256sum *.rpm > asteroid-browser-${{ github.event.inputs.environment }}.rpm.sha256
    
    - name: Upload RPM package
      uses: actions/upload-artifact@v3
      with:
        name: asteroid-browser-${{ github.event.inputs.environment }}-rpm
        path: |
          dist/*.rpm
          dist/*.sha256
        retention-days: 90

  build-flatpak:
    needs: setup
    if: needs.setup.outputs.build_flatpak == 'true'
    runs-on: ubuntu-24.04
    
    steps:
    - uses: actions/checkout@v4
      with:
        ref: ${{ github.event.inputs.environment }}
    
    - name: Install Flatpak
      run: |
        sudo apt-get update
        sudo apt-get install -y flatpak flatpak-builder inkscape
    
    - name: Generate icons if needed
      run: |
        if [ ! -d "resources/icons/16x16" ]; then
          chmod +x ./scripts/generate-icons.sh
          ./scripts/generate-icons.sh
        fi
    
    - name: Build Flatpak
      run: |
        chmod +x ./scripts/build-flatpak.sh
        VERSION=${{ needs.setup.outputs.version }} ./scripts/build-flatpak.sh
    
    - name: Upload Flatpak
      uses: actions/upload-artifact@v3
      with:
        name: asteroid-browser-${{ github.event.inputs.environment }}-flatpak
        path: dist/*.flatpak
        retention-days: 90

  summary:
    needs: [setup, build-deb, build-rpm, build-flatpak]
    if: always()
    runs-on: ubuntu-24.04
    
    steps:
    - name: Build Summary
      run: |
        echo "## Build Summary" >> $GITHUB_STEP_SUMMARY
        echo "" >> $GITHUB_STEP_SUMMARY
        echo "**Environment:** ${{ github.event.inputs.environment }}" >> $GITHUB_STEP_SUMMARY
        echo "**Version:** ${{ needs.setup.outputs.version }}" >> $GITHUB_STEP_SUMMARY
        echo "**Package Type:** ${{ github.event.inputs.package_type }}" >> $GITHUB_STEP_SUMMARY
        echo "" >> $GITHUB_STEP_SUMMARY
        echo "**Built Packages:**" >> $GITHUB_STEP_SUMMARY
        if [ "${{ needs.setup.outputs.build_deb }}" = "true" ]; then
          echo "- ✅ DEB package" >> $GITHUB_STEP_SUMMARY
        fi
        if [ "${{ needs.setup.outputs.build_rpm }}" = "true" ]; then
          echo "- ✅ RPM package" >> $GITHUB_STEP_SUMMARY
        fi
        if [ "${{ needs.setup.outputs.build_flatpak }}" = "true" ]; then
          echo "- ✅ Flatpak package" >> $GITHUB_STEP_SUMMARY
        fi
```

### How to Use These Workflows

**To Generate Icons:**
1. Go to GitHub → Actions tab
2. Select "Generate Icons" workflow
3. Click "Run workflow"
4. Choose environment (main/test/dev)
5. Click "Run workflow" button

**To Build Installation Packages:**
1. Go to GitHub → Actions tab
2. Select "Build Installation Package" workflow
3. Click "Run workflow"
4. Choose:
   - Environment (main/test/dev)
   - Package type (all/deb/rpm/flatpak)
   - Optional: Version string
5. Click "Run workflow" button

**Important Notes:**
- Workflows are **MANUAL ONLY** - no automatic triggers
- Icons are automatically generated during package build if missing
- Packages are uploaded as artifacts with 90-day retention
- Each environment can have its own builds
- Version strings are auto-generated if not provided

### Auto-Update Mechanism

#### Update Checker Implementation

```rust
// src/core/updater.rs
use serde::Deserialize;

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub struct UpdateChecker {
    current_version: String,
    repo: String,
}

impl UpdateChecker {
    pub fn new(current_version: &str) -> Self {
        Self {
            current_version: current_version.to_string(),
            repo: "username/asteroid-browser".to_string(),
        }
    }
    
    pub async fn check_for_updates(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let url = format!("https://api.github.com/repos/{}/releases/latest", self.repo);
        
        let client = reqwest::Client::builder()
            .user_agent("asteroid-browser")
            .build()?;
        
        let release: GitHubRelease = client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;
        
        if release.tag_name > self.current_version {
            Ok(Some(release.tag_name))
        } else {
            Ok(None)
        }
    }
    
    pub async fn download_update(&self, version: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("https://api.github.com/repos/{}/releases/tags/{}", self.repo, version);
        
        let client = reqwest::Client::builder()
            .user_agent("asteroid-browser")
            .build()?;
        
        let release: GitHubRelease = client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;
        
        // Find the appropriate package for this system
        let asset = release.assets.iter()
            .find(|a| a.name.ends_with(".deb") || a.name.ends_with(".rpm"))
            .ok_or("No compatible package found")?;
        
        Ok(asset.browser_download_url.clone())
    }
}
```

#### Update Check Schedule

```rust
// Check for updates every 24 hours
pub fn start_update_checker() {
    tokio::spawn(async {
        let checker = UpdateChecker::new(env!("CARGO_PKG_VERSION"));
        
        loop {
            match checker.check_for_updates().await {
                Ok(Some(version)) => {
                    // Show notification to user
                    show_update_notification(&version);
                }
                Ok(None) => {
                    // Up to date
                }
                Err(e) => {
                    eprintln!("Update check failed: {}", e);
                }
            }
            
            // Wait 24 hours
            tokio::time::sleep(tokio::time::Duration::from_secs(86400)).await;
        }
    });
}
```

---

## Packaging & Distribution

### Target Formats

**1. Native Packages:**
- `.deb` for Debian/Ubuntu
- `.rpm` for Fedora/RHEL
- `PKGBUILD` for Arch AUR

**2. Universal Packages:**
- Flatpak (recommended for sandboxing)
- AppImage (fully portable)

**3. Source:**
- GitHub releases with source tarball
- Build instructions for all platforms

### Dependencies

**Runtime dependencies:**
- GTK4
- GLib
- libva (for video acceleration)
- FFmpeg (if not statically linked)

**Build dependencies:**
- Rust toolchain (1.75+)
- Gecko build deps (if building from source)
- Meson, Ninja

### Desktop Integration

**Desktop Entry File** (`asteroid-browser.desktop`):

```ini
[Desktop Entry]
Version=1.0
Type=Application
Name=Asteroid Browser
GenericName=Web Browser
Comment=Lightweight, fast browser for low-resource systems
Exec=/usr/bin/asteroid-browser %U
Icon=asteroid-browser
Terminal=false
Categories=Network;WebBrowser;
MimeType=text/html;text/xml;application/xhtml+xml;x-scheme-handler/http;x-scheme-handler/https;
StartupNotify=true
StartupWMClass=asteroid-browser
Keywords=web;browser;internet;
Actions=NewWindow;NewPrivateWindow;

[Desktop Action NewWindow]
Name=New Window
Exec=/usr/bin/asteroid-browser --new-window

[Desktop Action NewPrivateWindow]
Name=New Private Window
Exec=/usr/bin/asteroid-browser --private-window
```

Install locations:
- Binary: `/usr/bin/asteroid-browser`
- Desktop file: `/usr/share/applications/asteroid-browser.desktop`
- Icons: `/usr/share/icons/hicolor/{size}/apps/asteroid-browser.png`
- Resources: `/usr/share/asteroid-browser/`

---

## Future Roadmap

### Post-Initial Build

**Planned Enhancements:**
- Extension support (minimal API, WebExtensions subset)
- Self-hosted sync (not cloud-based)
- Reader mode
- PWA (Progressive Web App) support
- Servo engine migration (when ready)
- Multi-process with lightweight isolation
- Custom protocol handlers
- Advanced fingerprinting resistance
- Distributed web support (IPFS integration)

---

## Build & Development Setup

### Initial Setup

```bash
# Clone repo structure
mkdir asteroid-browser && cd asteroid-browser
cargo init

# Install dependencies (Ubuntu/Debian)
sudo apt install libgtk-4-dev libva-dev build-essential \
    libglib2.0-dev libcairo2-dev libpango1.0-dev \
    libgdk-pixbuf2.0-dev

# Install Rust nightly (for some optimizations)
rustup toolchain install nightly
rustup default nightly

# Build and run
cargo build --release
cargo run --release

# Run tests
cargo test
```

### Development Tools

```bash
# Debug logging
RUST_LOG=flux=debug cargo run

# Memory profiling
heaptrack cargo run --release
heaptrack_gui heaptrack.flux.PID.gz

# Performance profiling
perf record -g cargo run --release
perf report
```

---

## Appendix A: Icon Generation

### Generate PNG Icons from SVG

Create `scripts/generate-icons.sh`:

```bash
#!/bin/bash
# Generate PNG icons from SVG logo

SIZES="16 32 48 128 256"
SVG_SOURCE="resources/logo-static.svg"
OUTPUT_DIR="resources/icons"

# Requires: inkscape or imagemagick

mkdir -p "$OUTPUT_DIR"

for size in $SIZES; do
    mkdir -p "$OUTPUT_DIR/${size}x${size}"
    
    # Using inkscape (preferred)
    if command -v inkscape &> /dev/null; then
        inkscape "$SVG_SOURCE" \
            --export-type=png \
            --export-filename="$OUTPUT_DIR/${size}x${size}/asteroid-browser.png" \
            --export-width=$size \
            --export-height=$size
    
    # Fallback to ImageMagick
    elif command -v convert &> /dev/null; then
        convert -background none \
            "$SVG_SOURCE" \
            -resize ${size}x${size} \
            "$OUTPUT_DIR/${size}x${size}/asteroid-browser.png"
    else
        echo "Error: Neither inkscape nor imagemagick found"
        exit 1
    fi
    
    echo "Generated ${size}x${size} icon"
done

# Generate favicon.ico (multiple sizes embedded)
if command -v convert &> /dev/null; then
    convert "$OUTPUT_DIR/16x16/asteroid-browser.png" \
            "$OUTPUT_DIR/32x32/asteroid-browser.png" \
            "$OUTPUT_DIR/48x48/asteroid-browser.png" \
            "$OUTPUT_DIR/favicon.ico"
    echo "Generated favicon.ico"
fi

echo "Icon generation complete!"
```

Make executable: `chmod +x scripts/generate-icons.sh`

---

## Appendix B: Gecko Build Configuration

### Minimal Gecko Build

```bash
# .mozconfig for minimal Gecko
ac_add_options --enable-application=browser
ac_add_options --disable-tests
ac_add_options --disable-debug
ac_add_options --enable-optimize
ac_add_options --enable-release

# Disable unwanted features
ac_add_options --disable-crashreporter
ac_add_options --disable-updater
ac_add_options --disable-maintenance-service
ac_add_options --disable-backgroundtasks

# Enable hardware acceleration
ac_add_options --enable-webrender
ac_add_options --enable-av1
ac_add_options --enable-ffmpeg
ac_add_options --enable-vaapi

# Strip bloat
ac_add_options --disable-eme  # DRM (if not needed)
ac_add_options --disable-parental-controls
ac_add_options --disable-geckodriver
```

### Gecko Patches

Apply custom patches to remove:
- Firefox branding
- Telemetry code
- Pocket integration
- Firefox Accounts

---

## Appendix C: Servo Future Integration

### Servo Architecture Compatibility

Servo's component architecture aligns well:
- `script` - JavaScript engine
- `style` - CSS engine  
- `layout` - Layout engine
- `net` - Networking
- `compositing` - GPU compositor

### Servo Integration Strategy

```rust
// Future servo/mod.rs
pub struct ServoEngine {
    servo: servo::Servo,
    views: HashMap<ViewId, ServoView>,
}

impl BrowserEngine for ServoEngine {
    fn load_url(&mut self, view_id: ViewId, url: &str) -> Result<()> {
        let view = self.views.get_mut(&view_id)?;
        view.load(url);
        Ok(())
    }
    // ... implement other trait methods
}
```

Monitor Servo's progress on:
- GitHub: https://github.com/servo/servo
- Blog: https://servo.org/blog/
- Annual review of Web Platform Tests pass rate

---

## Appendix D: Memory Profiling Commands

```bash
# Heaptrack (detailed heap usage)
heaptrack ./target/release/asteroid-browser
heaptrack_gui heaptrack.asteroid-browser.*.gz

# Valgrind massif (heap profiler)
valgrind --tool=massif --massif-out-file=massif.out ./target/release/asteroid-browser
ms_print massif.out

# /proc monitoring script
while true; do
    ps aux | grep asteroid-browser | awk '{print $6/1024 " MB"}'
    sleep 1
done

# Systemd monitoring
systemd-run --scope -p MemoryHigh=500M ./target/release/asteroid-browser
```

---

**END OF SPECIFICATION**

This document should be updated as development progresses and new requirements emerge.
