mod commands;
mod events;
mod parsing;
mod structs;
mod ui;

use crate::commands::execute_commands;
use crate::events::handle_events;
use crate::parsing::parse;
use crate::structs::{Args, Cmd, OutputData};
use crate::ui::ui;
use clap::Parser;
use ratatui::{
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
};
use std::io::{self, stdout};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

#[derive(Default)]
struct App {
    current_window: u8,
    _vertical_scroll: u8,
    full_screen: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if !args.file.exists() {
        println!("{:?} does not exist", &args.file);
        Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "File not found",
        )))
    } else {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut app = App {
            current_window: 0,
            _vertical_scroll: 0,
            full_screen: false,
        };
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

                terminal.draw(|f| ui(f, collections.clone(), &app)).unwrap();
                running = handle_events(&mut app).unwrap();
            }
            disable_raw_mode().unwrap();
            stdout().execute(LeaveAlternateScreen).unwrap();
        });

        handles.push(draw_ui);

        execute_commands(config, tx, &mut handles);

        handles
            .into_iter()
            .for_each(|handle| handle.join().unwrap());
        Ok(())
    }
}
