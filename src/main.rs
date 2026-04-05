mod keyword_intent;

use std::collections::{HashMap, HashSet};
use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, EnableBracketedPaste, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, Paragraph, Wrap},
    Terminal,
};
use serde::{Deserialize, Serialize};
use tiktoken_rs::o200k_base;
use tui_input::{backend::crossterm::EventHandler, Input};

// ── High-signal spam keywords (for manual weighting) ───────────────

const HIGH_SIGNAL_KEYWORDS: &[&str] = &[
    "แตกง่าย", "แตกบ่อย", "บาคาร่า", "วอเลท", "wallet",
    "เว็บตรง", "ไม่ผ่านเอเย่นต์", "เครดิตฟรี", "ฝาก15รับ100",
    "pg slot", "pgslot", "joker slot", "jokerslot",
    "สล็อต", "คาสิโนออนไลน์", "ทดลองเล่น", "ฟรีสปิน",
    "jackpot", "แจ็คพอต", "cashback", "คืนยอดเสีย",
];

// ── Bernoulli Naive Bayes with Lidstone smoothing ──────────────────

#[derive(Serialize, Deserialize, Clone)]
struct BernoulliNB {
    class_log_prior: Vec<f64>,
    feature_log_prob: Vec<Vec<f64>>,
    classes: Vec<u32>,
    alpha: f64, // Lidstone smoothing parameter
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

        // Lidstone smoothing (alpha < 1.0 for large vocabularies)
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

// ── Currency symbols (hard rule) ───────────────────────────────────

const CURRENCY_SYMBOLS: &[&str] = &[
    "$", "€", "£", "฿", "¥", "₹", "₽", "₿", "₮", "₩", "₱", "₫",
];

fn has_currency_symbol(text: &str) -> bool {
    CURRENCY_SYMBOLS.iter().any(|sym| text.contains(sym))
}

// ── Link/URL/Line ID detection ─────────────────────────────────────

fn contains_spam_links(text: &str) -> bool {
    let lower = text.to_lowercase();
    lower.contains("line.me")
        || lower.contains("@line")
        || (lower.contains('@') && lower.contains("add"))
        || lower.contains("bit.ly")
        || lower.contains("shorturl")
        || lower.contains("tinyurl")
        || lower.contains("liff.line")
        || lower.contains("lin.ee")
}

// ── Thai/English ratio feature ─────────────────────────────────────

fn thai_english_ratio(text: &str) -> f64 {
    let mut thai_count = 0;
    let mut english_count = 0;

    for ch in text.chars() {
        if ch >= '\u{0E00}' && ch <= '\u{0E7F}' {
            thai_count += 1;
        } else if ch.is_ascii_alphabetic() {
            english_count += 1;
        }
    }

    let total = thai_count + english_count;
    if total == 0 {
        return 0.0;
    }

    // Return ratio: how mixed the text is (0.5 = perfectly mixed)
    let thai_ratio = thai_count as f64 / total as f64;
    let eng_ratio = english_count as f64 / total as f64;

    // Mixed content scores higher (closer to 0.5 each)
    1.0 - (thai_ratio - eng_ratio).abs()
}

// ── Keyword weighting score ────────────────────────────────────────

fn keyword_weight_score(text: &str) -> f64 {
    let text_lower = text.to_lowercase();
    let mut score = 0.0;

    for &kw in HIGH_SIGNAL_KEYWORDS {
        if text_lower.contains(&kw.to_lowercase()) {
            score += 5.0; // Each high-signal keyword adds 5.0
        }
    }

    score
}

// ── Vectorizer (trigrams + quadgrams for Thai) ─────────────────────

struct Vectorizer {
    vocabulary: HashMap<String, usize>,
}

impl Vectorizer {
    fn new() -> Self {
        Vectorizer {
            vocabulary: HashMap::new(),
        }
    }

