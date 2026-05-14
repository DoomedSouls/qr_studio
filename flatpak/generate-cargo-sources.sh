#!/usr/bin/env bash
# Generate cargo-sources.json for Flatpak build from Cargo.lock
#
# This script downloads the flatpak-cargo-generator.py from the official
# flatpak-builder-tools repo and runs it against the project's Cargo.lock
# to produce a vendored source manifest that flatpak-builder can use.
#
# Usage:
#   cd flatpak/
#   ./generate-cargo-sources.sh
#
# Requires: python3, cargo

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CARGO_LOCK="$PROJECT_ROOT/Cargo.lock"
OUTPUT="$SCRIPT_DIR/cargo-sources.json"
GENERATOR="$SCRIPT_DIR/flatpak-cargo-generator.py"

echo "=== Flatpak Cargo Source Generator ==="

# Check prerequisites
if [ ! -f "$CARGO_LOCK" ]; then
    echo "ERROR: Cargo.lock not found at $CARGO_LOCK"
    exit 1
fi

if ! command -v python3 &>/dev/null; then
    echo "ERROR: python3 not found"
    exit 1
fi

# Download the generator if not present
if [ ! -f "$GENERATOR" ]; then
    echo "Downloading flatpak-cargo-generator.py..."
    curl -fsSL \
        "https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py" \
        -o "$GENERATOR"
    chmod +x "$GENERATOR"
fi

# Generate sources
echo "Generating cargo-sources.json from Cargo.lock..."
python3 "$GENERATOR" "$CARGO_LOCK" -o "$OUTPUT"

echo "Done: $OUTPUT"
echo ""
echo "Now you can build the Flatpak:"
echo "  cd $SCRIPT_DIR"
echo "  flatpak-builder --user --install --force-clean build-dir io.github.SlobCoder.qr_studio.yaml"
