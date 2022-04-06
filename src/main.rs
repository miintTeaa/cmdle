use clap::{Parser, Subcommand};
use cmdle::{get_daily_word, setup_cwd, Game, LetterResult, Word};
use colored::Colorize;

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

    setup_cwd();

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
            print_game(&game);
        }
        Commands::Guess { word } => {
            let mut game = Game::from_file("save.json")?;

            if game.is_full() {
                return Err("Out of guesses! Run \"cmdle check\" to see results.");
            }

            //println!("{}", game.goal); //Debug
            let word = match Word::from(word.clone()) {
                Err(e) => return Err(e),
                Ok(word) => word,
            };

            game.add_guess(word);
            print_game(&game);

            game.save_to_file("save.json")?;
        }
    })
}

fn print_game(game: &Game) {
    println!("{}", "C M D L E".black().on_bright_white());
    println!("{}", "         ".on_white());
    for guess in game.get_guess_iterator() {
        let results = game.compare_to_goal(guess);
        print!("{}", "  ".on_white());
        print_word(guess, results);
        println!("{}", "  ".on_white());
    }
    println!("{}", "         ".on_white());
    if game.is_won() {
        println!(
            "{}{}{}",
            "  ".on_white(),
            //
            (&game.get_goal().to_string())
                .to_uppercase()
                .black()
                .on_bright_green(),
            //
            "  ".on_white()
        );
        println!("{}", "         ".on_white());
    } else if game.is_lost() {
        println!(
            "{}{}{}",
            "  ".on_white(),
            //
            (&game.get_goal().to_string())
                .to_uppercase()
                .black()
                .on_bright_red(),
            //
            "  ".on_white()
        );
        println!("{}", "         ".on_white());
    }
}

fn print_word(word: &Word, results: [LetterResult; 5]) {
    for i in 0..5 {
        match results[i] {
            LetterResult::Correct => {
                print!(
                    "{}",
                    format!("{}", word.get(i))
                        .to_uppercase()
                        .black()
                        .on_bright_green()
                )
            }
            LetterResult::WrongPosition => {
                print!(
                    "{}",
                    format!("{}", word.get(i))
                        .to_uppercase()
                        .black()
                        .on_bright_yellow()
                )
            }
            LetterResult::WrongLetter => {
                print!("{}", format!("{}", word.get(i)).to_uppercase())
            }
        };
    }
}
