use clap::{Parser, Subcommand};
use cmdle::{get_daily_word, setup_cwd, Game, LetterResult, LetterStatus, Word, ALPHABET};
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

    if let Err(e) = setup_cwd() {
        eprintln!("[ERR] {}", e);
    }

    if let Err(e) = do_commands(&args) {
        eprintln!("[ERR] {}", e);
    }
}

fn do_commands(args: &Args) -> Result<(), &'static str> {
    Ok(match &args.command {
        Commands::Daily => {
            let game = Game::new(get_daily_word()?);
            game.save_to_file("save.json")?;
            print_game(&game);
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
    let blank_line = || println!("{}", "         ".on_white());
    //This will only work with odd length strings
    let bordered = |b: &str, size: usize| {
        let mut spacing = String::new();
        for _ in 0..size {
            spacing.push(' ');
        }
        println!("{}{}{}", spacing.on_white(), b, spacing.on_white())
    };
    let thin_bordered = |b: &str, size: usize| {
        let mut spacing = String::new();
        for _ in 0..(size - 1) {
            spacing.push(' ');
        }
        println!(
            "{}{}{}{}{}",
            " ".on_white(),
            spacing.on_black(),
            b,
            spacing.on_black(),
            " ".on_white()
        )
    };

    // Printing title
    println!("{}", "C M D L E".black().on_bright_white());
    //

    blank_line();

    // Printing guesses
    for guess in game.get_guess_iterator() {
        let results = game.compare_to_goal(guess);
        thin_bordered(&format_word(guess, results), 2);
    }
    for _ in 0..(6 - game.guess_num()) {
        thin_bordered(" ", 4);
    }
    //

    blank_line();

    // Printing goal
    let goal = game.get_goal().to_string().to_uppercase().black().clone();
    if game.is_won() {
        //Must convert to normal String to preserve color
        bordered(&goal.on_bright_green().to_string(), 2);
    } else if game.is_lost() {
        bordered(&goal.on_bright_red().to_string(), 2);
    } else {
        blank_line();
    }
    //

    blank_line();

    // Printing letter list
    let format_letter = |c: &char, status: &LetterStatus| {
        let c = c.to_string().to_uppercase();
        //I'm going to rename these later because I chose pretty confusing names for them
        match status {
            LetterStatus::FoundPosition => c.bright_green(),
            LetterStatus::FoundLetter => c.bright_yellow(),
            LetterStatus::NotPresent => c.red(),
            LetterStatus::Unused => c.underline(),
        }
    };

    let letters = game.letter_list();
    for i in 0..5 {
        for j in 0..5 {
            let c = format_letter(ALPHABET.get(i * 5 + j).unwrap(), &letters[i * 5 + j]);
            print!("{}", c);
            if j != 4 {
                print!(" ");
            }
        }
        println!();
    }
    let c = format_letter(&'z', &letters[25]);
    println!("    {}    ", c);
}

fn format_word(word: &Word, results: [LetterResult; 5]) -> String {
    let join = |a: String, b: String| format!("{}{}", a, b);
    let join_colored = |a: String, b: colored::ColoredString| format!("{}{}", a, b);
    let mut buffer = String::new();
    for i in 0..5 {
        match results[i] {
            LetterResult::Correct => {
                buffer = join_colored(
                    buffer,
                    word.get(i)
                        .to_string()
                        .to_uppercase()
                        .black()
                        .on_bright_green(),
                )
            }
            LetterResult::WrongPosition => {
                buffer = join_colored(
                    buffer,
                    word.get(i)
                        .to_string()
                        .to_uppercase()
                        .black()
                        .on_bright_yellow(),
                )
            }
            LetterResult::WrongLetter => {
                buffer = join(buffer, word.get(i).to_string().to_uppercase());
            }
        };
    }
    buffer
}
