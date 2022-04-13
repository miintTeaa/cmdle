use chrono::TimeZone;
use std::fmt;

use crate::{json_func::*, word::*};

//use crate::game::Game;

pub const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub mod config;
pub mod game;
pub mod json_func;
pub mod word;

#[cfg(test)]
mod tests {
    use crate::word::is_valid_guess;

    #[test]
    fn make_guess() {
        assert_eq!(is_valid_guess("shave"), Ok(true))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LetterResult {
    Correct,
    WrongPosition,
    WrongLetter,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LetterStatus {
    FoundLetter,
    FoundPosition,
    NotPresent,
    Unused,
}

impl fmt::Display for LetterResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        if self == &Self::Correct {
            write!(f, "{}", "O")
        } else if self == &Self::WrongPosition {
            write!(f, "{}", "X")
        } else {
            write!(f, "{}", "-")
        }
    }
}

fn get_day(length: usize) -> usize {
    let wordle_epoch = chrono::Local.ymd(2021, 06, 19);
    let date_now = chrono::Local::today();

    let date_diff: usize = date_now
        .signed_duration_since(wordle_epoch)
        .num_days()
        .try_into()
        .unwrap();

    date_diff % length
}

pub fn get_daily_word() -> Result<Word, &'static str> {
    let goals = get_json_array("goals.json")?;

    let goals_count = goals.len();

    let word = goals
        .members()
        .nth(get_day(goals_count))
        .unwrap()
        .as_str()
        .expect("Word empty");

    Ok(Word::from(word.to_owned())?)
}

pub fn setup_cwd() -> Result<(), &'static str> {
    let path = match std::env::current_exe() {
        Ok(exe_path) => exe_path,
        Err(_) => return Err("Failed to get executable path"),
    };
    let path = match path.as_path().parent() {
        Some(path) => path,
        None => return Err("Could not get exe parent dir"),
    };
    if let Err(_) = std::env::set_current_dir(path) {
        return Err("Failed to set working directory");
    }
    Ok(())
}

/// Creates a &'static str from a String by leaking it.
fn leak_into_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
