# Portfolio

A minimalist personal portfolio website built with **Rust** and the **Leptos** web framework.

## Features

*   **Fast & Efficient**: Built with Rust and WebAssembly.
*   **Minimalist Design**: Clean UI with a focus on typography and content.
*   **Dynamic Data**: Fetches project and profile data from a remote JSON source.
*   **Responsive**: Fully validated for mobile and desktop screens.

## Prerequisites

Ensure you have the following installed:

1.  **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
2.  **Cargo Leptos**: The build tool for Leptos applications.
    ```bash
    cargo install cargo-leptos
    ```
3.  **WASM Target**:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```

## Running Locally

To start the development server with hot-reloading:

```bash
cargo leptos watch
```

The application will be available at [http://localhost:3000](http://localhost:3000).

## Building for Production

To build the optimized WebAssembly and static references:

```bash
cargo leptos build --release
```

The artifacts will be generated in `target/site`.

## Data Management

Published profile, project, and skill data comes from SurrealDB through the server-side data layer. Experience is maintained in `content/experience.json`; reviewed project copy and artwork are applied in `src/data.rs`. Configure `SURREALDB_URL`, `SURREALDB_NS`, `SURREALDB_DB`, `SURREALDB_USER`, and `SURREALDB_PASS` in the server environment. Credentials are never sent to the browser.

For a database-free development preview, set `PORTFOLIO_PREVIEW_DATA` to an absolute path to a JSON file matching `PortfolioData` in `src/data.rs`. Use a snapshot of published content and keep it under the ignored `target/` directory. This override is compiled out of release builds.

For stable visual review, use `cargo leptos serve` instead of `watch` in the command below. On this macOS setup, cargo-leptos 0.2.38 repeatedly re-emits the same CSS asset event after asset synchronization, causing a full-page reload loop. `serve` builds once and runs without the watcher or reload client; restart it after source changes. This is a preview workaround, not an upstream watcher fix.

```sh
PORTFOLIO_PREVIEW_DATA="$PWD/target/preview-data.json" \
LEPTOS_SITE_ADDR=127.0.0.1:3002 LEPTOS_RELOAD_PORT=3003 \
cargo leptos serve
```

The page is on port 3002; port 3003 is only the reload connection. No tunnel is needed. The existing Leptos nightly feature requires a compatible nightly toolchain.

## Editorial design

`public/assets/editorial.css` owns the public design system and responsive/print layouts. Shared artwork, approach, contact, and footer components are in `src/components/editorial.rs`. `/projects/:slug` displays project data; Thelivi retains the `/projects/intelligence` URL for compatibility. Notes are intentionally absent until a published article source is available; no mockup drafts or placeholder articles are exposed.

Browser code hydrates the server-rendered application rather than mounting a second copy. The HTML document is owned only by `shell()` in `src/main.rs`.
