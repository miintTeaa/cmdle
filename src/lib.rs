use chrono::TimeZone;
use json;
use std::{fmt, fs::File, io::Read};

const ALLOWED_CHARS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

#[derive(Debug)]
pub struct Word<'a> {
    text: &'a str,
}

impl Word<'_> {
    pub fn from(text: &str) -> Result<Word, &str> {
        let text = Self::validate(text.trim())?;
        Ok(Word { text })
    }

    fn validate(text: &str) -> Result<&str, &str> {
        if text.chars().count() != 5 {
            return Err("Word must be 5 characters exactly.");
        }

        if !text.is_ascii() {
            return Err("Text must be ascii.");
        }

        let contains_bad_char = text.chars().find(|s| !ALLOWED_CHARS.contains(&s)).is_some();
        if contains_bad_char {
            return Err("Text must be alphabetical.");
        }

        if !is_valid_guess(text)? {
            return Err("Not in the word dictionary.");
        }

        Ok(text)
    }
}

impl fmt::Display for Word<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

fn get_day(length: usize) -> usize {
    let wordle_epoch = chrono::Local.ymd(2021, 06, 19);
    let date_now = chrono::Local::today();
    //TODO: Add wrapping to this
    date_now
        .signed_duration_since(wordle_epoch)
        .num_days()
        .try_into()
        .unwrap()
}

fn get_daily_word<'a>() -> Result<String, &'a str> {
    let mut goals_file = match File::open("goals.json") {
        Err(_) => return Err("Could not open goals.json"),
        Ok(file) => file,
    };

    let mut goals = String::new();
    if let Err(_) = goals_file.read_to_string(&mut goals) {
        return Err("goals.json is not valid utf8");
    }
    let goals = match json::parse(&goals) {
        Err(_) => return Err("goals.json is not valid json"),
        Ok(goals) if !goals.is_array() => return Err("goals.json is malformed"),
        Ok(goals) => goals,
    };

    let goals_count = goals.len();

    let word = goals
        .members()
        .nth(get_day(goals_count))
        .unwrap()
        .as_str()
        .expect("Word empty");

    Ok(word.to_owned())
}

fn is_valid_guess(guess: &str) -> Result<bool, &str> {
    let mut guess_file = match File::open("goals.json") {
        Err(_) => return Err("Could not open goals.json"),
        Ok(file) => file,
    };

    let mut guesses = String::new();
    if let Err(_) = guess_file.read_to_string(&mut guesses) {
        return Err("goals.json is not valid utf8");
    }

    let guesses = match json::parse(&guesses) {
        Err(_) => return Err("goals.json is not valid json"),
        Ok(guesses) if !guesses.is_array() => return Err("goals.json is malformed"),
        Ok(guesses) => guesses,
    };

    Ok(guesses.members().any(|g| g == guess))
}
