#!/bin/bash
# Generate PNG icons from SVG logo
# Requires: inkscape or imagemagick

set -e

SIZES="16 32 48 128 256"
SVG_SOURCE="resources/logo-static.svg"
OUTPUT_DIR="resources/icons"

if [ ! -f "$SVG_SOURCE" ]; then
    echo "Error: SVG source not found: $SVG_SOURCE"
    exit 1
fi

mkdir -p "$OUTPUT_DIR"

for size in $SIZES; do
    mkdir -p "$OUTPUT_DIR/${size}x${size}"

    # Using inkscape (preferred)
    if command -v inkscape &> /dev/null; then
        inkscape "$SVG_SOURCE" \
            --export-type=png \
            --export-filename="$OUTPUT_DIR/${size}x${size}/asteroid-browser.png" \
            --export-width=$size \
            --export-height=$size
    # Fallback to ImageMagick
    elif command -v convert &> /dev/null; then
        convert -background none \
            "$SVG_SOURCE" \
            -resize ${size}x${size} \
            "$OUTPUT_DIR/${size}x${size}/asteroid-browser.png"
    else
        echo "Error: Neither inkscape nor imagemagick found"
        exit 1
    fi

    echo "Generated ${size}x${size} icon"
done

# Generate favicon.ico (multiple sizes embedded)
if command -v convert &> /dev/null; then
    convert "$OUTPUT_DIR/16x16/asteroid-browser.png" \
            "$OUTPUT_DIR/32x32/asteroid-browser.png" \
            "$OUTPUT_DIR/48x48/asteroid-browser.png" \
            "$OUTPUT_DIR/favicon.ico"
    echo "Generated favicon.ico"
fi

echo "Icon generation complete!"
