use rdev::{ simulate, Key, EventType, SimulateError };


fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("Could not send keypress event {:?}", event_type);
        }
    }
}



pub fn paste() {
    send(&EventType::KeyPress(Key::ControlLeft));
    send(&EventType::KeyPress(Key::KeyV));

    send(&EventType::KeyRelease(Key::ControlLeft));
    send(&EventType::KeyRelease(Key::KeyV));
}


pub fn backspace(count: usize) {
    for _ in 0..count - 1 {
        send(&EventType::KeyPress(Key::Backspace));
        send(&EventType::KeyRelease(Key::Backspace));
    }
}
