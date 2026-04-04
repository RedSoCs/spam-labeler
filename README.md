# Spam Detection Tokenizer

Terminal application for detecting spam/casino content using ML-based text classification with GPT-4o tokenization.

## Features

- **GPT-4o Tokenizer** — Uses `o200k_base` (tiktoken-rs) for accurate token counting
- **Bernoulli Naive Bayes** — Trigram + quadgram vectorizer with Lidstone smoothing (α=0.1)
- **Keyword Weighting** — Manual boost for high-signal spam keywords
- **Hard Rules** — Currency symbol and spam link detection
- **Model Persistence** — Trained model saved to `model.bin`, reused across runs
- **TUI** — Real-time ratatui interface with token analysis and ML prediction
- **Firefox Extension** — Collect real-world training data from Google search results

## Quick Start

```bash
# Build
cargo build --release

# Run TUI application
cargo run --release
```

### Controls

| Key | Action |
|-----|--------|
| Type | Enter text in input field |
| Enter | Analyze text |
| Esc | Cancel input |
| q | Quit |

## Project Structure

```
test-token/
├── src/
│   ├── main.rs              # TUI application
│   └── bin/
│       ├── eval.rs           # Evaluate model against raw data
│       └── export_model.rs   # Export model for Firefox extension
├── extension/
│   ├── manifest.json         # Firefox extension manifest
│   ├── background.js         # Background script (persistent model)
│   ├── content.js            # Content script (UI + highlighting)
│   └── model.json.gz         # Compressed ML model for extension
├── data/
│   ├── spam.txt              # Spam training samples
│   ├── safe.txt              # Safe training samples
│   └── raw/                  # Collected data from extension
│       ├── spam.[timestamp].txt
│       └── safe.[timestamp].txt
├── retrain.sh                # Retrain and update extension model
├── model.bin                 # Trained model (binary)
└── model_version.txt         # Version counter
```

## Training Pipeline

### 1. Collect Data via Firefox Extension

1. Open Firefox → `about:debugging` → **This Firefox** → **Load Temporary Add-on**
2. Select `extension/manifest.json`
3. Browse Google Search — results are auto-scanned
4. Click **🔴 SPAM** or **🟢 SAFE** to download labeled data to `~/Downloads/`

### 2. Merge Raw Data

```bash
# Move downloaded files to data/raw/
mv ~/Downloads/spam.*.txt data/raw/
mv ~/Downloads/safe.*.txt data/raw/
```

### 3. Retrain

```bash
./retrain.sh
```

This merges raw data, deduplicates, trains the model, evaluates accuracy, and updates `extension/model.json.gz`.

### 4. Reload Extension

Open `about:debugging` → find "Spam Labeler" → click **Reload**.

## Model Architecture

| Component | Value |
|-----------|-------|
| **Tokenizer** | o200k_base (GPT-4o) |
| **Classifier** | Bernoulli Naive Bayes |
| **Features** | Tokens + Trigrams + Quadgrams |
| **Smoothing** | Lidstone α=0.1 |
| **Hard Rules** | Currency symbols ($€£฿), Spam links (line.me, @line) |
| **Keyword Boost** | +5.0 per high-signal word |

### High-Signal Keywords

Thai casino/scam keywords: `สล็อต`, `แตกง่าย`, `บาคาร่า`, `วอเลท`, `wallet`, `เว็บตรง`, `ไม่ผ่านเอเย่นต์`, `เครดิตฟรี`, `ฝาก15รับ100`, `pg slot`, `pgslot`, `joker slot`, `jackpot`, `cashback`, `คืนยอดเสีย`, etc.

## Versioning

Model versions follow the format **`v0.{N}`** where `N = sample_count / 10`:

| Samples | Version | Meaning |
|---------|---------|---------|
| 100 | v0.10 | Initial training |
| 458 | v0.45 | Current |
| 1000 | v0.100 | 1K samples |
| 2500 | v0.250 | 2.5K samples |

Each time you run `./retrain.sh`, the version increments based on the total number of training samples. This makes it easy to track model quality — higher versions mean more training data.

## Evaluation

```bash
cargo run --release --bin eval
```

Tests the model against all files in `data/raw/` and reports accuracy.

## Extension Installation

1. Firefox → `about:config` → set `xpinstall.signatures.required` = `false`
2. `about:debugging` → **This Firefox** → **Load Temporary Add-on**
3. Select `extension/manifest.json`

The extension runs a background script that keeps the model in memory across page reloads. Each search result's h3 title turns **yellow** if detected as spam.
