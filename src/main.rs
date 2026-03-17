use std::path::Path;
use hydrogen::file_io;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hydrogen", about = "Huffman compression tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compress a file using Huffman coding
    Compress {
        /// Path to the file to compress
        file: String,
        /// Output path (defaults to input as the name and forces .h2 extension)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Decompress a .h2 file
    Decompress {
        /// Path to the .h2 file to decompress
        file: String,
        /// Output path (defaults to input with .txt extension)
        #[arg(short, long)]
        output: Option<String>,
    }
}

fn main() {

    let cli = Cli::parse();

    match cli.command {
        Commands::Compress { file, output } => {
            let base = match output {
                Some(path) => path,
                None => file.clone(),
            };
            // force .h2 file ending for compressed files
            let output = Path::new(&base).with_extension("h2").to_string_lossy().to_string();

            match file_io::run_compression(&file, &output) {
                Ok(()) => println!("Compression Successful"),
                Err(e) => {
                    eprintln!("Compression Failed: {}", e);
                    std::process::exit(1);
            }
            };
        }
        Commands::Decompress { file, output } => {
            let output = match output {
                Some(path) => path,
                None => Path::new(&file).with_extension("txt").to_string_lossy().to_string(),
            };

            match file_io::run_decompression(&file, &output) {
                Ok(()) => println!("Decompression Successful"),
                Err(e) => {
                    eprintln!("Decompression Failed: {}", e);
                    std::process::exit(1);
            }
            };
        }
    }

}