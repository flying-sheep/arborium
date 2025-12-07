#!/usr/bin/env bash
# DEPRECATED: Use `cargo xtask deploy-website --version <version>` instead
#
# This script was used by the old CI workflow to prepare static files.
# The new workflow deploys directly to gh-pages from local.

set -euo pipefail

echo "WARNING: This script is deprecated. Use 'cargo xtask deploy-website --version <version>' instead." >&2

# Determine output directory (default to _site for CI compatibility)
OUT_DIR="${1:-_site}"

echo "==> Preparing website in $OUT_DIR"

# Create output directory
mkdir -p "$OUT_DIR"

# Copy static assets from demo/
echo "==> Copying static assets..."
cp demo/index.html "$OUT_DIR/"
cp demo/styles.css "$OUT_DIR/"
cp demo/registry.json "$OUT_DIR/"
cp demo/plugins.json "$OUT_DIR/"
cp demo/*.woff2 "$OUT_DIR/"
cp -r demo/pkg "$OUT_DIR/"
cp -r demo/samples "$OUT_DIR/"

# Copy WASM plugins
echo "==> Copying WASM plugins..."
mkdir -p "$OUT_DIR/plugins"

# From demo/plugins (if exists)
if [ -d demo/plugins ]; then
    cp -r demo/plugins/* "$OUT_DIR/plugins/" 2>/dev/null || true
fi

# From dist/plugins (built WASM files)
if [ -d dist/plugins ]; then
    cp -r dist/plugins/* "$OUT_DIR/plugins/" 2>/dev/null || true
fi

echo "==> Website prepared in $OUT_DIR"
echo "    To serve locally: python3 -m http.server -d $OUT_DIR"
