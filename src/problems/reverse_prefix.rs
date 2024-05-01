pub fn reverse_prefix(word: String, ch: char) -> String {
    if let Some(index) = word.find(ch) {
        word[0..index + 1]
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
        + &word[index + 1..word.len()].to_string()
    } else {
        word
    }
}