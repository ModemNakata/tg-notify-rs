use serde::Deserialize;
use std::sync::OnceLock;
use tracing::{debug, error, info};

static CONFIG: OnceLock<Config> = OnceLock::new();

pub struct Notifier {
    token: String,
    chat_id: String,
}

impl Notifier {
    pub fn new(token: impl Into<String>, chat_id: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            chat_id: chat_id.into(),
        }
    }

    pub fn notify(&self, message: &str) {
        let token = self.token.clone();
        let chat_id = self.chat_id.clone();
        let message = message.to_string();

        std::thread::spawn(move || {
            debug!("Spawning thread to send notification: {}", message);
            match send_message(&token, &chat_id, &message) {
                Ok(_) => info!("Notification sent: {}", message),
                Err(e) => error!("Failed to send notification: {}", e),
            }
        });
    }
}

struct Config {
    token: String,
    chat_id: String,
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

    let _ = CONFIG.set(Config { token, chat_id });

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

    let notifier = Notifier::new(&config.token, &config.chat_id);
    notifier.notify(message);
}

fn send_message(token: &str, chat_id: &str, message: &str) -> Result<(), String> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);

    debug!(
        "Sending to Telegram: chat_id={}, message={}",
        chat_id, message
    );

    let payload = serde_json::json!({
        "chat_id": chat_id,
        "text": message,
    });

    let response = ureq::post(&url)
        .set("Content-Type", "application/json")
        .send_string(&payload.to_string())
        .map_err(|e| e.to_string())?;

    let status = response.status();
    debug!("Telegram response status: {}", status);

    let body = response.into_string().map_err(|e| e.to_string())?;
    debug!("Telegram response body: {}", body);

    let telegram_response: TelegramResponse =
        serde_json::from_str(&body).map_err(|e| e.to_string())?;

    if !telegram_response.ok {
        let error_msg = telegram_response
            .description
            .unwrap_or_else(|| "Unknown error".to_string());
        error!("Telegram API error: {}", error_msg);
    } else {
        info!("Telegram notification sent successfully");
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

    #[test]
    fn test_notifier_new() {
        // TODO
        let notifier = Notifier::new("token", "chat_id");
        notifier.notify("test message");
    }
}
