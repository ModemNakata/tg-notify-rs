use tg_notify::Notifier;
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let notifier = Notifier::new("YOUR_BOT_TOKEN", "YOUR_CHAT_ID");

    notifier.notify("Hello from tg_notify!");

    println!("Notification sent! Check your Telegram.");

    std::thread::sleep(std::time::Duration::from_secs(2));
}
