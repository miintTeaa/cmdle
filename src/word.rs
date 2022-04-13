use std::{collections::HashMap, fmt};

use crate::{json_func::get_json_array, LetterResult, ALPHABET};

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

        let contains_bad_char = text.chars().find(|s| !ALPHABET.contains(&s)).is_some();
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

    pub fn as_string(&self) -> &String {
        &self.text
    }

    pub fn comp(&self, goal: &Word) -> [LetterResult; 5] {
        let mut result = [LetterResult::WrongLetter; 5];
        // HashMap where each key is a letter in the goal, and the value is how many times that letter shows up
        // Does not have entries for letters that are not in the goal
        let mut letter_count =
            goal.text
                .clone()
                .chars()
                .fold(HashMap::new(), |mut acc: HashMap<char, u32>, c| {
                    if !acc.contains_key(&c) {
                        acc.insert(c, 1);
                        return acc;
                    }
                    acc.insert(c, acc.get(&c).unwrap() + 1);
                    acc
                });

        for i in 0..5 {
            let self_c = self.get(i);
            let goal_c = goal.get(i);

            let count = *letter_count.get(&self_c).unwrap_or(&0);

            if count <= 0 {
                continue;
            }

            if self_c == goal_c {
                result[i] = LetterResult::Correct;
                letter_count.insert(self_c, count - 1);
            } else if goal.text.contains(self_c) {
                result[i] = LetterResult::WrongPosition;
                letter_count.insert(self_c, count - 1);
            }
        }
        result
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.text)
    }
}

pub fn is_valid_guess(guess: &str) -> Result<bool, &'static str> {
    let guesses = get_json_array("guesses.json")?;

    Ok(guesses.members().any(|g| g.as_str().unwrap() == guess))
}
