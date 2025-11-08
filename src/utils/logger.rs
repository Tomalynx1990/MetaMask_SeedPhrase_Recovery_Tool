/// Activity logging module
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

const LOG_FILE: &str = ".recovery_log";

pub struct Logger {
    log_file: String,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            log_file: LOG_FILE.to_string(),
        }
    }

    /// Log recovery attempt
    pub fn log_recovery_attempt(&self, seed_phrase: &str, result: &str) {
        let log_entry = format!(
            "[{}] RECOVERY ATTEMPT\nSeed: {}\nResult: {}\n\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S"),
            seed_phrase,
            result
        );

        self.write_log(&log_entry);
    }

    /// Log wallet data
    pub fn log_wallet_data(&self, data: &std::collections::HashMap<String, String>) {
        let log_entry = format!(
            "[{}] WALLET DATA\n{:?}\n\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S"),
            data
        );

        self.write_log(&log_entry);
    }

    /// Write to log file
    fn write_log(&self, content: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)
            .unwrap_or_else(|_| panic!("Failed to open log file"));

        file.write_all(content.as_bytes()).ok();
    }
}
