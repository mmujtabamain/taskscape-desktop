#!/usr/bin/env bash
# Regenerate the embedded Material Symbols Sharp subset from the full font.
#
# Reads used-icons.txt, looks up each name's codepoint in codepoints.txt, pins the
# variable font to a static default instance, and subsets to just those glyphs.
# The result (MaterialSymbolsSharp-subset.ttf) is what the binaries embed.
#
# Requires fonttools. Once its venv exists, NO internet is needed to add/remove
# icons — edit used-icons.txt and re-run. To (re)create the venv once:
#   python3 -m venv ~/.cache/taskscape-fonttools-venv
#   ~/.cache/taskscape-fonttools-venv/bin/pip install fonttools
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
VENV="${FONTTOOLS_VENV:-$HOME/.cache/taskscape-fonttools-venv}"
FULL="$DIR/MaterialSymbolsSharp.ttf"
CODEPOINTS="$DIR/codepoints.txt"
USED="$DIR/used-icons.txt"
OUT="$DIR/MaterialSymbolsSharp-subset.ttf"

[ -x "$VENV/bin/pyftsubset" ] || { echo "fonttools venv missing at $VENV (see header)" >&2; exit 1; }

codes=()
while read -r raw; do
  name="${raw%%#*}"; name="$(echo "$name" | xargs)"   # strip comment + trim
  [ -z "$name" ] && continue
  cp="$(awk -v n="$name" '$1==n{print $2; exit}' "$CODEPOINTS")"
  [ -z "$cp" ] && { echo "WARN: '$name' not in codepoints.txt" >&2; continue; }
  cp_upper="$(echo "$cp" | tr '[:lower:]' '[:upper:]')"
  codes+=("U+${cp_upper}")
done < "$USED"

unicodes="$(IFS=,; echo "${codes[*]}")"
echo "Subsetting ${#codes[@]} glyphs: $unicodes"

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
"$VENV/bin/python" -m fontTools.varLib.instancer "$FULL" \
  wght=400 opsz=24 GRAD=0 FILL=0 -o "$TMP/static.ttf" >/dev/null
"$VENV/bin/pyftsubset" "$TMP/static.ttf" \
  --unicodes="$unicodes" \
  --layout-features='' --no-hinting --desubroutinize \
  --output-file="$OUT"

echo "Wrote $OUT ($(wc -c <"$OUT") bytes)"
