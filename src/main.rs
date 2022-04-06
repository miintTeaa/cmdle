use clap::{Parser, Subcommand};
use cmdle::{get_daily_word, Game, LetterResult, Word};
use colored::Colorize;

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
    /// Shows current game
    Check,
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
            println!("Started new game with daily word.");
        }
        Commands::Check => {
            let game = Game::from_file("save.json")?;
            for guess in game.get_guess_iterator() {
                let results = game.compare_to_goal(guess);
                print_word(guess, results);
            }
        }
        Commands::Guess { word } => {
            let mut game = Game::from_file("save.json")?;

            if game.is_full() {
                return Err("Out of guesses! Run \"cmdle check\" to see results.");
            }

            println!("{}", game.goal); //Debug
            let word = match Word::from(word.clone()) {
                Err(e) => return Err(e),
                Ok(word) => word,
            };

            game.add_guess(word);
            for guess in game.get_guess_iterator() {
                let results = game.compare_to_goal(guess);
                print_word(guess, results);
            }

            game.save_to_file("save.json")?;
        }
    })
}

fn print_word(word: &Word, results: [LetterResult; 5]) {
    for i in 0..5 {
        match results[i] {
            LetterResult::Correct => {
                print!("{}", format!("{}", word.get(i)).black().on_bright_green())
            }
            LetterResult::WrongPosition => {
                print!("{}", format!("{}", word.get(i)).black().on_bright_yellow())
            }
            LetterResult::WrongLetter => {
                print!("{}", word.get(i))
            }
        };
    }
    print!("\n");
}
