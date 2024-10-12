use anyhow::{Context, Ok, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    // CODE WITHOUT LIBRARY CLAP
    // let pattern = std::env::args().nth(1).expect("No Pattern given");
    // let path = std::env::args().nth(2).expect("No path given");

    // println!("Pattern: {:?}, path {:?}", args.pattern, args.path);

    // CODE WHIT LIBRARY CLAP
    let args = Cli::parse();

    // Mensaje de error normal
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    // Mensaje de error con libreria anyhow
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
