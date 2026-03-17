use tg_notify::notify;
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tg_notify::init("YOUR_BOT_TOKEN", "YOUR_CHAT_ID");

    notify("Hello from tg_notify! This runs in the background.");

    println!("Notification sent! Check your Telegram.");

    std::thread::sleep(std::time::Duration::from_secs(2));
}
