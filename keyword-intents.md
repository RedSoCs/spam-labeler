Below is a proposed **SPEC.md** for a Rust-based keyword extraction and intent classification module. This specification bridges the gap between raw Thai text payloads and structured intent data using the strategies we discussed: **Tokenization**, **RAKE (Scoring)**, and **Intent Mapping**.

---

# Specification: Keyword Intent Extractor (Rust)

## 1. Overview

The goal of this module is to ingest text payloads (primarily in Thai), extract significant keywords using the **RAKE (Rapid Automatic Keyword Extraction)** algorithm, and classify the overall "Intent" of the payload into predefined categories (Transactional, Commercial, Informational).

## 2. Technical Stack

- **Language:** Rust (Edition 2021)
- **Tokenization:** `lindera` (with Thai dictionary) or `icu_segmenter`.
- **Extraction Algorithm:** `rake` crate (or custom TextRank implementation).
- **Data Handling:** `serde` for JSON payload parsing.
- **Regex:** `regex` crate for final intent mapping.

## 3. Data Architecture

### 3.1 Intent Enums

We define the target categories for the classification engine.

```rust
pub enum Intent {
    Transactional, // User ready to register/deposit
    Commercial,    // User looking for promotions/freebies
    Informational, // User looking for rules/how-to
    Navigational,  // User looking for specific brands (e.g., dee88)
    Unknown,
}
```

### 3.2 Process Flow

The system follows a linear pipeline to move from a raw string to a classified intent.

1.  **Input:** Raw string from payload.
2.  **Normalization:** Remove HTML tags, extra whitespace, and special characters.
3.  **Tokenization:** Segment Thai sentences into individual words.
4.  **Keyword Extraction:** Calculate scores based on word frequency and degree.
5.  **Intent Mapping:** Compare top-scoring keywords against an intent dictionary.
6.  **Output:** Structured JSON containing keywords and the primary intent.

## 4. Component Requirements

### 4.1 Thai Segmentation (The "Space" Problem)

Since Thai does not use spaces, the extractor must use a morphological analyzer.

- **Requirement:** The module must load a Thai dictionary at runtime or compile-time.
- **Logic:** `สมัครสมาชิกวันนี้` -> `["สมัครสมาชิก", "วันนี้"]`.

### 4.2 Scoring Logic (RAKE)

Keywords are ranked based on the ratio of the word's degree to its frequency ($deg(w)/freq(w)$).

- **Degree:** How often a word co-occurs with other candidate keywords.
- **Frequency:** How often the word appears in the text.

### 4.3 Intent Dictionary (Mapping)

A configuration file (or lazy-static HashMap) will map keywords to intents.

- **Commercial:** `["ฟรี", "โปรโมชั่น", "โบนัส", "ไม่ต้องฝาก"]`
- **Transactional:** `["สมัคร", "ทางเข้า", "ลงทะเบียน", "ฝากเงิน"]`
- **Informational:** `["วิธี", "กติกา", "คืออะไร"]`

## 5. Implementation Plan (Steps)

| Phase       | Task                 | Description                                                                        |
| :---------- | :------------------- | :--------------------------------------------------------------------------------- |
| **Phase 1** | **Tokenizer Setup**  | Integrate `lindera` with the Thai CC-CEDICT or custom dictionary.                  |
| **Phase 2** | **Extraction Logic** | Implement the RAKE crate to generate `(Keyword, Score)` pairs.                     |
| **Phase 3** | **Classifier**       | Create a scoring aggregator that sums the weights of keywords per Intent category. |
| **Phase 4** | **API / CLI**        | Wrap the logic in a function: `fn get_intent(input: &str) -> IntentResponse`.      |

## 6. Sample Output Schema

The module should return a structured format for the RedSocs auditing system:

```json
{
  "primary_intent": "Commercial",
  "confidence_score": 0.85,
  "extracted_keywords": [
    { "word": "เครดิตฟรี", "score": 4.5 },
    { "word": "สล็อต PG", "score": 2.0 },
    { "word": "ไม่ต้องฝาก", "score": 1.8 }
  ],
  "metadata": {
    "language": "th",
    "processor": "rake-rs-v1"
  }
}
```

## 7. Performance Constraints

- **Latency:** Processing a 1KB payload must take < 50ms.
- **Memory:** Dictionary-based segmentation should stay within 50MB RSS.
- **Safety:** Must handle malformed UTF-8 sequences without panicking.
