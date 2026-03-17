use reqwest::Client;
use serde::Deserialize;
use std::sync::OnceLock;
use tracing::{error, info};

static CONFIG: OnceLock<Config> = OnceLock::new();

struct Config {
    token: String,
    chat_id: String,
    client: Client,
}

#[derive(Deserialize, Debug)]
struct TelegramResponse {
    ok: bool,
    #[allow(dead_code)]
    #[serde(skip_serializing_if = "Option::is_none")]
    error_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

pub fn init(token: impl Into<String>, chat_id: impl Into<String>) {
    let token = token.into();
    let chat_id = chat_id.into();

    let client = Client::new();

    let _ = CONFIG.set(Config {
        token,
        chat_id,
        client,
    });

    info!("tg_notify initialized");
}

pub fn notify(message: &str) {
    let config = match CONFIG.get() {
        Some(c) => c,
        None => {
            error!("tg_notify not initialized, call init() first");
            return;
        }
    };

    let token = config.token.clone();
    let chat_id = config.chat_id.clone();
    let client = config.client.clone();
    let message = message.to_string();

    tokio::spawn(async move {
        if let Err(e) = send_message(&client, &token, &chat_id, &message).await {
            error!("Failed to send telegram notification: {}", e);
        } else {
            info!("Telegram notification sent");
        }
    });
}

async fn send_message(
    client: &Client,
    token: &str,
    chat_id: &str,
    message: &str,
) -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        token
    );

    let payload = serde_json::json!({
        "chat_id": chat_id,
        "text": message,
    });

    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await?;

    let telegram_response: TelegramResponse = response.json().await?;

    if !telegram_response.ok {
        let error_msg = telegram_response
            .description
            .unwrap_or_else(|| "Unknown error".to_string());
        error!("Telegram API error: {}", error_msg);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        init("test_token", "test_chat_id");
        assert!(CONFIG.get().is_some());
    }
}
