use std::collections::HashMap;
use std::fs;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tiktoken_rs::o200k_base;

// ── Re-implement keyword_intent logic inline for the test binary ──

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Intent {
    Transactional,
    Commercial,
    Informational,
    Navigational,
    Unknown,
}

impl std::fmt::Display for Intent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Intent::Transactional => write!(f, "Transactional"),
            Intent::Commercial => write!(f, "Commercial"),
            Intent::Informational => write!(f, "Informational"),
            Intent::Navigational => write!(f, "Navigational"),
            Intent::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordScore {
    pub word: String,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub language: String,
    pub processor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentResponse {
    pub primary_intent: String,
    pub confidence_score: f64,
    pub extracted_keywords: Vec<KeywordScore>,
    pub metadata: Metadata,
}

static INTENT_DICT: Lazy<HashMap<Intent, Vec<&str>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(
        Intent::Commercial,
        vec![
            "ฟรี", "โปรโมชั่น", "โบนัส", "ไม่ต้องฝาก", "เครดิตฟรี",
            "แจก", "ของขวัญ", "รางวัล", "พิเศษ", "ส่วนลด",
            "cashback", "คืนยอดเสีย",
        ],
    );
    m.insert(
        Intent::Transactional,
        vec![
            "สมัคร", "ทางเข้า", "ลงทะเบียน", "ฝากเงิน", "ถอนเงิน",
            "ฝาก", "ถอน", "ขั้นต่ำ", "สมาชิก", "vip",
            "wallet", "วอเลท", "เข้าระบบ", "เข้าสู่ระบบ",
        ],
    );
    m.insert(
        Intent::Informational,
        vec![
            "วิธี", "กติกา", "คืออะไร", "อย่างไร", "แนะนำ",
            "เล่น", "ขั้นตอน", "เงื่อนไข", "รายละเอียด", "ทดลอง",
        ],
    );
    m.insert(
        Intent::Navigational,
        vec![
            "888", "pg slot", "joker", "jili", "spade",
            "ufa", "bet", "goal", "livescore", "slot",
            "casino", "บาคาร่า", "รูเล็ต", "สล็อต",
        ],
    );
    m
});

static HTML_TAG_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"<[^>]+>").unwrap());
static WHITESPACE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());
static SPECIAL_CHAR_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\p{L}\p{M}\p{N}\s]").unwrap());

pub fn normalize_text(text: &str) -> String {
    let text = HTML_TAG_RE.replace_all(text, " ");
    let text = SPECIAL_CHAR_RE.replace_all(&text, " ");
    let text = WHITESPACE_RE.replace_all(&text, " ");
    text.trim().to_string()
}

static THAI_STOP_WORDS: Lazy<HashSet<String>> = Lazy::new(|| {
    let words = vec![
        "ที่", "ใน", "และ", "เป็น", "โดย", "ให้", "ได้", "มี",
        "ไม่", "ไป", "มา", "จะ", "ทำ", "การ", "ถูก", "แล้ว",
        "นี้", "นั้น", "แต่", "ถ้า", "หรือ", "จาก", "กับ",
        "ของ", "สำหรับ", "เมื่อ", "ยัง", "ก็", "อยู่",
        "คือ", "จึง", "เช่น", "อย่าง", "เพื่อ", "ไว้", "ขึ้น",
        "ลง", "ออก", "เรา", "คุณ", "เขา", "มัน", "อะไร",
        "มาก", "น้อย", "อีก", "เอง", "เดียว",
        "the", "a", "an", "is", "are", "was", "were", "be",
        "been", "being", "have", "has", "had", "do", "does",
        "did", "will", "would", "could", "should", "may",
        "might", "shall", "can", "to", "of", "in", "for",
        "on", "with", "at", "by", "from", "as", "into",
        "through", "during", "before", "after", "above",
        "below", "between", "out", "off", "over", "under",
        "again", "further", "then", "once", "here", "there",
        "when", "where", "why", "how", "all", "each", "few",
        "more", "most", "other", "some", "such", "no", "nor",
        "not", "only", "own", "same", "so", "than", "too",
        "very", "just", "because", "but", "and", "or",
        "if", "while", "about", "up", "it", "its", "this",
        "that", "these", "those", "what", "which", "who",
        "whom", "whose",
    ];
    words.into_iter().map(|s| s.to_string()).collect()
});

