use backend::WindowSize;
use serde::{Deserialize, Serialize};
use serde_yaml::Error;
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, stdout, BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
    text::{Line, Text}
};

// Assuming Args and parse are defined in the respective modules
mod parsing;
mod structs;
use crate::structs::Args;
use crate::parsing::parse;
use clap::Parser;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Cmd {
    command: String,
    size: u8,
}

#[derive(Debug, Clone)]
struct OutputData {
    size: u8,
    data: Vec<String>,
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
}

struct Config {
    pub command: String,
    pub size: u8,
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
} 

#[derive(Default)]
struct App {
    current_window: u8,
    vertical_scroll: u8
}

fn ui(frame: &mut Frame, mut collections: Vec<OutputData>) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            collections
            .iter()
            .map(|item| Constraint::Percentage(item.size as u16))
            .collect::<Vec<_>>(),
        )
        .split(frame.size());
    for (index, item) in collections.iter_mut().enumerate() {
        let log = item.data.pop();
        item.data.insert(item.data.len(), log.unwrap());
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut state = ListState::default();
        let list = List::new(item.data.clone())
            .block(Block::bordered().title("List"))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        let available_height = layout[index].height as usize;
        if item.data.len() > available_height {
            *state.offset_mut() = item.data.len() - available_height;
        } else if item.data.len() == available_height {
            *state.offset_mut() = item.data.len() - available_height + 2;
        } else {
            *state.offset_mut() = 0;
        }

        frame.render_stateful_widget(list, layout[index], &mut state);

        frame.render_stateful_widget(
            scrollbar,
            layout[index].inner(Margin {
                vertical: 1,
                horizontal: 0
            }),
            &mut item.vertical_scroll_state,
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if !args.file.exists() {
        println!("{:?} does not exist", &args.file);
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "File not found")));
    } else {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut app = App::default();
        app.current_window = 0;
        app.vertical_scroll = 0;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        let (tx, rx): (Sender<OutputData>, Receiver<OutputData>) = mpsc::channel();
        let parsed_contents = parse(args.file);
        let config: Vec<Cmd> = serde_yaml::from_str(parsed_contents.as_str())?;

        let mut handles = Vec::new();

        let draw_ui = thread::spawn(move || {
            let mut collections: Vec<OutputData> = Vec::new();
            let mut running = true;
            while running {
                let id = rx.recv().unwrap();
                if collections.is_empty() {
                    collections.push(id.clone());
                };

                let mut index_to_insert = collections.len();
                let mut indices_to_remove = Vec::new();

                for (index, item) in collections.iter().enumerate() {
                    if item.size == id.size {
                        indices_to_remove.push(index);
                        index_to_insert = index;
                    }
                }

                for index in indices_to_remove.into_iter().rev() {
                    collections.remove(index);
                }

                if index_to_insert < collections.len() {
                    collections.insert(index_to_insert, id);
                } else {
                    collections.push(id);
                }

                terminal.draw(|f| ui(f, collections.clone())).unwrap();



                running = handle_events().unwrap();
            }
            disable_raw_mode().unwrap();
            stdout().execute(LeaveAlternateScreen).unwrap();
        });

        handles.push(draw_ui);

        for item in config {
            let thread_tx = tx.clone();
            let handle = thread::spawn(move || {
                let args = item.command.split_whitespace().collect::<Vec<&str>>();
                if !args.is_empty() {
                    let mut output = Command::new(args[0])
                        .args(&args[1..])
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("failed to execute process");

                    let stdoute = output.stdout.as_mut().unwrap();
                    let stdout_reader = BufReader::new(stdoute);
                    let stdout_lines = stdout_reader.lines();

                    let mut data: Vec<String> = Vec::new();
                    for line in stdout_lines {
                        data.push(line.unwrap());

                        //let scroll_to = item.size

                        //app.vertical_scroll += 1;
                        //app.vertical_scroll %= 10;
                        let mut output_data = OutputData {
                            size: item.size,
                            data: data.clone(),
                            vertical_scroll_state: ScrollbarState::new(data.len()).content_length(data.len()).viewport_content_length(1),
                            horizontal_scroll_state: ScrollbarState::default(),
                            vertical_scroll: data.len() - 1,
                            horizontal_scroll: 0,
                        };

                        match thread_tx.send(output_data) {
                            Ok(_) => (),
                            Err(_e) => {
                                disable_raw_mode().unwrap();
                                stdout().execute(LeaveAlternateScreen).unwrap();
                            },
                        }
                    }
                }
            });

            handles.push(handle);
        }

        handles.into_iter().for_each(|handle| handle.join().unwrap());
        Ok(())
    }
}

fn handle_events() -> io::Result<bool> {
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(false);
            }
        }
    }
    Ok(true)
}
