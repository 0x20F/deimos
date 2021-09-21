mod queue;
mod config;
mod clipboard;
mod simulate;


use rdev::{
    listen, 
    Event,
    EventType,
};
use queue::Queue;



fn main() {
    let mut queue = Queue::new();
    
    let callback = move |event: Event| match event.event_type {
        EventType::KeyPress(_) => {
            queue.update(event);
        },
        _ => ()
    };


    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
