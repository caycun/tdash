mod parsing;
mod structs;
use crate::structs::Args;
use clap:: Parser;
use crate::parsing::parse;
use std::thread;
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use std::time::Duration;

fn main() {
    let args = Args::parse();

    if !args.file.exists() {
        println!("{:?} does not exist", &args.file);
    } else {
        let parsed_contents = parse(args.file);

        println!("Parsed contents: {:?}", parsed_contents);
        let mut handles = Vec::new();
        for item in parsed_contents {
            println!("test");
          let handle = thread::spawn(move || {
               println!("test thread");
            let args = item.1.split_whitespace().collect::<Vec<&str>>();
            println!("Running: {:?}", args);
            if args.capacity() > 0 {
                let mut output = Command::new(args[0])
                    .args(&args[1..])
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("failed to execute process");

                    let stdout = output.stdout.as_mut().unwrap();
                    let stdout_reader = BufReader::new(stdout);
                    let stdout_lines = stdout_reader.lines();

                    for line in stdout_lines {
                         println!("Read: {:?}", line);
                         thread::sleep(Duration::from_millis(1));
                     }

            } else {

                let mut output = Command::new(args[0])
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("failed to execute process");

                    let stdout = output.stdout.as_mut().unwrap();
                    let stdout_reader = BufReader::new(stdout);
                    let stdout_lines = stdout_reader.lines();

                    for line in stdout_lines {
                         println!("Read: {:?}", line);
                         thread::sleep(Duration::from_millis(1));
                     }

            }

                });
            handles.push(handle);
        }
            handles.into_iter().for_each(|handle| handle.join().unwrap());
    }
}
