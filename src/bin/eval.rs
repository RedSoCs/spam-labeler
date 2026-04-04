use std::collections::{HashMap, HashSet};
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct BernoulliNB {
    class_log_prior: Vec<f64>,
    feature_log_prob: Vec<Vec<f64>>,
    classes: Vec<u32>,
    alpha: f64,
}

impl BernoulliNB {
    fn fit(samples: &[(Vec<f64>, u32)], alpha: f64) -> Self {
        let n_samples = samples.len();
        if n_samples == 0 {
            return BernoulliNB {
                class_log_prior: vec![],
                feature_log_prob: vec![],
                classes: vec![],
                alpha,
            };
        }

        let n_features = samples[0].0.len();
        let class_set: Vec<u32> = samples
            .iter()
            .map(|(_, y)| *y)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        let n_classes = class_set.len();

        let mut class_log_prior = vec![0.0; n_classes];
        let mut feature_counts = vec![vec![0.0; n_features]; n_classes];
        let mut class_counts = vec![0.0; n_classes];

        for (features, label) in samples {
            let class_idx = class_set.iter().position(|&c| c == *label).unwrap();
            class_counts[class_idx] += 1.0;
            for (j, &f) in features.iter().enumerate() {
                if f > 0.0 {
                    feature_counts[class_idx][j] += 1.0;
                }
            }
        }

        for i in 0..n_classes {
            class_log_prior[i] =
                ((class_counts[i] + alpha) / (n_samples as f64 + n_classes as f64 * alpha)).ln();
            for j in 0..n_features {
                let val: f64 = (feature_counts[i][j] + alpha) / (class_counts[i] + 2.0 * alpha);
                feature_counts[i][j] = val.ln();
            }
        }

        BernoulliNB {
            class_log_prior,
            feature_log_prob: feature_counts,
            classes: class_set,
            alpha,
        }
    }

    fn predict(&self, features: &[f64]) -> (bool, f64) {
        if self.classes.is_empty() {
            return (false, 0.0);
        }

        let scores: Vec<f64> = self
            .classes
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut s = self.class_log_prior[i];
                for (j, &f) in features.iter().enumerate() {
                    if f > 0.0 {
                        s += self.feature_log_prob[i][j];
                    } else {
                        let prob = self.feature_log_prob[i][j].exp();
                        s += (1.0 - prob).max(1e-300).ln();
                    }
                }
                s
            })
            .collect();

        let max_s = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exps: Vec<f64> = scores.iter().map(|s| (s - max_s).exp()).collect();
        let sum: f64 = exps.iter().sum();
        let prob_class_1 = if self.classes.len() > 1 {
            exps[1] / sum
        } else {
            0.0
        };

        let best_idx = scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);

        (self.classes[best_idx] == 1, prob_class_1)
    }
}

const CURRENCY_SYMBOLS: &[&str] = &[
    "$", "€", "£", "฿", "¥", "₹", "₽", "₿", "₮", "₩", "₱", "₫",
];

fn has_currency_symbol(text: &str) -> bool {
    CURRENCY_SYMBOLS.iter().any(|sym| text.contains(sym))
}

fn contains_spam_links(text: &str) -> bool {
    let lower = text.to_lowercase();
    lower.contains("line.me") || lower.contains("@line") || lower.contains("lin.ee")
}

const HIGH_SIGNAL_KEYWORDS: &[&str] = &[
    "แตกง่าย", "แตกบ่อย", "บาคาร่า", "วอเลท", "wallet",
    "เว็บตรง", "ไม่ผ่านเอเย่นต์", "เครดิตฟรี", "ฝาก15รับ100",
    "pg slot", "pgslot", "joker slot", "jokerslot",
    "สล็อต", "คาสิโนออนไลน์", "ทดลองเล่น", "ฟรีสปิน",
    "jackpot", "แจ็คพอต", "cashback", "คืนยอดเสีย",
];

fn keyword_weight_score(text: &str) -> f64 {
    let text_lower = text.to_lowercase();
    let mut score = 0.0;
    for &kw in HIGH_SIGNAL_KEYWORDS {
        if text_lower.contains(&kw.to_lowercase()) {
            score += 5.0;
        }
    }
    score
}

struct Vectorizer {
    vocabulary: HashMap<String, usize>,
}

impl Vectorizer {
    fn new() -> Self {
        Vectorizer { vocabulary: HashMap::new() }
    }

