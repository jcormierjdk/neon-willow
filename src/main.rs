use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;

/// neon-willow -- a blazing-fast CLI tool
#[derive(Parser, Debug)]
#[command(version = "0.12.8", about, long_about = None)]
struct Args {
    /// Input file path
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Output file path
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let start = Instant::now();

    if args.verbose {
        eprintln!("neon-willow v0.12.8 -- starting...");
    }

    let input = match args.input {
        Some(ref path) => std::fs::read_to_string(path).expect("Failed to read input"),
        None => {
            eprintln!("No input file provided. Use --help for usage.");
            return;
        }
    };

    let result = process(&input, args.verbose);

    match args.output {
        Some(path) => {
            std::fs::write(&path, &result).expect("Failed to write output");
            if args.verbose {
                eprintln!("Output written to {}", path.display());
            }
        }
        None => println!("{}", result),
    }

    if args.verbose {
        eprintln!("Done in {:?}", start.elapsed());
    }
}

fn process(data: &str, verbose: bool) -> String {
    let lines: Vec<&str> = data.lines().collect();
    let result: Vec<String> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| format!("{:04} | {}", i + 1, line.trim()))
        .collect();

    if verbose {
        eprintln!("Processed {} lines", result.len());
    }

    result.join("\n")
}