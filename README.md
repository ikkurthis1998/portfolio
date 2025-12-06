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

The portfolio content is driven by `assets/data.json`. To update your projects, skills, or experience, simply edit this file. The application fetches this data dynamically at runtime.