    fn build_vocabulary(&mut self, samples: &[(String, bool)]) {
        let mut vocab = HashMap::new();
        let mut idx = 0;

        for (text, _label) in samples {
            let text_lower = text.to_lowercase();

            // Single tokens
            for token in text_lower.split_whitespace() {
                let cleaned: String = token.chars().filter(|c| !c.is_whitespace()).collect();
                if !cleaned.is_empty() && !vocab.contains_key(&cleaned) {
                    vocab.insert(cleaned, idx);
                    idx += 1;
                }
            }

            let chars: Vec<char> = text_lower.chars().collect();

            // Trigrams (3 chars) - good for Thai
            for i in 0..chars.len().saturating_sub(2) {
                let trigram: String = chars[i..i + 3].iter().collect();
                if !vocab.contains_key(&trigram) {
                    vocab.insert(trigram, idx);
                    idx += 1;
                }
            }

            // Quadgrams (4 chars) - very specific to Thai spam patterns
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

        // Single tokens
        for token in text_lower.split_whitespace() {
            let cleaned: String = token.chars().filter(|c| !c.is_whitespace()).collect();
            if let Some(&idx) = self.vocabulary.get(&cleaned) {
                features[idx] = 1.0;
            }
        }

        let chars: Vec<char> = text_lower.chars().collect();

        // Trigrams
        for i in 0..chars.len().saturating_sub(2) {
            let trigram: String = chars[i..i + 3].iter().collect();
            if let Some(&idx) = self.vocabulary.get(&trigram) {
                features[idx] = 1.0;
            }
        }

        // Quadgrams
        for i in 0..chars.len().saturating_sub(3) {
            let quadgram: String = chars[i..i + 4].iter().collect();
            if let Some(&idx) = self.vocabulary.get(&quadgram) {
                features[idx] = 1.0;
            }
        }

        features
    }
}

// ── Saved model ────────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
struct SavedModel {
    model: BernoulliNB,
    vocabulary: HashMap<String, usize>,
}

// ── ML Model wrapper ───────────────────────────────────────────────

struct MLModel {
    vectorizer: Vectorizer,
    model: Option<BernoulliNB>,
    is_trained: bool,
    model_path: String,
    vocab_size: usize,
}

impl MLModel {
    fn new(model_path: &str) -> Self {
        MLModel {
            vectorizer: Vectorizer::new(),
            model: None,
            is_trained: false,
            model_path: model_path.to_string(),
            vocab_size: 0,
        }
    }

    fn save_model(&self) {
        if let Some(m) = &self.model {
            let saved = SavedModel {
                model: m.clone(),
                vocabulary: self.vectorizer.vocabulary.clone(),
            };
            if let Ok(bytes) = bincode::serialize(&saved) {
                if let Err(e) = std::fs::write(&self.model_path, bytes) {
                    eprintln!("Warning: Could not save model: {}", e);
                }
            }
        }
    }

    fn load_model(&mut self) -> bool {
        if let Ok(bytes) = std::fs::read(&self.model_path) {
            if let Ok(saved) = bincode::deserialize::<SavedModel>(&bytes) {
                self.vectorizer.vocabulary = saved.vocabulary;
                self.model = Some(saved.model);
                self.vocab_size = self.vectorizer.vocabulary.len();
                self.is_trained = true;
                return true;
            }
        }
        false
    }

    fn train(&mut self, samples: &[(String, bool)]) {
        if samples.is_empty() {
            return;
        }

        self.vectorizer.build_vocabulary(samples);
        self.vocab_size = self.vectorizer.vocabulary.len();

        let ml_samples: Vec<(Vec<f64>, u32)> = samples
            .iter()
            .map(|(text, label)| {
                (self.vectorizer.transform(text), if *label { 1 } else { 0 })
            })
            .collect();

        // Lidstone smoothing with alpha=0.1 for large vocabulary
        self.model = Some(BernoulliNB::fit(&ml_samples, 0.1));
        self.is_trained = true;
    }

