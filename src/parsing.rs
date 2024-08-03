use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn parse(file_name: PathBuf) -> String {
    let file = File::open(file_name);
    let mut contents = String::new();

    match file {
        Ok(mut result) => result
            .read_to_string(&mut contents)
            .expect("Unable to read file"),
        Err(_error) => panic!(),
    };

    contents
}
