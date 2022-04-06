use clap::{Parser, Subcommand};
use cmdle::{get_daily_word, Game, LetterResult, Word};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    if let Err(e) = do_commands(&args, &mut stdout) {
        eprintln!("[ERR] {}", e);
    }
}

#[allow(unused_must_use)]
fn do_commands(args: &Args, mut out: &mut StandardStream) -> Result<(), &'static str> {
    Ok(match &args.command {
        Commands::Daily => {
            (Game::new(get_daily_word()?)).save_to_file("save.json")?;
            writeln!(&mut out, "Started new game with daily word.");
        }
        Commands::Guess { word } => {
            let game = Game::from_file("save.json")?;
            println!("{}", game.goal); //Debug
            let word = match Word::from(word.clone()) {
                Err(e) => return Err(e),
                Ok(word) => word,
            };

            let results = game.compare_to_goal(&word);
            let correct = ColorSpec::new()
                .set_fg(Some(Color::Black))
                .set_bg(Some(Color::Green))
                .to_owned();
            let wrong_pos = ColorSpec::new()
                .set_fg(Some(Color::Black))
                .set_bg(Some(Color::Yellow))
                .to_owned();
            let wrong_ltr = ColorSpec::new()
                .set_fg(Some(Color::Black))
                .set_bg(Some(Color::Red))
                .to_owned();
            let default = ColorSpec::new().to_owned();
            for i in 0..5 {
                match results[i] {
                    LetterResult::Correct => out.set_color(&correct),
                    LetterResult::WrongPosition => out.set_color(&wrong_pos),
                    LetterResult::WrongLetter => out.set_color(&wrong_ltr),
                };
                write!(out, "{}", word.get(i));
            }
            out.set_color(&default);
            write!(out, "\n");
        }
    })
}
