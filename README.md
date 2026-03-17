# tg_notify

A lightweight Rust library for sending Telegram bot notifications in the background without blocking your code.

## What it solves

Fire-and-forget notifications. Call `notify()` and it runs in a background thread. Works with any Rust codebase - tokio, actix, async-std, or sync code. Zero runtime dependencies.
Based on ureq v2.

## Install

```toml
# Cargo.toml
[dependencies]
tg_notify = "1.1.2"
```

## Usage

### Option 1: Global init (simple)
```rust
use tg_notify::Notifier;

fn main() {
    let notifier = Notifier::new("BOT_TOKEN", "CHAT_ID");

    notifier.notiy("Hello from main!");

    // you may want to add sleep for short-lived programs, because notifier-thread is not awaited and will be terminated after program' end, so notification may not go through.
    // Basically this crate is designed to be used in `continuous` execution/programs.
    //
    // also, you can add tracing_subscriber for debug purposes. (see examples/basic.rs)
}
```

### Option 2: Example with App State (Axum/Actix style)

```rust
use tg_notify::Notifier;
use axum::{
    Router,
    State,
    routing::get,
};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    tg: Notifier,
}

async fn handler(State(state): State<AppState>) {
    state.tg.notify("Request received!");
}

let notifier = Notifier::new("BOT_TOKEN", "CHAT_ID");
let app = Router::new()
    .route("/", get(handler))
    .with_state(AppState { tg: notifier });
```

Works anywhere - tokio, actix, sync apps, CLI tools.

## Requirements

- Rust 2024 edition

## License

MIT
