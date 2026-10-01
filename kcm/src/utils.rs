pub fn lowercase_first_char(mut content: String) -> String {
    if let Some(character) = content.get_mut(0..1) {
        character.make_ascii_lowercase();
    }
    content
}

pub fn uppercase_first_char(mut content: String) -> String {
    if let Some(character) = content.get_mut(0..1) {
        character.make_ascii_uppercase();
    }
    content
}
