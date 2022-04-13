use json::{self, JsonValue};
use std::{fs::File, io::Read};

use crate::leak_into_str;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_save_load_json() {
        assert!(save_json(JsonValue::Null, "test.json").is_ok());
        assert!(load_json("test.json").is_ok());
        assert_eq!(load_json("test.json").unwrap(), JsonValue::Null);
    }
}

pub fn get_json_array(path: &str) -> Result<JsonValue, &str> {
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

pub fn save_json(json: JsonValue, path: &str) -> Result<(), &'static str> {
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

pub fn load_json(path: &str) -> Result<JsonValue, &'static str> {
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
