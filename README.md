# anydef

Personal website built with Rust and Leptos, compiled to WebAssembly for GitHub Pages.

## Technology Stack

- **Leptos** - Rust web framework with reactive UI
- **WebAssembly** - Rust compiled to WASM for browser execution
- **Trunk** - Build tool and dev server for Rust WASM apps
- **Typst** - PDF generation for downloadable CV
- **YAML** - Configuration and CV data

## Prerequisites

1. Install Rust: https://rustup.rs
2. Add the WASM target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. Install Trunk:
   ```bash
   cargo install trunk
   ```
4. Install Typst:
   ```bash
   brew install typst
   # or: cargo install typst-cli
   ```

## Development

```bash
cd crates/web
trunk serve
```

Opens at http://localhost:8080. The CV PDF is automatically generated on each build.

## Build for Production

```bash
cd crates/web
trunk build --release
```

Output goes to `docs/`. The pre-build hook automatically compiles `cv.typ` to `cv.pdf`.

## Configuration

| File | Purpose |
|------|---------|
| `cv.yaml` | CV data (single source of truth for web and PDF) |
| `cv.typ` | Typst template for PDF styling |
| `main.yaml` | Website text (header, hero, footer) |

## Project Structure

```
.
├── cv.yaml                 # CV data
├── cv.typ                  # Typst PDF template
├── main.yaml               # Website text
├── crates/web/
│   ├── Trunk.toml          # Build config with pre-build hook
│   ├── index.html
│   ├── style.css
│   └── src/
│       ├── main.rs
│       ├── config.rs
│       └── components/
└── docs/                   # Build output (GitHub Pages)
```

## Deploy to GitHub Pages

1. Build: `cd crates/web && trunk build --release`
2. Commit and push `docs/`
3. GitHub Settings > Pages > Deploy from branch `main` folder `/docs`
