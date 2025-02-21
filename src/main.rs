use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use anyhow::{Context, Result};  //  helps with custom errors

// search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    pattern: String,  // pattern to look for
    path: PathBuf,  // path to file that will be read
}

fn main() -> Result<()> {  // helps with catching errors
    let args = Cli::parse();  // parses cli input
    // opens file that will be read
    let file = File::open(&args.path)
        .with_context(|| format!("could not open file `{}`", args.path.display()))?;  // Custom error messages 

    let reader = BufReader::new(file);  // Reads line by line instead of loading the whole fime into memory

    for line in reader.lines() {
        let line = line?;  // Handles errors when reading lines
        // Checks if patern is included
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())  // error catching
}