    fn build_vocabulary(&mut self, samples: &[(String, bool)]) {
        let mut vocab = HashMap::new();
        let mut idx = 0;

        for (text, _label) in samples {
            let text_lower = text.to_lowercase();

            for token in text_lower.split_whitespace() {
                let cleaned: String = token.chars().filter(|c| !c.is_whitespace()).collect();
                if !cleaned.is_empty() && !vocab.contains_key(&cleaned) {
                    vocab.insert(cleaned, idx);
                    idx += 1;
                }
            }

            let chars: Vec<char> = text_lower.chars().collect();
            for i in 0..chars.len().saturating_sub(2) {
                let trigram: String = chars[i..i + 3].iter().collect();
                if !vocab.contains_key(&trigram) {
                    vocab.insert(trigram, idx);
                    idx += 1;
                }
            }
            for i in 0..chars.len().saturating_sub(3) {
                let quadgram: String = chars[i..i + 4].iter().collect();
                if !vocab.contains_key(&quadgram) {
                    vocab.insert(quadgram, idx);
                    idx += 1;
                }
            }
        }
        self.vocabulary = vocab;
    }

    fn transform(&self, text: &str) -> Vec<f64> {
        let text_lower = text.to_lowercase();
        let mut features = vec![0.0f64; self.vocabulary.len()];

        for token in text_lower.split_whitespace() {
            let cleaned: String = token.chars().filter(|c| !c.is_whitespace()).collect();
            if let Some(&idx) = self.vocabulary.get(&cleaned) {
                features[idx] = 1.0;
            }
        }

        let chars: Vec<char> = text_lower.chars().collect();
        for i in 0..chars.len().saturating_sub(2) {
            let trigram: String = chars[i..i + 3].iter().collect();
            if let Some(&idx) = self.vocabulary.get(&trigram) {
                features[idx] = 1.0;
            }
        }
        for i in 0..chars.len().saturating_sub(3) {
            let quadgram: String = chars[i..i + 4].iter().collect();
            if let Some(&idx) = self.vocabulary.get(&quadgram) {
                features[idx] = 1.0;
            }
        }
        features
    }
}

fn load_training_data(spam_path: &str, safe_path: &str) -> Vec<(String, bool)> {
    let mut samples = Vec::new();
    if let Ok(content) = fs::read_to_string(spam_path) {
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                samples.push((trimmed.to_string(), true));
            }
        }
    }
    if let Ok(content) = fs::read_to_string(safe_path) {
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                samples.push((trimmed.to_string(), false));
            }
        }
    }
    samples
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

fn main() {
    let train_data = load_training_data("data/spam.txt", "data/safe.txt");
    println!(
        "Training data: {} samples ({} spam, {} safe)",
        train_data.len(),
        train_data.iter().filter(|(_, l)| *l).count(),
        train_data.iter().filter(|(_, l)| !*l).count(),
    );

    let mut vectorizer = Vectorizer::new();
    vectorizer.build_vocabulary(&train_data);
    println!("Vocabulary size: {} features (trigrams + quadgrams)\n", vectorizer.vocabulary.len());

    let ml_samples: Vec<(Vec<f64>, u32)> = train_data
        .iter()
        .map(|(text, label)| (vectorizer.transform(text), if *label { 1 } else { 0 }))
        .collect();

    let model = BernoulliNB::fit(&ml_samples, 0.1);

    // Test against raw data files (leave-one-out style)
    println!("=== Testing against data/raw/ files ===\n");

    let mut correct = 0;
    let mut total = 0;

    // Test spam files
    if let Ok(entries) = fs::read_dir("data/raw") {
        let mut files: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().map_or(false, |ext| ext == "txt")
            })
            .collect();
        files.sort_by_key(|e| e.file_name());

        for entry in &files {
            let path = entry.path();
            let filename = path.file_stem().unwrap().to_string_lossy();
            let is_spam_file = filename.starts_with("spam.");

            if let Ok(content) = fs::read_to_string(&path) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }

                    total += 1;

                    if has_currency_symbol(trimmed) {
                        if is_spam_file { correct += 1; }
                        continue;
                    }

                    if contains_spam_links(trimmed) {
                        if is_spam_file { correct += 1; }
                        continue;
                    }

                    let features = vectorizer.transform(trimmed);
                    let (is_spam, confidence) = model.predict(&features);

                    let kw_score = keyword_weight_score(trimmed);
                    let boosted = if kw_score > 0.0 {
                        (confidence + (kw_score / 100.0)).min(1.0)
                    } else {
                        confidence
                    };

                    let predicted_spam = is_spam;
                    let label = if predicted_spam { "SPAM" } else { "SAFE" };
                    let color = if predicted_spam { "🔴" } else { "🟢" };
                    let expected = if is_spam_file { "spam" } else { "safe" };

                    if predicted_spam == is_spam_file {
                        correct += 1;
                    }

                    println!("[{}] {} ({:.0}%) kw+{:.0} | expected:{} | {}",
                        label, color, boosted * 100.0, kw_score, expected,
                        truncate(trimmed, 70));
                }
            }
        }
    }

    println!("\n=== Results ===");
    if total > 0 {
        println!("Correct: {}/{} ({:.1}%)", correct, total, (correct as f64 / total as f64) * 100.0);
    } else {
        println!("No test data found in data/raw/");
    }
}
