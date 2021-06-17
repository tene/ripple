use crossterm::event::{Event as CEvent, KeyCode, KeyModifiers};
use tokio::{spawn, sync::mpsc::Sender};
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, List, ListItem, Widget},
};

use crate::Event;
pub struct UI {
    log: Vec<String>,
    input: String,
    tx: Sender<Event>,
}

impl UI {
    pub fn new(tx: Sender<Event>) -> Self {
        let log = vec!["Hi".into()];
        let input = String::from("asdf");
        Self { log, input, tx }
    }
    pub async fn handle_input(&mut self, input: CEvent) {
        match input {
            CEvent::Key(key) => self.handle_key(key.code, key.modifiers).await,
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
            KeyCode::Char(ch) => {
                self.input.push(ch);
            }
            KeyCode::Null => {}
            KeyCode::Esc => {
                let _ = self.tx.send(Event::Quit).await;
            }
        };
    }
}

impl Widget for &UI {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(1), Constraint::Max(1)].as_ref())
            .split(area);
        List::new(vec![ListItem::new("Blah")]).render(chunks[0], buf);
        buf.set_string(
            chunks[1].left(),
            chunks[1].top(),
            &self.input,
            Style::default(),
        );
    }
}
