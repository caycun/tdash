use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn parse(file_name: PathBuf) -> BTreeMap<String, String> {
    let file = File::open(file_name);
    let mut contents = String::new();

    match file {
       Ok(mut result) =>  result.read_to_string(&mut contents)
        .expect("Unable to read file"),
        Err(_error) => panic!()
    };

    serde_yaml::from_str(&contents).unwrap()
}
