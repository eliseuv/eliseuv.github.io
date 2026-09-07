# Personal website + resume — task runner
# Run `just` or `just --list` to see all recipes.

default:
    @just --list

# --- Development ---------------------------------------------------------

# Serve the Zola site locally with live reload.
dev:
    zola serve

# Validate internal/external links and structure without building output.
check:
    zola check

# --- Building -------------------------------------------------------------

# Build everything: WASM simulations, resume PDF, then the Zola site.
build:
    nix run .#default

# Build only the WASM simulations into static/wasm.
build-simulations:
    nix run .#simulations

# Build only the resume PDF into static/resume.pdf.
build-resume:
    nix run .#resume

# Build only the Zola site (expects simulations/resume already built).
build-zola:
    nix run .#zola

# Live-preview the resume PDF as you edit the Typst source.
watch-resume:
    typst watch --root . resume/resume.typ static/resume.pdf

# --- Formatting & linting --------------------------------------------------

# Format Rust and Typst sources.
fmt:
    cd simulations && cargo fmt
    typstyle -i resume/*.typ

# Check formatting without writing changes.
fmt-check:
    cd simulations && cargo fmt --check
    typstyle --check resume/*.typ

# Lint the Rust simulation crate.
clippy:
    cd simulations && cargo clippy --target wasm32-unknown-unknown -- -D warnings

# --- Cleaning ---------------------------------------------------------------

# Remove all build outputs.
clean:
    rm -rf public
    rm -rf static/wasm
    rm -f static/resume.pdf
    cd simulations && cargo clean
