# tg_notify

A lightweight Rust library for sending Telegram bot notifications without blocking your async application.

## What it solves

When building async applications (Axum, Actix, etc.), you often want to send notifications without interrupting request handling. This library provides a simple fire-and-forget interface - just call `notify()` and it runs in the background via `tokio::spawn`. If it fails, it fails silently (with logging), never breaking your application.

## Install

```toml
# Cargo.toml
[dependencies]
tg_notify = "0.1"
```

## Usage

```rust
use tg_notify::notify;

fn main() {
    // Initialize once at startup
    tg_notify::init("BOT_TOKEN", "CHAT_ID");

    // Send notifications anywhere - doesn't block your code
    notify("Server started!");
    notify(&format!("Request processed: {}", id));
}
```

That's it. Works with any tokio-based runtime.

## Requirements

- Rust with tokio runtime

## License

MIT
