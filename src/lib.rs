use std::fmt;

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

        Ok(text)
    }
}

impl fmt::Display for Word<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}
