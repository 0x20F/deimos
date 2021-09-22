use rdev::{
    Event,
    EventType,
    Key
};
use std::fmt;

use crate::config::Config;
use crate::clipboard::ClipboardWrapper;
use crate::simulate;
use std::{ thread, time };



pub struct Queue {
    keys: Vec<String>,
    clipboard: ClipboardWrapper,
    pub config: Config
}

impl Queue {
    pub fn new() -> Self {
        Self {
            keys: vec![],
            config: Config::new(),
            clipboard: ClipboardWrapper::new()
        }
    }

    pub fn update(&mut self, event: Event) {
        match event.event_type {
            EventType::KeyPress(k) => match k {
                Key::Return => self.keys = vec![],
                Key::Space => {
                    // Check if the current pattern is defined in the
                    // config file before emptying.
                    match self.config.has_match(&self.to_string()) {
                        Some(value) => {
                            // Delete the alias characters
                            simulate::backspace(self.to_string().len());

                            // Get the current clipboard data
                            let current = self.clipboard.get();

                            // Set the clipboard to the found snippet
                            self.clipboard.set(value);

                            // Paste the snippet
                            simulate::paste();

                            // Set the clipboard back to what it was before the snippet
                            // A bit of sleep is needed so the OS can have enough time
                            // to paste the snippet value before we override it with the
                            // old clipboard value.
                            thread::sleep(time::Duration::from_millis(20));
                            self.clipboard.set(current);
                        },
                        None => ()
                    };

                    self.keys = vec![];
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