use std::collections::HashSet;

static BPE_TOKENIZER: Lazy<Option<tiktoken_rs::CoreBPE>> = Lazy::new(|| {
    o200k_base().ok()
});

// ── Technique 1: Prefix Keyword Extraction ─────────────────────────

fn is_pure_thai(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| ('\u{0E00}'..='\u{0E7F}').contains(&c))
}

fn is_pure_ascii(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii())
}

fn extract_english_prefix(s: &str) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        if ch.is_ascii() { result.push(ch); } else { break; }
    }
    result
}

fn extract_thai_prefix(s: &str) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        if ('\u{0E00}'..='\u{0E7F}').contains(&ch) { result.push(ch); } else { break; }
    }
    result
}

fn extract_prefix_intent_keyword(text: &str) -> Option<String> {
    let normalized = normalize_text(text);
    let first_word = normalized.split_whitespace().next()?;
    
    if first_word.len() < 2 { return None; }
    
    let has_thai = first_word.chars().any(|c| ('\u{0E00}'..='\u{0E7F}').contains(&c));
    let has_ascii = first_word.chars().any(|c| c.is_ascii_alphanumeric());
    
    if has_thai && has_ascii {
        let first_char_is_thai = first_word.chars().next()
            .map(|c| ('\u{0E00}'..='\u{0E7F}').contains(&c))
            .unwrap_or(false);
        
        if first_char_is_thai {
            let thai_prefix = extract_thai_prefix(first_word);
            if thai_prefix.len() >= 2 {
                Some(thai_prefix)
            } else {
                let ascii_part: String = first_word.chars()
                    .skip_while(|c| ('\u{0E00}'..='\u{0E7F}').contains(c))
                    .collect();
                Some(ascii_part.to_lowercase())
            }
        } else {
            let eng_prefix = extract_english_prefix(first_word);
            Some(eng_prefix.to_lowercase())
        }
    } else if is_pure_thai(first_word) {
        Some(first_word.to_string())
    } else if is_pure_ascii(first_word) {
        Some(first_word.to_lowercase())
    } else {
        Some(first_word.to_lowercase())
    }
}

// ── Technique 2: Sub-tokenize with tiktoken ────────────────────────

fn subtokenize_keyword(keyword: &str) -> Vec<String> {
    let bpe = match BPE_TOKENIZER.as_ref() {
        Some(bpe) => bpe,
        None => return vec![keyword.to_string()],
    };
    
    let token_ids = bpe.encode_with_special_tokens(keyword);
    let mut sub_tokens = Vec::new();
    
    for id in token_ids {
        if let Ok(decoded) = bpe.decode(vec![id]) {
            let cleaned = decoded.trim().to_string();
            if !cleaned.is_empty() && cleaned.len() >= 2 {
                sub_tokens.push(cleaned.to_lowercase());
            }
        }
    }
    
    if sub_tokens.is_empty() {
        sub_tokens.push(keyword.to_lowercase());
    }
    
    sub_tokens
}

// ── RAKE: Intent Dictionary Scanning ───────────────────────────────

