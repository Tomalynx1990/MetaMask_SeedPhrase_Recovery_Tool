/// Seed phrase validation and recovery
use std::collections::HashMap;

pub struct SeedPhraseValidator {
    valid_word_count: Vec<usize>,
}

impl SeedPhraseValidator {
    pub fn new() -> Self {
        SeedPhraseValidator {
            valid_word_count: vec![12, 15, 18, 21, 24],
        }
    }

    /// Validate seed phrase format
    pub fn validate(&self, phrase: &str) -> bool {
        let words: Vec<&str> = phrase.split_whitespace().collect();
        let word_count = words.len();

        if !self.valid_word_count.contains(&word_count) {
            return false;
        }

        // All words should be lowercase alphanumeric
        words.iter().all(|w| w.chars().all(|c| c.is_ascii_lowercase()))
    }

    /// Extract wallet information from seed phrase
    pub fn extract_wallet_info(&self, phrase: &str) -> HashMap<String, String> {
        let mut info = HashMap::new();

        info.insert("seed_phrase".to_string(), phrase.to_string());
        info.insert("word_count".to_string(),
                   phrase.split_whitespace().count().to_string());
        info.insert("status".to_string(), "recovered".to_string());

        info
    }

    /// Check if seed phrase is recoverable
    pub fn is_recoverable(&self, partial_phrase: &str) -> bool {
        let word_count = partial_phrase.split_whitespace().count();
        word_count >= 8
    }

    /// Generate recovery suggestions
    pub fn get_recovery_suggestions(&self, phrase: &str) -> Vec<String> {
        vec![
            "Try entering the complete 12-word phrase".to_string(),
            "Check for typos in each word".to_string(),
            "Ensure words are in the correct order".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_12_words() {
        let validator = SeedPhraseValidator::new();
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        assert!(validator.validate(phrase));
    }
}
