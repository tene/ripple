use std::io::stdout;

use anyhow;
use futures::{future::FutureExt, select, StreamExt};

use crossterm::{
    cursor::position,
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
pub struct UI;

impl UI {
    pub async fn run(&mut self) -> anyhow::Result<()> {
        enable_raw_mode()?;

        let mut stdout = stdout();
        execute!(
            stdout,
            EnableMouseCapture,
            EnterAlternateScreen,
            DisableLineWrap
        )?;
        let rv = self.event_loop().await;
        execute!(
            stdout,
            LeaveAlternateScreen,
            DisableMouseCapture,
            EnableLineWrap
        )?;

        disable_raw_mode()?;
        rv
    }
    async fn event_loop(&mut self) -> anyhow::Result<()> {
        let mut reader = EventStream::new();
        while let Some(event) = reader.next().await {
            let event = event?;

            println!("Event::{:?}\r", event);

            if event == Event::Key(KeyCode::Char('c').into()) {
                println!("Cursor position: {:?}\r", position());
            }

            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        }
        Ok(())
    }
}
