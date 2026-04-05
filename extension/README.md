# Spam Labeler — Firefox Extension

A Firefox add-on that scans Google search results, highlights spam titles in **yellow**, and lets you collect training data with **🔴 SPAM** / **🟢 SAFE** buttons.

## How It Works

```
┌────────────────────────────────────────────────────┐
│  Toolbar (top of page)                              │
│  🏷️ Label:  🔴 SPAM   🟢 SAFE   ■v0.68   (0 sel)  │
├────────────────────────────────────────────────────┤
│                                                      │
│  [☑]  Result 1 title ← yellow if spam               │
│       Description text scanned by ML model           │
│                                                      │
│  [ ]  Result 2 title ← normal color                 │
│       Description text scanned by ML model           │
│                                                      │
└────────────────────────────────────────────────────┘
```

### Spam Detection & Highlighting

1. The extension loads a compressed ML model (`model.json.gz`) into a **persistent background script**
2. On each Google search, every result description is sent to the model for classification
3. If **spam probability > 50%**, the **h3 title turns yellow** (`#d4a017`)
4. A checkbox appears on the right of each result for manual labeling

### 🟢 SAFE / 🔴 SPAM Buttons — Training Data Collection

The two buttons on the toolbar let you **download labeled training data**:

| Scenario | What gets downloaded |
|----------|---------------------|
| **No checkboxes checked** | ALL results → downloaded as the label you clicked |
| **Some checkboxes checked** | Checked items → downloaded as the **opposite** label<br>Unchecked items → downloaded as the **label you clicked** |

This means:
- Click **🔴 SPAM** with no checks → everything on page is labeled spam
- Click **🟢 SAFE** with no checks → everything on page is labeled safe
- Check a result, then click **🔴 SPAM** → checked = safe, unchecked = spam

**Why the opposite label?** Because if you're about to mark a page as spam but one result looks legitimate, you check it so it gets saved as safe instead.

### Downloaded Files

Files are downloaded to your `~/Downloads/` folder:

```
spam.1712345678.txt    ← spam-labeled data
safe.1712345678.txt    ← safe-labeled data
```

Each file contains **one description per line** — ready for training:

```
สล็อตทดลองฟรี ระบบสมาชิกออกแบบมาเพื่อให้คุณเข้าถึงสิทธิประโยชน์
คาสิโนออนไลน์ที่ดีที่สุด สมัครสมาชิกฟรี รับโบนัส 100%
Welcome bonus! Deposit now get 200% match up to $500
```

## Install

1. Open Firefox → `about:config`
2. Set `xpinstall.signatures.required` = **`false`**
3. Go to `about:debugging` → **This Firefox** → **Load Temporary Add-on**
4. Select this directory's `manifest.json`

The status indicator (■v0.68) in the toolbar turns **green** when the model is loaded successfully.

## Training Workflow

### Step 1: Collect Data

Browse Google with the extension active. When you see spam or safe results, use the toolbar buttons to download them.

### Step 2: Move to Project

```bash
# Move downloaded files to data/raw/
mv ~/Downloads/spam.*.txt /path/to/project/data/raw/
mv ~/Downloads/safe.*.txt /path/to/project/data/raw/
```

### Step 3: Retrain

```bash
cd /path/to/project
./retrain.sh
```

This merges raw data, deduplicates, trains the model, evaluates accuracy, and updates `extension/model.json.gz`.

### Step 4: Reload Extension

Go to `about:debugging` → find "Spam Labeler" → click **Reload**.

The new model version will appear in the toolbar status indicator.

## Files

| File | Purpose |
|------|---------|
| `manifest.json` | Extension manifest (Firefox add-on config) |
| `background.js` | Persistent background script — loads & runs the ML model |
| `content.js` | Injected on Google pages — toolbar, checkboxes, highlighting |
| `style.css` | Styling for highlighted spam results |
| `model.json.gz` | Compressed Bernoulli NB model (vocabulary + weights) |
| `sign.sh` | Helper script for optional extension signing |

## Model Details

- **Classifier**: Bernoulli Naive Bayes with Lidstone smoothing (α=0.1)
- **Features**: Tokens + trigrams + quadgrams from training data
- **Hard rules**: Currency symbols (`$€£฿`) and spam links (`line.me`, `@line`) → auto-flag as spam
- **Compression**: gzip compressed, loaded via `DecompressionStream("gzip")` in background script
