use rdev::{
    Event,
    EventType,
    Key
};
use std::fmt;

use crate::config::Config;
use crate::clipboard;
use crate::simulate;



pub struct Queue {
    keys: Vec<String>,
    pub config: Config
}

impl Queue {
    pub fn new() -> Self {
        Self {
            keys: vec![],
            config: Config::new()
        }
    }

    pub fn update(&mut self, event: Event) {
        match event.event_type {
            EventType::KeyPress(k) => match k {
                Key::Space | Key::Return => {
                    // Check if the current pattern is defined in the
                    // config file before emptying.
                    match self.config.has_match(&self.to_string()) {
                        Some(value) => {
                            // Get the current clipboard data
                            let current = clipboard::get();

                            // Set the clipboard to the found snippet
                            clipboard::set(value);
                            
                            // Delete the alias characters
                            simulate::clear();

                            // Paste the snippet
                            simulate::paste();

                            // Set the clipboard back to what it was before the snippet
                            clipboard::set(current);
                        },
                        None => ()
                    };

                    self.keys = vec![]
                },
                Key::Backspace => {
                    if self.keys.len() > 0 {
                        self.keys.pop();
                    }

                    ()
                },
                Key::Escape => (),
                _ => match event.name {
                    Some(string) => self.keys.push(string),
                    None => ()
                }
            },
            _ => ()
        }
    }
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        write!(f, "{}", self.keys.join(""))
    }
}


