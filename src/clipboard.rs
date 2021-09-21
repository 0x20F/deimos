use arboard::Clipboard;


pub fn get() -> String {
    let mut clipboard = Clipboard::new().unwrap();

    clipboard.get_text().unwrap_or(String::from(""))
}


pub fn set(to: String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(to).unwrap()
}