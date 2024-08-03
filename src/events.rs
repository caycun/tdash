use std::io;
use std::time::Duration;
use ratatui::crossterm::event::{self, Event, KeyCode};
use crate::App;

pub fn handle_events(app: &mut App) -> io::Result<bool> {
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(false);
            } else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('f') {
                while let Ok(true) = event::poll(Duration::from_millis(100)) {
                    if let Event::Key(key) = event::read()? {
                        if let KeyCode::Char(c) = key.code {
                            if c.is_ascii_digit() {
                                app.current_window = c.to_digit(10).unwrap() as u8;
                                return Ok(true);
                            }
                        }
                    }
                }
            } else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Enter {
                app.full_screen = true;
            } else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Esc && app.full_screen {
                    app.full_screen = false;
            }
        }
    }
    Ok(true)
}
