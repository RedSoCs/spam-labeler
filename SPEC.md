# Specification — Spam Detection System

## Overview

A Rust terminal application that classifies text as spam/casino or safe using ML inference, combined with a Firefox extension for collecting real-world training data from Google search results.

---

## 1. Architecture

### Three-Layer Design

```
┌─────────────────────────────────────┐
│           TUI (ratatui)             │  Input, token display, analysis
├─────────────────────────────────────┤
│        ML Engine (smartcore)         │  Bernoulli NB, vectorizer, prediction
├─────────────────────────────────────┤
│       Data Ingestion                │  Training data loader, file I/O
└─────────────────────────────────────┘
         ↕
  Firefox Extension (background + content script)
         ↕
  data/raw/  (labeled spam/safe samples)
```

### Crate Stack

| Component | Crate | Purpose |
|-----------|-------|---------|
| **Terminal UI** | `ratatui` | Layouts, lists, blocks |
| **User Input** | `tui-input` | Text fields, cursor movement |
| **Tokenization** | `tiktoken-rs` | GPT-4o tokenizer (o200k_base) |
| **Terminal** | `crossterm` | Raw mode, events, screen |
| **Serialization** | `bincode`, `serde` | Model save/load |
| **ML** | Custom BernoulliNB | Text classification |

---

## 2. ML Pipeline

### 2.1 Data Ingestion

- **Training files**: `data/spam.txt` (labeled spam), `data/safe.txt` (labeled safe)
- **Format**: One sample per line, plain text
- **Collection**: Firefox extension downloads `spam.[timestamp].txt` / `safe.[timestamp].txt` to Downloads

### 2.2 Feature Extraction

The vectorizer builds vocabulary from training data using:

1. **Single tokens** — whitespace-split, cleaned of whitespace
2. **Trigrams (3 chars)** — catches Thai subword patterns
3. **Quadgrams (4 chars)** — captures longer Thai spam phrases

### 2.3 Classifier

**Bernoulli Naive Bayes** with:
- **Lidstone smoothing** — α=0.1 (reduces overconfidence for large vocabularies)
- **Balanced classes** — Equal prior probability for spam/safe
- **Binary features** — Presence/absence of each vocabulary item

### 2.4 Feature Engineering

| Feature | Type | Description |
|---------|------|-------------|
| **Currency symbols** | Hard rule | `$ € £ ฿ ¥ ₹ ₽ ₿ ₮ ₩ ₱ ₫` → auto-spam (100%) |
| **Spam links** | Hard rule | `line.me`, `@line`, `lin.ee` → auto-spam (95%) |
| **Keyword weight** | Manual boost | +5.0 per high-signal word per sample |
| **Thai/EN ratio** | Supplementary | Mixed Thai+English content signal |

### 2.5 Prediction

```
score = class_log_prior + Σ(feature_log_prob[i][j] for matched features)
      + Σ(log(1 - exp(feature_log_prob[i][j])) for unmatched features)

probability = softmax(scores)[spam_class]
```

---

## 3. Data Quality Rules (V2)

### 3.1 The 80/20 Rule

Machine learning is 80% data, 20% code. Key principles:

- **Diversify safe dataset** — Add 500+ lines of real Thai text (news, Wikipedia, business emails, tech articles)
- **Avoid template spam** — Real-world spam uses character hacking (spaced chars, dashes, dots)
- **Length normalization** — Ensure safe samples have similar character counts to spam
- **Deduplication** — Always `sort -u` before training

### 3.2 Noise Injection for Spam

Add samples with obfuscated patterns:
- `ส ล็ อ ต` (added spaces)
- `ส.ล็อต` (added punctuation)
- `p-g-s-l-o-t` (dashes)
- `สล็อต-เว็บตรง-คาสิโนออนไลน์` (hyphens)

### 3.3 Balanced Training Data

- Target ratio: ~1:1 spam to safe (use oversampling if needed)
- Minimum 100 samples per class
- Include both short (3-5 words) and long (20-30 words) samples

