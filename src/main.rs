use clap::{Parser, Subcommand};
use cmdle::Word;

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
            let word = match Word::from(word) {
                Err(e) => {
                    println!("Failed to guess word: {}", e);
                    return;
                },
                Ok(word) => word,
            };
            println!("Tried to guess {}", word);
        }
    }
}
