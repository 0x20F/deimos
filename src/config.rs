use dirs::home_dir;
use std::fs;
use serde::Deserialize;
use std::collections::HashMap;


#[derive(Deserialize, Debug)]
pub struct Config {
    snippets: HashMap<String, String>
}

impl Config {
    pub fn new() -> Self {
        let mut config_path = home_dir().unwrap();
        config_path.push("deimos.toml");

        let file = fs::read_to_string(config_path).unwrap();
        let config: Config = toml::from_str(&file).unwrap();
        
        config
    }

    pub fn has_match(&self, pattern: &str) -> Option<String> {
        for (key, value) in self.snippets.iter() {
            if key == pattern {
                return Some(value.clone());
            }
        }

        None
    }
}
