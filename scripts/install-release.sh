#!/bin/sh
set -eu

mkdir -p bin

if [ -e bin/herdr-palette ]; then
    echo "bin/herdr-palette already exists; using development binary."
    exit 0
fi

cargo build --release
cp target/release/herdr-palette bin/herdr-palette
echo "Built development binary at bin/herdr-palette."
