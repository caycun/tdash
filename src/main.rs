mod parsing;
mod structs;
use crate::structs::Args;
use clap:: Parser;
use crate::parsing::parse;
use std::{collections, thread};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use std::time::Duration;

use std::io::{self, stdout};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};

fn ui(frame: &mut Frame, collections: Vec<Vec<String>>) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            collections
            .iter()
            .map(|_| Constraint::Percentage(100 / collections.len() as u16))
            .collect::<Vec<_>>(),
        )
        .split(frame.size());
    for (index, item) in collections.iter().enumerate() {
        let block = Block::default().title("Output").borders(Borders::ALL);
        let paragraph = Paragraph::new(item.join("\n")).block(block);
        frame.render_widget(paragraph, layout[index]);
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    if !args.file.exists() {
        println!("{:?} does not exist", &args.file);
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    } else {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        let (tx, rx): (Sender<Vec<String>>, Receiver<Vec<String>>) = mpsc::channel();
        let parsed_contents = parse(args.file);


        let mut handles = Vec::new();

        let draw_ui = thread::spawn(move || {
            let mut collections: Vec<Vec<String>> = Vec::new();
            let mut bruh = true;
            while bruh {
                let id = rx.recv().unwrap();
                if collections.len() == 0 {
                    collections.push(id.clone());
                };

                let mut index_to_insert = collections.len();
                let mut indices_to_remove = Vec::new();

                for (index, item) in collections.iter().enumerate() {
                    if item[0] == id[0] {
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
                //println!("NEXT");
                //println!("Collections: {}", collections.len());

                //thread::sleep(Duration::from_millis(100));
                bruh = handle_events().unwrap();
                //println!("bruh: {}", bruh);
            }
        });

        handles.push(draw_ui);

        for item in parsed_contents {

            let thread_tx = tx.clone();
            let handle = thread::spawn(move || {

                let args = item.1.split_whitespace().collect::<Vec<&str>>();
                if args.capacity() > 0 {
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

                        match thread_tx.send(data.clone()) {
                            Ok(_) => (),
                            Err(_e) => {
                                disable_raw_mode().unwrap();
                                stdout().execute(LeaveAlternateScreen).unwrap();
                            }
                            ,
                        }

                        //thread::sleep(Duration::from_millis(10));
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
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(false);
            }
        }
    }
    Ok(true)
}
