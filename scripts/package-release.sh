#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
	echo "uso: package-release.sh <binario> <piattaforma>" >&2
	exit 1
fi

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BINARY="$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
PLATFORM="$2"
VERSION="$(grep -m1 '^version' "$REPO_ROOT/apps/api/Cargo.toml" | cut -d '"' -f2)"
NAME="quickshare-$VERSION-$PLATFORM"
OUT_DIR="$REPO_ROOT/build/$PLATFORM"
STAGE_DIR="$(mktemp -d)"

trap 'rm -rf "$STAGE_DIR"' EXIT

mkdir -p "$STAGE_DIR/$NAME"
cp "$BINARY" "$STAGE_DIR/$NAME/"

if [[ "$PLATFORM" == linux-* ]]; then
	install -m 0755 "$REPO_ROOT/scripts/serve.sh" "$REPO_ROOT/scripts/hotspot.sh" "$STAGE_DIR/$NAME/"
fi

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

if [[ "$BINARY" == *.exe ]]; then
	(cd "$STAGE_DIR" && zip -rq "$OUT_DIR/$NAME.zip" "$NAME")
else
	tar -C "$STAGE_DIR" -czf "$OUT_DIR/$NAME.tar.gz" "$NAME"
fi

echo "$OUT_DIR"