pub fn rake_keywords(text: &str, top_n: usize) -> Vec<(String, f64)> {
    let normalized = normalize_text(text);
    let lower = normalized.to_lowercase();
    let mut keyword_scores: HashMap<String, f64> = HashMap::new();
    
    for (_intent, dict_words) in INTENT_DICT.iter() {
        for &dict_word in dict_words {
            if lower.contains(&dict_word.to_lowercase()) {
                let count = lower.matches(&dict_word.to_lowercase()).count();
                let score = count as f64 * 2.0;
                *keyword_scores.entry(dict_word.to_string())
                    .or_insert(0.0) += score;
            }
        }
    }
    
    for word in lower.split_whitespace() {
        let cleaned: String = word.chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        if cleaned.len() >= 2 && !THAI_STOP_WORDS.contains(&cleaned) {
            for (_intent, dict_words) in INTENT_DICT.iter() {
                for &dict_word in dict_words {
                    if cleaned.contains(&dict_word.to_lowercase())
                        && !keyword_scores.contains_key(dict_word)
                    {
                        *keyword_scores.entry(dict_word.to_string())
                            .or_insert(0.0) += 1.0;
                    }
                }
            }
        }
    }
    
    let mut word_scores: Vec<(String, f64)> = keyword_scores.into_iter().collect();
    word_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    word_scores.into_iter().take(top_n).collect()
}

pub fn classify_intent(keywords: &[(String, f64)]) -> (Intent, f64) {
    if keywords.is_empty() {
        return (Intent::Unknown, 0.0);
    }

    let mut intent_scores: HashMap<Intent, f64> = HashMap::new();

    for (keyword, score) in keywords {
        for (intent, dict_words) in INTENT_DICT.iter() {
            for &dict_word in dict_words {
                if keyword.to_lowercase().contains(&dict_word.to_lowercase())
                    || dict_word.to_lowercase().contains(&keyword.to_lowercase())
                {
                    *intent_scores.entry(intent.clone()).or_insert(0.0) += score;
                }
            }
        }
    }

    let (best_intent, best_score) = intent_scores
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or((Intent::Unknown, 0.0));

    let total_score: f64 = keywords.iter().map(|(_, s)| s).sum();
    let confidence = if total_score > 0.0 {
        (best_score / total_score).min(1.0)
    } else {
        0.0
    };

    (best_intent, confidence)
}

pub fn extract_keyword_intents(text: &str) -> IntentResponse {
    let mut all_keywords: Vec<(String, f64)> = Vec::new();
    
    if let Some(prefix_kw) = extract_prefix_intent_keyword(text) {
        let sub_tokens = subtokenize_keyword(&prefix_kw);
        all_keywords.push((prefix_kw.clone(), 10.0));
        for token in sub_tokens {
            if !all_keywords.iter().any(|(k, _)| k == &token) {
                all_keywords.push((token, 8.0));
            }
        }
    }
    
    let rake_kws = rake_keywords(text, 8);
    for (word, score) in rake_kws {
        if !all_keywords.iter().any(|(k, _)| k == &word) {
            all_keywords.push((word, score));
        }
    }
    
    let (primary_intent, confidence) = classify_intent(&all_keywords);
    
    let max_score = all_keywords.iter().map(|(_, s)| s).cloned().fold(0.0f64, f64::max);
    let normalized_keywords: Vec<KeywordScore> = if max_score > 0.0 {
        all_keywords.into_iter().map(|(word, score)| {
            KeywordScore {
                word,
                score: (score / max_score * 100.0).round() / 100.0,
            }
        }).collect()
    } else {
        all_keywords.into_iter().map(|(word, score)| {
            KeywordScore { word, score }
        }).collect()
    };

    IntentResponse {
        primary_intent: primary_intent.to_string(),
        confidence_score: (confidence * 100.0).round() / 100.0,
        extracted_keywords: normalized_keywords,
        metadata: Metadata {
            language: "th".to_string(),
            processor: "rake-rs-v1".to_string(),
        },
    }
}

// ── Report Generator ────────────────────────────────────────────────

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

