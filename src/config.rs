use json::{object, JsonValue};

use crate::json_func::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_to_json() {
        let config = Config { test_int: 3 };
        let config_json = object! {
            test_int: 3
        };

        assert_eq!(config.to_json(), config_json);
    }

    #[test]
    fn json_to_config() {
        let config = Config { test_int: 3 };
        let config_json = object! {
            test_int: 3
        };

        assert_eq!(config, Config::from_json(config_json).unwrap());
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Config {
    pub test_int: i32,
}

impl Config {
    pub fn save_to_file(&self, path: &str) -> Result<(), &'static str> {
        save_json(self.to_json(), path)
    }

    pub fn from_file(path: &str) -> Result<Config, &'static str> {
        Self::from_json(load_json(path)?)
    }

    fn to_json(&self) -> JsonValue {
        object! {
            "test_int":self.test_int
        }
    }

    fn from_json(mut value: JsonValue) -> Result<Self, &'static str> {
        if !value.is_object() {
            return Err("Config is not a json object");
        }
        if !value.has_key("test_int") {
            return Err("Missing test int");
        }
        let test_int = value.remove("test_int");
        if !test_int.is_number() {
            return Err("Test int is not a number");
        }
        let test_int = test_int.as_i32().expect("Couldn't convert test int to i32");
        Ok(Config { test_int })
    }
}
