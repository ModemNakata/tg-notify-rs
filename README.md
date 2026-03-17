# tg_notify

A lightweight Rust library for sending Telegram bot notifications in the background without blocking your code.

## What it solves

Fire-and-forget notifications. Call `notify()` and it runs in a background thread. Works with any Rust codebase - tokio, actix, async-std, or sync code. Zero runtime dependencies.
Based on ureq v2.

## Install

```toml
# Cargo.toml
[dependencies]
tg_notify = "1.1"
```

## Usage

```rust
fn main() {
    tg_notify::init("BOT_TOKEN", "CHAT_ID");

    notify("Server started!");
    notify(&format!("Request processed: {}", id));
}
```

Works anywhere - tokio, actix, sync apps, CLI tools.

## Requirements

- Rust 2024 edition

## License

MIT
