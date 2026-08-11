#!/bin/sh
set -eu

repository_root=$(pwd)
binary_directory="$repository_root/bin"
binary_path="$binary_directory/herdr-palette"
manifest="$repository_root/herdr-plugin.toml"

if [ -e "$binary_path" ]; then
    echo "bin/herdr-palette already exists; using development binary."
    exit 0
fi

version=$(sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)"[[:space:]]*$/\1/p' "$manifest" | head -n 1)
if [ -z "$version" ]; then
    echo "could not read plugin version from $manifest" >&2
    exit 1
fi

platform="$(uname -s):$(uname -m)"
case "$platform" in
    Linux:x86_64)
        target=x86_64-unknown-linux-gnu
        ;;
    Darwin:arm64)
        target=aarch64-apple-darwin
        ;;
    Darwin:x86_64)
        target=x86_64-apple-darwin
        ;;
    *)
        echo "unsupported platform: $platform" >&2
        exit 1
        ;;
esac

archive=$(mktemp "${TMPDIR:-/tmp}/herdr-palette.XXXXXX")
cleanup() {
    rm -f "$archive"
}
trap cleanup EXIT HUP INT TERM

url="https://github.com/cesarferreira/herdr-palette/releases/download/v$version/herdr-palette-$target.tar.gz"
mkdir -p "$binary_directory"
curl -fsSL -o "$archive" "$url"
tar -xzf "$archive" -C "$binary_directory"
chmod +x "$binary_path"
