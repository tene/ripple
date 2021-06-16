//! Demonstrates how to read events asynchronously with tokio.
//!
//! cargo run --features="event-stream" --example event-stream-tokio

use anyhow;
use ripple::App;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = App::new()?;
    app.run().await
}
