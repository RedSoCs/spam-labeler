use std::collections::{HashMap, HashSet};
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct BernoulliNB {
    class_log_prior: Vec<f64>,
    feature_log_prob: Vec<Vec<f64>>,
    classes: Vec<u32>,
}

impl BernoulliNB {
    fn fit(samples: &[(Vec<f64>, u32)], alpha: f64) -> Self {
        let n_samples = samples.len();
        if n_samples == 0 {
            return BernoulliNB {
                class_log_prior: vec![],
                feature_log_prob: vec![],
                classes: vec![],
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
        }
    }
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

#[derive(Serialize)]
struct ExportModel {
    version: String,
    vocabulary: HashMap<String, usize>,
    class_log_prior: Vec<f64>,
    feature_log_prob: Vec<Vec<f64>>,
    classes: Vec<u32>,
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

fn main() {
    let train_data = load_training_data("data/spam.txt", "data/safe.txt");
    println!("Training: {} samples ({} spam, {} safe)",
        train_data.len(),
        train_data.iter().filter(|(_, l)| *l).count(),
        train_data.iter().filter(|(_, l)| !*l).count(),
    );

    let mut vectorizer = Vectorizer::new();
    vectorizer.build_vocabulary(&train_data);

    let ml_samples: Vec<(Vec<f64>, u32)> = train_data
        .iter()
        .map(|(text, label)| (vectorizer.transform(text), if *label { 1 } else { 0 }))
        .collect();

    let model = BernoulliNB::fit(&ml_samples, 0.1);

    // Version based on sample count: v0.{samples / 10}
    let sample_count = train_data.len();
    let feature_count = vectorizer.vocabulary.len();
    let minor = sample_count / 10;
    let version = format!("v0.{}", minor);

    let exported = ExportModel {
        version: version.clone(),
        vocabulary: vectorizer.vocabulary,
        class_log_prior: model.class_log_prior,
        feature_log_prob: model.feature_log_prob,
        classes: model.classes,
    };

    let json = serde_json::to_string(&exported).expect("Failed to serialize");
    fs::write("extension/model.json", &json).expect("Failed to write model.json");

    println!("Model {} ({} samples, {} features) exported to extension/model.json ({:.1} KB)", version, sample_count, feature_count, json.len() as f64 / 1024.0);
}