---

## 4. Technical Improvements

### 4.1 Thai Tokenization

Thai has no word boundaries. The approach:
- Use **trigrams + quadgrams** instead of bigrams
- Trigrams catch common Thai patterns (`เครดิต`, `ทดลอง`)
- Quadgrams catch longer spam phrases (`เครดิตฟรี`, `ทดลองเล่น`)

### 4.2 Lidstone Smoothing

Standard Laplace smoothing (α=1.0) is too strong for large vocabularies. Use **α=0.1**:

```rust
let val: f64 = (feature_counts[i][j] + 0.1) / (class_counts[i] + 0.2);
```

### 4.3 Model Persistence

- **Save**: Serialize model + vocabulary via `bincode` → `model.bin`
- **Load**: Deserialize on startup, skip retraining if file exists
- **Export**: JSON format for Firefox extension (`model.json.gz`)

### 4.4 CPU Efficiency

- **Event-driven rendering** — Only redraw when state changes
- **Poll timeout** — 200ms between event polls (not busy-looping)
- **Lazy evaluation** — Prediction runs only on user action

---

## 5. Extension Design

### 5.1 Background Script

- Loads `model.json.gz` once on startup (persistent background page)
- Decompresses via `DecompressionStream("gzip")`
- Handles prediction requests from content scripts via `chrome.runtime.onMessage`
- Model persists across page reloads and navigation

### 5.2 Content Script

- Injects toolbar at top of Google search pages
- Adds checkbox next to each result title
- Sends descriptions to background for prediction
- Highlights spam result titles in **yellow** (`#d4a017`)
- Shows model version in status indicator (e.g., `v0.45`)

### 5.3 Data Collection Flow

```
User clicks SPAM/SAFE
    → Extension collects all descriptions
    → Downloads as spam.[timestamp].txt or safe.[timestamp].txt
    → User moves files to data/raw/
    → Run ./retrain.sh
    → Model updates
```

---

## 6. Model Versioning

### Format: `v0.{N}`

Where `N = total_training_samples / 10`

| Version | Samples | Description |
|---------|---------|-------------|
| v0.10 | 100 | Initial training |
| v0.45 | 458 | Current default |
| v0.100 | 1,000 | 1K milestone |
| v0.250 | 2,500 | Large dataset |

### Tracking

- `model_version.txt` — Auto-incremented by `export_model.rs`
- Extension status dot shows current version
- Console logs: `[BG] Model v0.45 loaded: 32447 features`

### Versioning Notes

- **Higher version = more data = better accuracy** (generally)
- Version reflects **sample count**, not feature count
- Each retrain after adding new data produces a new version
- Use `./retrain.sh` to ensure version is consistent across TUI and extension
- Track version in commit messages: `Retrain → v0.52 (520 samples)`

---

## 7. File I/O

### Training Data Format

```
data/spam.txt:
สล็อตทดลองฟรี ระบบสมาชิกออกแบบมา...
สมัครสมาชิกวันนี้ รับโบนัสทันที...

data/safe.txt:
Hello, how are you today?
ร้านอาหารอร่อยใกล้บ้าน...
```

### Collected Data Format

```
data/raw/spam.1712345678.txt:
Description line 1 from search result
Description line 2 from search result
```

### Export Pipeline

```bash
# 1. Merge raw files
cat data/raw/spam.*.txt > data/spam.txt
cat data/raw/safe.*.txt > data/safe.txt

# 2. Deduplicate
sort -u data/spam.txt > tmp && mv tmp data/spam.txt
sort -u data/safe.txt > tmp && mv tmp data/safe.txt

# 3. Export and compress
cargo run --release --bin export_model
cd extension && gzip -c model.json > model.json.gz && rm model.json
```

---

## 8. Evaluation

Run `cargo run --release --bin eval` to:

1. Load training data
2. Build vectorizer and train model
3. Test against all files in `data/raw/`
4. Report accuracy with per-sample breakdown

Expected accuracy: **>95%** with balanced data (500+ samples per class).
