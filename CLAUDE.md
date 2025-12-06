# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a personal portfolio website built with **Leptos** (a Rust web framework) featuring:
- Server-side rendering (SSR) with hydration
- Client-side routing
- TailwindCSS for styling
- Sound effects system
- Multi-page application (Home, Projects, About, Contact)

## Architecture

**Framework**: Leptos 0.8.3 with Rust
- **Frontend**: Leptos components with client-side hydration
- **Backend**: Axum web server for SSR
- **Styling**: TailwindCSS v4 with custom configuration
- **Build System**: cargo-leptos for building and serving

**Key Files**:
- `src/lib.rs` - Main app component with routing setup
- `src/main.rs` - Axum server setup for SSR mode
- `src/components.rs` - All page components and UI elements
- `src/data.rs` - Static data for projects, skills, and experiences
- `src/sounds.rs` - Sound effect utilities using web-sys
- `Cargo.toml` - Rust dependencies and feature flags
- `Leptos.toml` - Leptos-specific build configuration
- `tailwind.config.js` - TailwindCSS configuration with custom theme

**Component Architecture**:
- Functional components using Leptos `#[component]` macro
- Data flows through function calls to data module
- No complex state management - uses simple reactive signals
- Route-based page components (HomePage, ProjectsPage, AboutPage, ContactPage)

## Development Commands

**Build and serve (development)**:
```bash
cargo leptos watch
```

**Build for production**:
```bash
cargo leptos build --release
```

**Serve production build**:
```bash
cargo leptos serve --release
```

**Check Rust code**:
```bash
cargo check
cargo clippy
```

**Build TailwindCSS**:
```bash
npx tailwindcss -i input.css -o target/site/output.css --watch
```

## Features and Build System

**Cargo Features**:
- `ssr` (default) - Server-side rendering with Axum
- `csr` - Client-side rendering only
- `hydrate` - Client-side hydration support

**Server Configuration**:
- Development server runs on `127.0.0.1:3000`
- Hot reload port: `3001`
- Static assets served from `/assets`, `/pkg`, `/static`

**Styling System**:
- TailwindCSS v4 with custom primary/secondary color schemes
- Custom animations (fade-in, slide-up, pulse-slow)
- Custom box shadows (soft, medium, strong)
- Inter font for body text, Poppins for headings

## Key Implementation Details

**Routing**: Uses leptos_router with path-based routing
**State**: Minimal state - mostly static data from data module
**Styling**: Utility-first CSS with TailwindCSS, CDN loaded in development
**Assets**: Static assets in `/assets` directory, copied to build output
**Sounds**: Web audio API integration through web-sys bindings