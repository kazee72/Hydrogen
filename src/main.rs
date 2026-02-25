use hydrogen::file_io;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hydrogen")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compress {
        file: String,
        #[arg(short)]
        o: Option<String>,
    },
    Decompress {
        file: String,
        #[arg(short)]
        o: Option<String>,
    }
}

fn main() {

    let cli = Cli::parse();

    match cli.command {
        Commands::Compress { file, o } => {
            let output = match o {
                Some(path) => path,
                None => file.replace(".txt", ".h2"),
            };
            match file_io::run_compression(&file, &output) {
                Ok(()) => println!("Compression Successful"),
                Err(e) => eprintln!("Compression Failed: {}", e),
            };
        }
        Commands::Decompress { file, o } => {
            let output = match o {
                Some(path) => path,
                None => file.replace(".h2", ".txt"),
            };

            match file_io::run_decompression(&file, &output) {
                Ok(()) => println!("Decompression Successful"),
                Err(e) => eprintln!("Decompression Failed: {}", e),
            };
        }
    }

}