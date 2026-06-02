#!/usr/bin/env bash
#
# Dev runner: builds the workspace and launches the main app, which auto-starts
# the tray service from the sibling `taskscape-tray` binary (see launch.rs).
#
# On exit (Ctrl-C or the main window closing) it stops the background tray and
# removes the IPC socket, so the next run starts clean.
#
# Usage:
#   ./run-dev.sh            # debug build, run main app (spawns tray)
#   ./run-dev.sh --release  # release build
#   ./run-dev.sh tray       # run ONLY the tray service (no main window)

set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

PROFILE_FLAG=""
PROFILE_DIR="debug"
TARGET="main" # main | tray

for arg in "$@"; do
  case "$arg" in
    --release) PROFILE_FLAG="--release"; PROFILE_DIR="release" ;;
    tray) TARGET="tray" ;;
    main) TARGET="main" ;;
    *) echo "Unknown argument: $arg" >&2; exit 1 ;;
  esac
done

SOCKET="${TMPDIR:-/tmp}/taskscape.sock"

cleanup() {
  # Stop the background tray we (or the main app) started, and clear the socket.
  pkill -f "target/$PROFILE_DIR/taskscape-tray" 2>/dev/null || true
  rm -f "$SOCKET" 2>/dev/null || true
}
trap cleanup EXIT INT TERM

echo "==> building workspace ($PROFILE_DIR)"
cargo build $PROFILE_FLAG

# Start from a clean slate so the launcher's socket probe is accurate.
cleanup

if [ "$TARGET" = "tray" ]; then
  echo "==> running taskscape-tray (background service only)"
  exec cargo run $PROFILE_FLAG --bin taskscape-tray
else
  echo "==> running taskscape (main app — auto-starts the tray service)"
  exec cargo run $PROFILE_FLAG --bin taskscape
fi