    fn predict(&self, text: &str) -> Option<(bool, f64)> {
        // Hard rule: currency symbol = spam
        if has_currency_symbol(text) {
            return Some((true, 1.0));
        }

        // Hard rule: spam links = spam
        if contains_spam_links(text) {
            return Some((true, 0.95));
        }

        let model = self.model.as_ref()?;
        let features = self.vectorizer.transform(text);
        let (is_spam, prob) = model.predict(&features);

        // Add keyword weight boost
        let kw_score = keyword_weight_score(text);
        let boosted_prob = if kw_score > 0.0 {
            (prob + (kw_score / 100.0)).min(1.0)
        } else {
            prob
        };

        // Add Thai/English mixed content signal
        let mix_ratio = thai_english_ratio(text);
        let final_prob = if is_spam && mix_ratio > 0.3 {
            // Mixed Thai+English with spam prediction = higher confidence
            (boosted_prob + 0.05).min(1.0)
        } else {
            boosted_prob
        };

        Some((is_spam, final_prob))
    }

    fn train_with_defaults(&mut self) {
        let samples: Vec<(String, bool)> = vec![
            ("สล็อตทดลองฟรี ระบบสมาชิกออกแบบมาเพื่อให้คุณเข้าถึงสิทธิประโยชน์".to_string(), true),
            ("สมัครสมาชิกวันนี้ รับโบนัสทันที ฝากขั้นต่ำ 100 บาท".to_string(), true),
            ("สล็อตเว็บตรง ไม่ผ่านเอเย่นต์ แตกง่าย แจกเครดิตฟรี".to_string(), true),
            ("Welcome bonus! Deposit now get 200% match up to $500".to_string(), true),
            ("คาสิโนออนไลน์ บาคาร่า รูเล็ต แจ็คพอตแตกใหญ่".to_string(), true),
            ("Hello, how are you today?".to_string(), false),
            ("The weather is nice and sunny.".to_string(), false),
            ("ร้านอาหารอร่อยใกล้บ้าน แนะนำเมนูพิเศษวันนี้".to_string(), false),
            ("การประชุมวันพรุ่งนี้ เวลา 10 โมงเช้า".to_string(), false),
            ("Machine learning is transforming technology.".to_string(), false),
        ];
        self.train(&samples);
    }
}

// ── Training data loader ───────────────────────────────────────────

fn load_training_data(spam_path: &str, safe_path: &str) -> Vec<(String, bool)> {
    let mut samples = Vec::new();

    if let Ok(content) = std::fs::read_to_string(spam_path) {
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                samples.push((trimmed.to_string(), true));
            }
        }
    }

    if let Ok(content) = std::fs::read_to_string(safe_path) {
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                samples.push((trimmed.to_string(), false));
            }
        }
    }

    samples
}

// ── App State ──────────────────────────────────────────────────────

struct App {
    input: Input,
    last_tokens: Vec<String>,
    last_token_ids: Vec<u32>,
    token_count: usize,
    char_count: usize,
    ml_prediction: Option<(bool, f64)>,
    status_message: String,
    currency_detected: bool,
    link_detected: bool,
    keyword_score: f64,
    mix_ratio: f64,
    analyzed: bool,
    quit: bool,
}

impl App {
    fn new() -> Self {
        App {
            input: Input::default(),
            last_tokens: Vec::new(),
            last_token_ids: Vec::new(),
            token_count: 0,
            char_count: 0,
            ml_prediction: None,
            status_message: "Type text and press Enter to analyze | q to quit".to_string(),
            currency_detected: false,
            link_detected: false,
            keyword_score: 0.0,
            mix_ratio: 0.0,
            analyzed: false,
            quit: false,
        }
    }
}