fn main() {
    let mut report = String::new();
    
    report.push_str("# Keyword Intent Extraction Report\n\n");
    report.push_str("Generated against: `data/spam.txt`\n\n");
    report.push_str("## Techniques\n\n");
    report.push_str("1. **Prefix Keyword Extraction** — Extract first word before space as primary intent keyword\n");
    report.push_str("2. **Tiktoken Sub-tokenization** — Split mixed Thai/English prefixes (e.g., `888NEO` → `888`, `neo`)\n");
    report.push_str("3. **Intent Dictionary Scanning** — RAKE-style keyword extraction from body text\n\n");
    report.push_str("---\n\n");

    let spam_content = match fs::read_to_string("data/spam.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading data/spam.txt: {}", e);
            return;
        }
    };

    let lines: Vec<&str> = spam_content.lines().filter(|l| !l.trim().is_empty()).collect();
    
    let mut intent_counts: HashMap<String, usize> = HashMap::new();
    let mut total_keywords = 0;
    let mut results: Vec<IntentResponse> = Vec::new();

    for line in &lines {
        let result = extract_keyword_intents(line);
        *intent_counts.entry(result.primary_intent.clone()).or_insert(0) += 1;
        total_keywords += result.extracted_keywords.len();
        results.push(result);
    }

    // Summary table
    report.push_str("## Intent Distribution Summary\n\n");
    report.push_str(&format!("- **Total samples analyzed**: {}\n", lines.len()));
    report.push_str(&format!("- **Total keywords extracted**: {}\n\n", total_keywords));
    
    report.push_str("| Intent | Count | Percentage |\n");
    report.push_str("|--------|-------|------------|\n");
    
    let mut intents: Vec<_> = intent_counts.iter().collect();
    intents.sort_by(|a, b| b.1.cmp(a.1));
    
    for (intent, count) in &intents {
        let pct = (**count as f64 / lines.len() as f64) * 100.0;
        report.push_str(&format!("| {} | {} | {:.1}% |\n", intent, count, pct));
    }

    report.push_str("\n---\n\n");
    report.push_str("## Per-Sample Results\n\n");

    for (idx, (line, result)) in lines.iter().zip(results.iter()).enumerate() {
        let prefix_kw = extract_prefix_intent_keyword(line);
        let sub_tokens = prefix_kw.as_ref()
            .map(|kw| subtokenize_keyword(kw))
            .unwrap_or_default();

        report.push_str(&format!("### Sample {}\n\n", idx + 1));
        report.push_str(&format!("- **Input**: `{}`\n", truncate(line, 100)));
        report.push_str(&format!("- **Prefix keyword**: `{:?}`\n", prefix_kw));
        report.push_str(&format!("- **Sub-tokens**: `{:?}`\n", sub_tokens));
        report.push_str(&format!("- **Primary intent**: `{}` (confidence: {:.0}%)\n\n",
            result.primary_intent, result.confidence_score * 100.0));
        
        report.push_str("| Keyword | Score |\n");
        report.push_str("|---------|-------|\n");
        for kw in &result.extracted_keywords {
            report.push_str(&format!("| {} | {:.2} |\n", kw.word, kw.score));
        }
        report.push_str("\n");
    }

    report.push_str("---\n\n");
    report.push_str("## Sample JSON Output\n\n");
    report.push_str("```json\n");
    if let Some(first) = results.first() {
        report.push_str(&serde_json::to_string_pretty(first).unwrap());
    }
    report.push_str("\n```\n\n");

    report.push_str("---\n\n");
    report.push_str("## Performance\n\n");
    let start = std::time::Instant::now();
    let test_text = "สมัครสมาชิกวันนี้ รับโบนัสฟรี ไม่ต้องฝากแรกเข้า เล่นสล็อต PG";
    for _ in 0..100 {
        let _ = extract_keyword_intents(test_text);
    }
    let duration = start.elapsed();
    let avg_ms = duration.as_millis() as f64 / 100.0;
    report.push_str(&format!("- **100 iterations**: `{:?}`\n", duration));
    report.push_str(&format!("- **Average per iteration**: `{:.2}ms` (target: <50ms)\n", avg_ms));
    report.push_str(&format!("- **Status**: {}\n", if avg_ms < 50.0 { "PASS ✓" } else { "FAIL ✗" }));

    fs::write("tests/keyword-intent-report.md", &report)
        .expect("Failed to write report");
    
    println!("Report saved to tests/keyword-intent-report.md");
    println!("  {} bytes, {} samples analyzed", report.len(), lines.len());
}
