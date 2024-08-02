use crate::structs::{OutputData, Cmd};
use ratatui::widgets::ScrollbarState;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::thread;

pub fn execute_commands(config: Vec<Cmd>, tx: Sender<OutputData>, handles: &mut Vec<thread::JoinHandle<()>>) {
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

                    let output_data = OutputData {
                        direction: item.direction,
                        size: item.size,
                        data: data.clone(),
                        vertical_scroll_state: ScrollbarState::new(data.len()).content_length(data.len()).viewport_content_length(1),
                    };

                    match thread_tx.send(output_data) {
                        Ok(_) => (),
                        Err(_e) => {
                            // Handle the error properly
                        },
                    }
                }
            }
        });

        handles.push(handle);
    }
}
