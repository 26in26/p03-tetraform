# P03 — Tetraform

> 26 in 26 · Weeks 05–06 · Game Development + Web

## Goal
Experiment with **Dioxus 0.7** and build an engaging volcanic-themed landing page featuring advanced procedural generation and smooth animations. Explore modern web UI patterns in Rust while creating a visually striking experience that showcases a volcanic game concept.

## Scope
**In scope**
- Landing page for volcanic game (Tetraform)
- Procedurally generated animated rocky cliff walls
- Dioxus 0.7 routing and component framework
- Responsive design with Tailwind CSS
- Animation effects (lava glow, grid shimmer, vignette)

**Out of scope**
- Actual game mechanics or gameplay
- Backend/API integration
- Mobile apps (web-only for now)
- Audio/sound effects

## Timeline
- **Week 1:** Design, research, Dioxus 0.7 spike
- **Week 2:** Implementation, animation polish, documentation

## Status
- [x] Design
- [x] POC
- [x] Core implementation

## 🛠 Tech Stack
- **Language:** Rust
- **Framework:** Dioxus 0.7 (web)
- **Styling:** Tailwind CSS + Custom CSS
- **Procedural Generation:** Rust `rand` crate
- **Tooling:** dx CLI, Make

### Note on Assets
The base cliff wall assets (`cliff.png`) were **hand-drawn and animated in Aseprite**, then procedurally layered and positioned to create depth and variation.

## 🚀 Running the Project
```bash
# Install Dioxus CLI (if not already installed)
curl -sSL http://dioxus.dev/install.sh | sh

# Run the development server
make run

# Or directly with dx
dx serve
```

The app will be available at `http://localhost:8080` (or as indicated by the CLI).

## 🎨 Features
- **Procedurally Generated Cliffs:** Rocky walls are created via procedural generation, layered for depth perception
- **Animated Effects:** Lava glow shimmer, grid overlay, vignette effects
- **Responsive Layout:** Works on desktop and tablet screens
- **Routing:** Home page with extensible routing pattern (Blog route included for future expansion)


<p align="center"><img alt="tetraform" src="docs/page.gif" /></p>