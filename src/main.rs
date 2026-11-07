use clap::{Parser};
use std::{fs::File, io::{BufRead, BufReader}};
use std::io::stdout;
use std::io::Write;

#[derive(Parser)]
struct Cli {
    path: Vec<std::path::PathBuf>
}
fn read_contents (item: &File) -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(item);
    let stdout = stdout();
    let mut stdout_lock = stdout.lock();
    for line in reader.lines() {
        //println!("{}",line?);
        writeln!(stdout_lock,"{}",line?)?;
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
