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

    #[test]
    fn try_save_load_json() {
        assert!(save_json(JsonValue::Null, "test.json").is_ok());
        assert!(load_json("test.json").is_ok());
        assert_eq!(load_json("test.json").unwrap(), JsonValue::Null);
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Game {
    pub goal: Word,
    pub guesses: Vec<Word>,
}

impl Game {
    pub fn new(word: Word) -> Game {
        Game {
            goal: word,
            guesses: vec![],
        }
    }

    pub fn guess_num(&self) -> usize {
        self.guesses.len()
    }

    pub fn get_goal(&self) -> Word {
        self.goal.clone()
    }

    pub fn is_won(&self) -> bool {
        let last = self.guesses.last();
        if last.is_none() {
            return false;
        };
        last.unwrap() == &self.goal
    }

    pub fn is_lost(&self) -> bool {
        return self.is_full() && !self.is_won();
    }

    pub fn is_full(&self) -> bool {
        self.guesses.len() >= 5
    }

    pub fn add_guess(&mut self, guess: Word) {
        self.guesses.push(guess)
    }

    pub fn get_guess_iterator(&self) -> std::slice::Iter<Word> {
        return (&self.guesses).into_iter();
    }

    pub fn from_file(path: &str) -> Result<Game, &'static str> {
        Self::from_json(load_json(path)?)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), &'static str> {
        save_json(self.to_json(), path)
    }

    pub fn compare_to_goal(&self, guess: &Word) -> [LetterResult; 5] {
        comp_words(guess, &self.goal)
    }

    fn from_json(mut value: JsonValue) -> Result<Game, &'static str> {
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

    fn to_json(&self) -> JsonValue {
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LetterResult {
    Correct,
    WrongPosition,
    WrongLetter,
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

#[derive(Debug, Eq, PartialEq, Clone)]
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

    /// Panics if index is above 5
    pub fn get(&self, index: usize) -> char {
        if index >= 5 {
            panic!("Word index too large");
        }

        self.text.chars().nth(index).unwrap()
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

fn is_valid_guess(guess: &str) -> Result<bool, &'static str> {
    let guesses = get_json_array("guesses.json")?;

    Ok(guesses.members().any(|g| g.as_str().unwrap() == guess))
}

fn get_json_array(path: &str) -> Result<JsonValue, &str> {
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

fn save_json(json: JsonValue, path: &str) -> Result<(), &'static str> {
    let file_result = File::options()
        .write(true)
        .create(true)
        .read(false)
        .open(path);

    let mut file = match file_result {
        Err(e) => return Err(leak_into_str(format!("Failed to open {}: {:?}", path, e))),
        Ok(file) => file,
    };

    if let Err(e) = file.set_len(0) {
        return Err(leak_into_str(format!(
            "Failed to write to {}: {:?}",
            path, e
        )));
    }

    if let Err(e) = json.write_pretty(&mut file, 3) {
        return Err(leak_into_str(format!(
            "Failed to write to {}: {:?}",
            path, e
        )));
    }

    Ok(())
}

fn load_json(path: &str) -> Result<JsonValue, &'static str> {
    let mut contents = String::new();
    {
        let mut file = match File::open(path) {
            Err(e) => {
                return Err(leak_into_str(format!(
                    "Have you ran \"cmdle daily\"? Failed to open {}: {:?}",
                    path, e
                )))
            }
            Ok(file) => file,
        };
        if let Err(e) = file.read_to_string(&mut contents) {
            return Err(leak_into_str(format!("Failed to read {}: {:?}", path, e)));
        }
    }
    match json::parse(&contents) {
        Err(e) => return Err(leak_into_str(format!("Failed to parse {}: {:?}", path, e))),
        Ok(value) => Ok(value),
    }
}

fn comp_words(guess: &Word, goal: &Word) -> [LetterResult; 5] {
    let mut result = [LetterResult::WrongLetter; 5];
    for i in 0..5 {
        let guess_c = guess.get(i);
        let goal_c = goal.get(i);
        if guess_c == goal_c {
            result[i] = LetterResult::Correct;
        } else if goal.text.contains(guess_c) {
            result[i] = LetterResult::WrongPosition;
        }
    }
    result
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
