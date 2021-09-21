use rdev::{
    Event,
    EventType,
    Key
};
use std::fmt;



#[derive(Debug)]
pub struct Queue {
    keys: Vec<String>
}

impl Queue {
    pub fn new() -> Self {
        Self {
            keys: vec![]
        }
    }

    pub fn update(&mut self, event: Event) {
        match event.event_type {
            EventType::KeyPress(k) => match k {
                Key::Space | Key::Return => {
                    // Check if the current pattern is defined in the
                    // config file before emptying.
                    println!("Exec was pressed: {}", self.to_string());

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

