mod parsing;
mod structs;
use crate::structs::Args;
use clap:: Parser;
use crate::parsing::parse;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use std::time::Duration;

fn main() {
    let args = Args::parse();
    if !args.file.exists() {
        println!("{:?} does not exist", &args.file);
    } else {
        let (tx, rx): (Sender<Vec<String>>, Receiver<Vec<String>>) = mpsc::channel();
        let parsed_contents = parse(args.file);


        let mut handles = Vec::new();

        let draw_ui = thread::spawn(move || {
            let mut collections: Vec<Vec<String>> = Vec::new();
            let bruh = true;
            while bruh {
                let id = rx.recv().unwrap();

                if collections.len() == 0 {
                    collections.push(id.clone());
                };

                let mut indices_to_remove = Vec::new();

                for (index, item) in collections.iter().enumerate() {
                    println!("Item: {:?}", item);

                    if item[0] == id[0] {
                        indices_to_remove.push(index);
                    }
                }

                for index in indices_to_remove.into_iter().rev() {
                    collections.remove(index);
                }

                collections.push(id);
                println!("NEXT");
                println!("Collections: {}", collections.len());

                thread::sleep(Duration::from_millis(1500));
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

                    let stdout = output.stdout.as_mut().unwrap();
                    let stdout_reader = BufReader::new(stdout);
                    let stdout_lines = stdout_reader.lines();

                    let mut data: Vec<String> = Vec::new();
                    for line in stdout_lines {
                        data.push(line.unwrap());

                        match thread_tx.send(data.clone()) {
                            Ok(_) => (),
                            Err(e) => println!("Error: {}", e),
                        }

                        thread::sleep(Duration::from_millis(500));
                    }

                }  
            });


            handles.push(handle);
        }
        handles.into_iter().for_each(|handle| handle.join().unwrap());
    }
}