// ── UI Rendering ───────────────────────────────────────────────────

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Input
            Constraint::Length(3),  // Summary
            Constraint::Min(6),     // Tokens
            Constraint::Min(6),     // Analysis
            Constraint::Length(3),  // Status
        ])
        .split(f.area());

    // Input area - always active
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(" Input (type text, press Enter to analyze) ");
    let input_widget = Paragraph::new(app.input.value())
        .block(input_block)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(input_widget, chunks[0]);
    f.set_cursor_position((
        chunks[0].x + app.input.cursor() as u16 + 1,
        chunks[0].y + 1,
    ));

    // Summary
    let summary_text = format!(
        "Tokens: {} | Characters: {} | TH/EN mix: {:.0}%",
        app.token_count, app.char_count, app.mix_ratio * 100.0
    );
    let summary = Paragraph::new(summary_text)
        .block(Block::default().borders(Borders::ALL).title(" Summary "));
    f.render_widget(summary, chunks[1]);

    // Tokens
    let token_lines: Vec<Line> = if app.last_tokens.is_empty() {
        vec![Line::from(Span::styled(
            "No tokens yet. Type text and press Enter.",
            Style::default().fg(Color::DarkGray),
        ))]
    } else {
        let mut lines = Vec::new();
        for chunk in app.last_tokens.chunks(10) {
            let spans: Vec<Span> = chunk
                .iter()
                .enumerate()
                .flat_map(|(i, t)| {
                    let id = app.last_token_ids.get(i).copied().unwrap_or(0);
                    vec![
                        Span::styled(
                            format!("{} ", t.replace('\u{200b}', "·")),
                            Style::default().fg(Color::Cyan),
                        ),
                        Span::styled(
                            format!("[{}] ", id),
                            Style::default().fg(Color::DarkGray),
                        ),
                    ]
                })
                .collect();
            lines.push(Line::from(spans));
        }
        lines
    };
    let token_list = List::new(token_lines)
        .block(Block::default().borders(Borders::ALL).title(" Tokens "));
    f.render_widget(token_list, chunks[2]);

    // Analysis
    let mut analysis_lines: Vec<Line> = Vec::new();

    if app.analyzed {
        if let Some((is_spam, prob)) = app.ml_prediction {
            let (label, color) = if is_spam {
                ("SPAM/CASINO DETECTED", Color::Red)
            } else {
                ("LIKELY SAFE", Color::Green)
            };
            analysis_lines.push(Line::from(Span::styled(
                format!("ML Prediction: {} ({:.1}%)", label, prob * 100.0),
                Style::default()
                    .fg(color)
                    .add_modifier(Modifier::BOLD),
            )));
        }

        if app.currency_detected {
            analysis_lines.push(Line::from(Span::styled(
                "⚠ Currency symbol detected — auto-flagged as SPAM",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            )));
        }

        if app.link_detected {
            analysis_lines.push(Line::from(Span::styled(
                "⚠ Spam link detected (Line/URL shortener)",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            )));
        }

        if app.keyword_score > 0.0 {
            analysis_lines.push(Line::from(Span::styled(
                format!("Keyword weight: +{:.1} ({} high-signal words)",
                    app.keyword_score,
                    (app.keyword_score / 5.0) as usize),
                Style::default().fg(Color::Magenta),
            )));
        }
    } else {
        analysis_lines.push(Line::from(Span::styled(
            "No analysis yet. Type text and press Enter.",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let analysis_widget = Paragraph::new(analysis_lines)
        .block(Block::default().borders(Borders::ALL).title(" Analysis "))
        .wrap(Wrap { trim: true });
    f.render_widget(analysis_widget, chunks[3]);

    // Status
    let status = Paragraph::new(app.status_message.as_str())
        .block(Block::default().borders(Borders::ALL).title(" Status "));
    f.render_widget(status, chunks[4]);
}

// ── Main ───────────────────────────────────────────────────────────

fn main() -> io::Result<()> {
    // Initialize tokenizer
    let bpe = o200k_base().expect("Failed to load o200k_base tokenizer");

    // Initialize ML model
    let mut ml_model = MLModel::new("model.bin");

    if ml_model.load_model() {
        // Loaded from file
    } else {
        let custom_data = load_training_data("data/spam.txt", "data/safe.txt");
        if !custom_data.is_empty() {
            ml_model.train(&custom_data);
        } else {
            ml_model.train_with_defaults();
        }
        ml_model.save_model();
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        EnableBracketedPaste,
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    // Main loop - only draw when needed, sleep to reduce CPU
    let mut needs_draw = true;

    loop {
        if needs_draw {
            terminal.draw(|f| ui(f, &app))?;
            needs_draw = false;
        }

        if app.quit {
            break;
        }

        // Block waiting for event (reduces CPU usage)
        if event::poll(Duration::from_millis(200)).unwrap_or(false) {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }

                    match key.code {
                        KeyCode::Enter => {
                            let text = app.input.value().to_string();
                            if !text.is_empty() {
                                let token_ids = bpe.encode_with_special_tokens(&text);
                                let token_texts: Vec<String> = token_ids
                                    .iter()
                                    .map(|id| bpe.decode(vec![*id]).unwrap_or_default())
                                    .collect();

                                app.last_tokens = token_texts;
                                app.last_token_ids = token_ids.clone();
                                app.token_count = token_ids.len();
                                app.char_count = text.chars().count();
                                app.currency_detected = has_currency_symbol(&text);
                                app.link_detected = contains_spam_links(&text);
                                app.keyword_score = keyword_weight_score(&text);
                                app.mix_ratio = thai_english_ratio(&text);
                                app.ml_prediction = ml_model.predict(&text);
                                app.analyzed = true;

                                let verdict = match &app.ml_prediction {
                                    Some((true, p)) => {
                                        format!("SPAM/CASINO ({:.0}% confidence)", p * 100.0)
                                    }
                                    Some((false, p)) => {
                                        format!("SAFE ({:.0}% confidence)", (1.0 - p) * 100.0)
                                    }
                                    None => "No prediction".to_string(),
                                };
                                app.status_message = verdict;
                            }
                            app.input.reset();
                            needs_draw = true;
                        }
                        KeyCode::Char('q') if app.input.value().is_empty() => {
                            app.quit = true;
                            needs_draw = true;
                        }
                        _ => {
                            app.input.handle_event(&Event::Key(key));
                            needs_draw = true;
                        }
                    }
                }
                Event::Paste(text) => {
                    for ch in text.chars() {
                        if ch == '\n' || ch == '\r' {
                            let text = app.input.value().to_string();
                            if !text.is_empty() {
                                let token_ids = bpe.encode_with_special_tokens(&text);
                                let token_texts: Vec<String> = token_ids
                                    .iter()
                                    .map(|id| bpe.decode(vec![*id]).unwrap_or_default())
                                    .collect();

                                app.last_tokens = token_texts;
                                app.last_token_ids = token_ids.clone();
                                app.token_count = token_ids.len();
                                app.char_count = text.chars().count();
                                app.currency_detected = has_currency_symbol(&text);
                                app.link_detected = contains_spam_links(&text);
                                app.keyword_score = keyword_weight_score(&text);
                                app.mix_ratio = thai_english_ratio(&text);
                                app.ml_prediction = ml_model.predict(&text);
                                app.analyzed = true;

                                let verdict = match &app.ml_prediction {
                                    Some((true, p)) => {
                                        format!("SPAM/CASINO ({:.0}% confidence)", p * 100.0)
                                    }
                                    Some((false, p)) => {
                                        format!("SAFE ({:.0}% confidence)", (1.0 - p) * 100.0)
                                    }
                                    None => "No prediction".to_string(),
                                };
                                app.status_message = verdict;
                            }
                            app.input.reset();
                            needs_draw = true;
                            break;
                        } else {
                            app.input.handle_event(&Event::Key(
                                crossterm::event::KeyEvent::new(
                                    KeyCode::Char(ch),
                                    crossterm::event::KeyModifiers::NONE,
                                ),
                            ));
                            needs_draw = true;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}
