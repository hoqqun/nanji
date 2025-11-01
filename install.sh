#!/usr/bin/env sh
# Portable one-line installer for nanji
# - POSIX sh compatible (works with `| sh`)
# - Prefers cargo (or cargo-binstall if available)
# - Falls back gracefully with clear guidance

set -eu

export PATH="$HOME/.cargo/bin:$PATH"

REPO="hoqqun/nanji"
BINARY_NAME="nanji"

# Detect if cargo is available (POSIX sh compatible)
if ! command -v cargo >/dev/null 2>&1; then
  # try well-known default install path
  if [ -x "$HOME/.cargo/bin/cargo" ]; then
    PATH="$HOME/.cargo/bin:$PATH"
    export PATH
  else
    echo "❌ Cargo not found. Please install Rust first:"
    echo "   https://www.rust-lang.org/tools/install"
    exit 1
  fi
fi

echo "🦀 Installing $BINARY_NAME from $REPO..."

# Prefer cargo-binstall if available (fast prebuilt binaries when provided)
if command -v cargo-binstall >/dev/null 2>&1; then
  echo "⚡ Using cargo-binstall (prebuilt if available)"
  if cargo binstall "$BINARY_NAME" --no-confirm; then
    echo "✅ $BINARY_NAME installed via cargo-binstall"
    echo "Run: $BINARY_NAME"
    exit 0
  fi
  echo "ℹ️ cargo-binstall failed or no prebuilt found. Falling back to cargo install."
fi

# Use cargo install directly from Git repository (no crates.io required)
echo "🧰 Using cargo install from Git"
cargo install --git "https://github.com/$REPO.git" --branch main --force "$BINARY_NAME"

echo "✅ $BINARY_NAME installed successfully!"
echo ""
echo "Run it now with:"
echo "  $BINARY_NAME"
