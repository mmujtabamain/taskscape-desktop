#!/usr/bin/env bash
#
# Builds the release binaries and assembles macOS .app bundles into ./dist:
#
#   dist/Taskscape.app                         (main window — has a Dock icon)
#     Contents/MacOS/taskscape
#     Contents/Info.plist
#     Contents/Library/LoginItems/Taskscape Tray.app   (tray — LSUIElement, no Dock icon)
#       Contents/MacOS/taskscape-tray
#       Contents/Info.plist
#
# The tray bundle is nested inside the main bundle so the user launches one app,
# and the main app can find the tray at a stable relative path (see launch.rs).
#
# Usage: ./make-app.sh && open dist/Taskscape.app

set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

DIST="$ROOT/dist"
MAIN_APP="$DIST/Taskscape.app"
TRAY_APP="$MAIN_APP/Contents/Library/LoginItems/Taskscape Tray.app"

echo "==> cargo build --release"
cargo build --release

echo "==> assembling bundles in $DIST"
rm -rf "$MAIN_APP"
mkdir -p "$MAIN_APP/Contents/MacOS" "$MAIN_APP/Contents/Resources"
mkdir -p "$TRAY_APP/Contents/MacOS" "$TRAY_APP/Contents/Resources"

# Main bundle
cp "$ROOT/target/release/taskscape" "$MAIN_APP/Contents/MacOS/taskscape"
cp "$ROOT/main_src/macos/Info.plist" "$MAIN_APP/Contents/Info.plist"

# Nested tray bundle (LSUIElement → no Dock icon)
cp "$ROOT/target/release/taskscape-tray" "$TRAY_APP/Contents/MacOS/taskscape-tray"
cp "$ROOT/tray_src/macos/Info.plist" "$TRAY_APP/Contents/Info.plist"

# Bundle fonts as resources (optional; the binaries also embed them).
if [ -d "$ROOT/assets" ]; then
  cp -R "$ROOT/assets" "$MAIN_APP/Contents/Resources/assets"
fi

echo "==> done"
echo "    open \"$MAIN_APP\""
