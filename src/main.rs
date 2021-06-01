//! Demonstrates how to read events asynchronously with tokio.
//!
//! cargo run --features="event-stream" --example event-stream-tokio

use anyhow;
use ripple::UI;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut ui = UI;
    ui.run().await
}
