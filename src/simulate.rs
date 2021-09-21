use rdev::{ simulate, Key, EventType, SimulateError };
use std::{ thread, time };


fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);

    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("Could not send keypress event {:?}", event_type);
        }
    }

    // Let the OS catch up (at least MacOS)
    thread::sleep(delay);
}



pub fn paste() {
    send(&EventType::KeyPress(Key::ControlLeft));
    send(&EventType::KeyPress(Key::KeyV));

    send(&EventType::KeyRelease(Key::ControlLeft));
    send(&EventType::KeyRelease(Key::KeyV));
}


pub fn clear() {
    send(&EventType::KeyPress(Key::ControlLeft));
    send(&EventType::KeyPress(Key::Backspace));

    send(&EventType::KeyRelease(Key::ControlLeft));
    send(&EventType::KeyRelease(Key::Backspace));
}
