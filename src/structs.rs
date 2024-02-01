use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

pub struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value = "content")]
    pub file: PathBuf,
}


