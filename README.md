# anydef

A personal website built with Rust and Leptos, compiled to WebAssembly.

## Prerequisites

1. Install Rust: https://rustup.rs
2. Add the WASM target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. Install Trunk (build tool for Rust WASM apps):
   ```bash
   cargo install trunk
   ```

## Development

Run the development server with hot reload:

```bash
trunk serve
```

Open http://localhost:8080 in your browser.

## Build for Production

Build optimized static files:

```bash
trunk build --release
```

Output will be in the `docs/` directory.

## Deploy to GitHub Pages

### Option 1: Deploy from `docs/` folder

1. Build the project:
   ```bash
   trunk build --release
   ```
2. Commit and push the `docs/` folder
3. In GitHub repository settings:
   - Go to Settings > Pages
   - Source: "Deploy from a branch"
   - Branch: `main` (or your branch)
   - Folder: `/docs`
   - Save

### Option 2: GitHub Actions (automated)

Create `.github/workflows/deploy.yml`:

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [main]

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - name: Install Trunk
        run: cargo install trunk
      - name: Build
        run: trunk build --release --public-url /anydef/
      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: docs

  deploy:
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

Note: If deploying to a user site (username.github.io), remove `--public-url /anydef/` from the build command.

## Project Structure

```
.
├── Cargo.toml          # Rust dependencies
├── Trunk.toml          # Trunk build configuration
├── index.html          # HTML template
├── rust-toolchain.toml # Rust toolchain config
└── src/
    └── main.rs         # Application code
```
