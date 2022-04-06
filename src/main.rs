use clap::{Parser, Subcommand};
use cmdle::{get_daily_word, Game, Word};

extern crate chrono;

#[derive(Parser)]
#[clap(author, version, about = "A word game for the command line.")]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts a new game with the daily word
    Daily,
    /// Makes a guess
    Guess { word: String },
}

fn main() {
    let args = Args::parse();

    if let Err(e) = do_commands(&args) {
        eprintln!("[ERR] {}", e);
    }
}

fn do_commands(args: &Args) -> Result<(), &'static str> {
    Ok(match &args.command {
        Commands::Daily => {
            (Game::new(get_daily_word()?)).save_to_file("save.json")?;
        }
        Commands::Guess { word } => {
            let game = Game::from_file("save.json")?;
            println!("{}", game.goal);
            let word = match Word::from(word.clone()) {
                Err(e) => return Err(e),
                Ok(word) => word,
            };

            println!("Tried to guess {}", word);
            print!("Results: ");
            let results = game.compare_to_goal(&word);
            for result in results {
                print!("{}", result);
            }
            print!("\n");
        }
    })
}
