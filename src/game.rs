use crate::{json_func::*, word::Word, LetterResult, LetterStatus, ALPHABET};
use json::{object, JsonValue};

#[cfg(test)]
mod tests {
    use super::*;

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

    pub fn letter_list(&self) -> [LetterStatus; 26] {
        let mut status = [LetterStatus::Unused; 26];

        self.get_guess_iterator()
            .map(|guess| (guess.to_string().to_owned(), self.compare_to_goal(guess)))
            .for_each(|(guess_str, result)| {
                for i in 0..5 {
                    let c = guess_str.chars().nth(i).unwrap();
                    let alphabet_pos = ALPHABET
                        .iter()
                        .position(|ch| &c == ch)
                        .expect(&format!("Couldn't find {} in alphabet", c));
                    match result[i] {
                        LetterResult::Correct => {
                            status[alphabet_pos] = LetterStatus::FoundPosition;
                        }
                        LetterResult::WrongPosition => {
                            if status[alphabet_pos] == LetterStatus::FoundPosition {
                                continue;
                            }
                            status[alphabet_pos] = LetterStatus::FoundLetter;
                        }
                        LetterResult::WrongLetter => {
                            status[alphabet_pos] = LetterStatus::NotPresent;
                        }
                    };
                }
            });

        status
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
        self.guesses.len() >= 6
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
        guess.comp(&self.goal)
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
        let goal: &str = self.goal.as_string();
        let guesses: Vec<String> = (&self.guesses)
            .into_iter()
            .map(|guess| guess.as_string().clone())
            .collect();
        object! {
            "goal": goal,
            "guesses": guesses
        }
    }
}
