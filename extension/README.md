# Spam Labeler Firefox Extension

Collects real-world Google search results as training data for the spam detection model.

## Install

1. Open Firefox → `about:config`
2. Set `xpinstall.signatures.required` = **`false`**
3. Go to `about:debugging` → **This Firefox** → **Load Temporary Add-on**
4. Select `extension/manifest.json`

## Usage

1. Go to **Google Search** and search for anything
2. A dark toolbar appears at the top with:
   - **🔴 SPAM** — download selected items as spam
   - **🟢 SAFE** — download selected items as safe
   - **(N selected)** — count of checked items
3. **Check the boxes** next to search results you want to label
4. Click **SPAM** or **SAFE** → downloads `spam.[timestamp].txt` or `safe.[timestamp].txt`

### Behavior

- **With checkboxes checked** → downloads only the checked items' descriptions
- **No checkboxes checked** → downloads ALL results on the page
- After download → checkboxes auto-clear

## Output Format

Each file contains **one description per line** — ready for training:

```
spam.1712345678.txt:
สล็อตทดลองฟรี ระบบสมาชิกออกแบบมาเพื่อให้คุณเข้าถึงสิทธิประโยชน์
คาสิโนออนไลน์ที่ดีที่สุด สมัครสมาชิกฟรี รับโบนัส 100%
Welcome bonus! Deposit now get 200% match up to $500
```

## Training Pipeline

```bash
# Append collected data to training files
for f in ~/Downloads/spam.*.txt; do
  cat "$f" >> data/spam.txt
done

for f in ~/Downloads/safe.*.txt; do
  cat "$f" >> data/safe.txt
done

# Deduplicate
sort -u data/spam.txt > data/spam_tmp.txt && mv data/spam_tmp.txt data/spam.txt
sort -u data/safe.txt > data/safe_tmp.txt && mv data/safe_tmp.txt data/safe.txt

# Retrain
rm -f model.bin
cargo run --release
```
