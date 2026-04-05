#!/bin/bash
# Setup training data from example files
# Usage: ./setup_data.sh

set -e

PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$PROJECT_DIR"

echo "Setting up training data from examples..."

if [ ! -f "data/spam.txt" ] && [ -f "data/spam.example.txt" ]; then
    cp data/spam.example.txt data/spam.txt
    echo "  Created data/spam.txt from example"
else
    echo "  data/spam.txt already exists, skipping"
fi

if [ ! -f "data/safe.txt" ] && [ -f "data/safe.example.txt" ]; then
    cp data/safe.example.txt data/safe.txt
    echo "  Created data/safe.txt from example"
else
    echo "  data/safe.txt already exists, skipping"
fi

if [ ! -d "data/raw" ]; then
    mkdir -p data/raw
    echo "  Created data/raw/ directory"
fi

echo ""
echo "Done. Edit data/spam.txt and data/safe.txt with your own data."
echo "Then run ./retrain.sh to train the model."
