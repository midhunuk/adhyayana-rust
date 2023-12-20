fn main() {
    let plain_text = String::from("Attack Montpellier at Dawn");
    let caeser_secret = caesar_cipher(&plain_text, 4);
    println!("Caesar Secret: {}", caeser_secret);
}

fn caesar_cipher(message: &str, shift: i32) -> String {
    message
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shifted = (c as u8 + shift as u8 - base) % 26 + base;
                shifted as char
            } else {
                c
            }
        })
        .collect()
}
