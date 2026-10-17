use clap::{Parser, error};
use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Parser)]
struct Cli {
    path: Vec<std::path::PathBuf>
}
fn read_contents (item: &File) -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(item);
    for line in reader.lines() {
        print!("{}",line?);
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    for item in &args.path {
        let file = std::fs::File::open(item)?;
        read_contents(&file)?;
    }
    Ok(())
}
