#!/bin/bash
# Retrain the spam detection model and update the extension
# Usage: ./retrain.sh

set -e

PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$PROJECT_DIR"

# Ensure training data exists
if [ ! -f "data/spam.txt" ] || [ ! -f "data/safe.txt" ]; then
    echo "No training data found. Running setup..."
    bash setup_data.sh
    echo ""
fi

echo "========================================="
echo "  Spam Detection Model - Retrain"
echo "========================================="
echo ""

# Step 1: Merge raw data into training files
echo "[1/5] Merging raw data..."
if [ -d "data/raw" ] && [ "$(ls -A data/raw/ 2>/dev/null)" ]; then
    cat data/raw/spam.*.txt > data/spam.txt 2>/dev/null || true
    cat data/raw/safe.*.txt > data/safe.txt 2>/dev/null || true

    # Deduplicate
    sort -u data/spam.txt > data/spam_tmp.txt && mv data/spam_tmp.txt data/spam.txt
    sort -u data/safe.txt > data/safe_tmp.txt && mv data/safe_tmp.txt data/safe.txt

    SPAM_COUNT=$(wc -l < data/spam.txt)
    SAFE_COUNT=$(wc -l < data/safe.txt)
    echo "  Spam: $SPAM_COUNT samples"
    echo "  Safe: $SAFE_COUNT samples"
else
    echo "  No raw data found in data/raw/"
    echo "  Using existing training files..."
    SPAM_COUNT=$(wc -l < data/spam.txt 2>/dev/null || echo 0)
    SAFE_COUNT=$(wc -l < data/safe.txt 2>/dev/null || echo 0)
    echo "  Spam: $SPAM_COUNT samples"
    echo "  Safe: $SAFE_COUNT samples"
fi

# Step 2: Build and export model
echo ""
echo "[2/5] Training model..."
cargo run --release --bin export_model 2>&1 | grep -E "Training:|Model exported"

# Step 3: Compress model
echo ""
echo "[3/5] Compressing model..."
cd extension
gzip -c model.json > model.json.gz
GZ_SIZE=$(ls -lh model.json.gz | awk '{print $5}')
echo "  Compressed: $GZ_SIZE"
rm -f model.json
cd ..

# Step 4: Copy to extension
echo ""
echo "[4/5] Model updated at extension/model.json.gz"

# Step 5: Run evaluation
echo ""
echo "[5/5] Running evaluation..."
echo ""
cargo run --release --bin eval 2>&1 | tail -10

echo ""
echo "========================================="
echo "  Done! Reload extension in Firefox:"
echo "  about:debugging -> This Firefox -> Reload"
echo "========================================="
