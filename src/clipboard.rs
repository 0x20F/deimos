use arboard::Clipboard;


pub struct ClipboardWrapper {
    instance: Clipboard
}

impl ClipboardWrapper {
    pub fn new() -> Self {
        Self {
            instance: Clipboard::new().unwrap()
        }
    }


    pub fn get(&mut self) -> String {
        match self.instance.get_text() {
            Ok(text) => text,
            Err(_) => String::from("")
        }
    }
    
    
    pub fn set(&mut self, to: String) {
        self.instance.set_text(to).unwrap()
    }
}
