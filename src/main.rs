use rdev::{
    listen, 
    Event,
    EventType,
    Key
};




#[derive(Debug)]
struct Queue {
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
                Key::Space => self.keys = vec![],
                Key::Backspace => {
                    if self.keys.len() > 0 {
                        self.keys.pop();
                    }

                    ()
                },
                Key::ShiftLeft | Key::ShiftRight => (),
                _ => self.keys.push(event.name.unwrap())
            },
            _ => ()
        };
    }
}



fn main() {
    let mut queue = Queue::new();

    let callback = move |event: Event| {
        println!("Queue is: {:?}", queue);

        match event.event_type {
            EventType::ButtonPress(_) => (),
            EventType::ButtonRelease(_) => (),

            EventType::Wheel { .. } => (),
            EventType::MouseMove { .. } => (),

            EventType::KeyRelease(_) => (),
            EventType::KeyPress(k) => queue.update(event)
        }
    };


    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
