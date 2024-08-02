use ratatui::widgets::ScrollbarState;
use serde::{Deserialize, Serialize};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

pub struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value = "content")]
    pub file: PathBuf,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Cmd {
    pub command: String,
    pub size: u8,
    pub direction: u8
}

#[derive(Debug, Clone)]
pub struct OutputData {
    pub direction: u8,
    pub size: u8,
    pub data: Vec<String>,
    pub vertical_scroll_state: ScrollbarState,
}

