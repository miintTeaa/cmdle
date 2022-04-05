use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about = "A word game for the command line.")]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Makes a guess
    Guess { word: String },
}

fn main() {
    let args = Args::parse();
    
    match &args.command {
        Commands::Guess { word } => {
            println!("Tried to guess {}", word);
        }
    }
}
