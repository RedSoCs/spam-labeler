#!/bin/bash
# Sign Firefox extension using web-ext
# Requires: npm, web-ext, AMO API credentials

set -e

EXT_DIR="$(cd "$(dirname "$0")" && pwd)"
OUTPUT_DIR="$EXT_DIR/dist"

echo "=== Firefox Extension Signer ==="
echo ""

# Check if web-ext is installed
if ! command -v web-ext &> /dev/null; then
    echo "Installing web-ext..."
    npm install -g web-ext
fi

# Check for credentials
if [ -z "$WEB_EXT_API_KEY" ] || [ -z "$WEB_EXT_API_SECRET" ]; then
    echo ""
    echo "Missing AMO API credentials."
    echo ""
    echo "To get credentials:"
    echo "  1. Go to https://addons.mozilla.org/developers/addon/api/key/"
    echo "  2. Log in and generate API credentials"
    echo "  3. Set environment variables:"
    echo ""
    echo "  export WEB_EXT_API_KEY='your-issuer-key'"
    echo "  export WEB_EXT_API_SECRET='your-secret-key'"
    echo ""
    echo "Alternative (no signing needed for testing):"
    echo "  1. Open Firefox → about:config"
    echo "  2. Set xpinstall.signatures.required = false"
    echo "  3. Load extension via about:debugging (temporary)"
    echo ""
    exit 1
fi

# Build and sign
echo "Signing extension..."
mkdir -p "$OUTPUT_DIR"

web-ext sign \
    --source-dir "$EXT_DIR" \
    --artifacts-dir "$OUTPUT_DIR" \
    --api-key "$WEB_EXT_API_KEY" \
    --api-secret "$WEB_EXT_API_SECRET" \
    --channel=unlisted

echo ""
echo "Signed extension saved to: $OUTPUT_DIR/"
echo "Install by opening the .xpi file in Firefox"
