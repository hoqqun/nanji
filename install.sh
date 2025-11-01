#!/usr/bin/env sh
set -e

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

# Create a temp directory for building
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Clone the latest main branch
git clone --depth 1 "https://github.com/$REPO.git" .
echo "📦 Cloned repository"

# Build and install using Cargo
cargo install --path .

# Cleanup
cd -
rm -rf "$TMP_DIR"

echo "✅ $BINARY_NAME installed successfully!"
echo ""
echo "Run it now with:"
echo "  $BINARY_NAME"
