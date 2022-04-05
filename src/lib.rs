use chrono::TimeZone;
use json::{self, object, JsonValue};
use std::{convert::Into, fmt, fs::File, io::Read};

const ALLOWED_CHARS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_guess() {
        assert_eq!(is_valid_guess("shave"), Ok(true))
    }

    #[test]
    fn game_to_json() {
        let game = Game {
            goal: Word::from("hello".to_string()).unwrap(),
            guesses: vec![
                Word::from("there".to_string()).unwrap(),
                Word::from("carts".to_string()).unwrap(),
                Word::from("fears".to_string()).unwrap(),
            ],
        };

        let json_game = object! {
            goal:"hello",
            guesses:[
                "there",
                "carts",
                "fears"
            ]
        };

        assert_eq!(game.to_json(), json_game);
    }

    #[test]
    fn json_to_game() {
        let game = Game {
            goal: Word::from("hello".to_string()).unwrap(),
            guesses: vec![
                Word::from("there".to_string()).unwrap(),
                Word::from("carts".to_string()).unwrap(),
                Word::from("fears".to_string()).unwrap(),
            ],
        };

        let json_game = object! {
            goal:"hello",
            guesses:[
                "there",
                "carts",
                "fears"
            ]
        };

        assert_eq!(game, Game::from_json(json_game).unwrap());
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Game {
    goal: Word,
    guesses: Vec<Word>,
}

impl Game {
    pub fn from_json(mut value: json::JsonValue) -> Result<Game, &'static str> {
        if !value.is_object() {
            return Err("Not valid json object");
        }
        if !value.has_key("goal") {
            return Err("Missing goal key");
        }
        if !value.has_key("guesses") {
            return Err("Missing guesses key");
        }

        let guesses: Vec<Word> = value
            .remove("guesses")
            .members()
            .filter_map(|guess| {
                let guess = match guess.as_str() {
                    Some(guess) => guess,
                    None => return None,
                };
                Word::from(guess.to_string()).ok()
            })
            .collect();

        if guesses.len() > 6 {
            return Err("Too many guesses");
        }

        let goal = Word::from(match value.remove("goal").as_str() {
            Some(goal) => goal.to_string(),
            None => return Err("Goal key is not valid string"),
        })?;

        Ok(Game { goal, guesses })
    }

    pub fn to_json(&self) -> JsonValue {
        let goal: &str = &self.goal.text;
        let guesses: Vec<String> = (&self.guesses)
            .into_iter()
            .map(|guess| guess.text.clone())
            .collect();
        object! {
            "goal": goal,
            "guesses": guesses
        }
    }
}

#[derive(Debug)]
enum LetterResult {
    Correct,
    WrongPosition,
    WrongLetter,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Word {
    text: String,
}

impl Word {
    pub fn from(text: String) -> Result<Word, &'static str> {
        Self::validate(text.trim())?;
        Ok(Word { text })
    }

    fn validate(text: &str) -> Result<(), &'static str> {
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

        if !is_valid_guess(&text)? {
            return Err("Not in the word dictionary.");
        }

        Ok(())
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.text)
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

fn get_daily_word() -> Result<Word, &'static str> {
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

fn is_valid_guess(guess: &str) -> Result<bool, &'static str> {
    let guesses = get_json_array("guesses.json")?;

    Ok(guesses.members().any(|g| g.as_str().unwrap() == guess))
}

fn get_json_array(path: &str) -> Result<json::JsonValue, &str> {
    let mut file = match File::open(path) {
        Err(_) => return Err(leak_into_str(format!("Could not open {}", path))),
        Ok(file) => file,
    };

    let mut arr = String::new();
    if let Err(_) = file.read_to_string(&mut arr) {
        return Err(leak_into_str(format!("{} is not valid utf8", path)));
    }

    let arr = match json::parse(&arr) {
        Err(_) => return Err(leak_into_str(format!("{} is not valid json", path))),
        Ok(arr) if !arr.is_array() => return Err(leak_into_str(format!("{} is malformed", path))),
        Ok(arr) => arr,
    };

    Ok(arr)
}

/// Creates a &'static str from a String by leaking it.
fn leak_into_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
