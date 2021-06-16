use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use anyhow;
use futures::{future::FutureExt, select, StreamExt, TryStreamExt};

use crossterm::{
    cursor::position,
    event::{
        DisableMouseCapture, EnableMouseCapture, Event as CEvent, EventStream, KeyCode, KeyEvent,
        KeyModifiers,
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, DisableLineWrap, EnableLineWrap,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use tokio::{
    spawn,
    sync::mpsc::{channel, Receiver, Sender},
};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Widget},
    Terminal,
};

#[derive(PartialEq, Eq)]
pub enum Event {
    Input(CEvent),
    Tick,
    Quit,
}

pub struct App {
    input: String,
    term: Terminal<CrosstermBackend<Stdout>>,
    rx: Receiver<Event>,
    tx: Sender<Event>,
}

impl Widget for App {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        todo!()
    }
}

impl Drop for App {
    fn drop(&mut self) {
        execute!(
            self.term.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            EnableLineWrap
        )
        .expect("Problem during cleanup?");

        disable_raw_mode().expect("Problem during cleanup?");
    }
}

pub async fn run_app() {}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let stdout = stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        let (tx, rx) = channel(1024);

        Ok(Self {
            input: String::new(),
            term: terminal,
            rx,
            tx,
        })
    }
    pub async fn run(&mut self) -> anyhow::Result<()> {
        enable_raw_mode()?;
        execute!(
            self.term.backend_mut(),
            EnableMouseCapture,
            EnterAlternateScreen,
            DisableLineWrap
        )?;

        {
            let tx = self.tx.clone();
            spawn(async move {
                let mut reader = EventStream::new();
                while let Some(Ok(event)) = reader.next().await {
                    let _ = tx.send(Event::Input(event)).await;
                }
            });
            let tx = self.tx.clone();
            spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    let _ = tx.send(Event::Tick);
                }
            });
        }
        self.draw()?;
        while let Some(event) = self.rx.recv().await {
            match event {
                Event::Input(cevent) => {
                    self.handle_input(cevent).await;
                }
                Event::Tick => {}
                Event::Quit => break,
            };
            self.draw()?;
        }
        Ok(())
    }
    fn draw(&mut self) -> anyhow::Result<()> {
        self.term.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Block").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;
        Ok(())
    }
    async fn handle_input(&mut self, event: CEvent) {
        match event {
            CEvent::Key(KeyEvent { code, modifiers }) => self.handle_key(code, modifiers).await,
            CEvent::Mouse(_) => {}
            CEvent::Resize(_, _) => {}
        }
    }
    async fn handle_key(&mut self, k: KeyCode, _m: KeyModifiers) {
        match k {
            KeyCode::Backspace => {}
            KeyCode::Enter => {}
            KeyCode::Left => {}
            KeyCode::Right => {}
            KeyCode::Up => {}
            KeyCode::Down => {}
            KeyCode::Home => {}
            KeyCode::End => {}
            KeyCode::PageUp => {}
            KeyCode::PageDown => {}
            KeyCode::Tab => {}
            KeyCode::BackTab => {}
            KeyCode::Delete => {}
            KeyCode::Insert => {}
            KeyCode::F(_) => {}
            KeyCode::Char(_) => {}
            KeyCode::Null => {}
            KeyCode::Esc => {
                self.tx.send(Event::Quit).await;
            }
        };
    }
}
