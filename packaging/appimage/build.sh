#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
IMAGE_NAME="supernova-appimage-builder:ubuntu22.04"
CACHE_VOL="supernova-appimage-cargo-cache"

REBUILD_IMAGE=0
for arg in "$@"; do
  case "$arg" in
    --rebuild-image)
      REBUILD_IMAGE=1
      ;;
    -h|--help)
      echo "Usage: $0 [--rebuild-image]"
      exit 0
      ;;
    *)
      echo "Unknown option: $arg" >&2
      echo "Usage: $0 [--rebuild-image]" >&2
      exit 1
      ;;
  esac
done

if [ "$REBUILD_IMAGE" -eq 1 ]; then
  podman build --no-cache -t "$IMAGE_NAME" -f "$SCRIPT_DIR/Containerfile" "$SCRIPT_DIR"
elif ! podman image exists "$IMAGE_NAME"; then
  podman build -t "$IMAGE_NAME" -f "$SCRIPT_DIR/Containerfile" "$SCRIPT_DIR"
fi

podman volume exists "$CACHE_VOL" || podman volume create "$CACHE_VOL" >/dev/null

podman run --rm \
  --security-opt label=disable \
  -v "$REPO_ROOT:/src" \
  -v "$CACHE_VOL:/cache" \
  "$IMAGE_NAME" \
  /src/packaging/appimage/build-in-container.sh
