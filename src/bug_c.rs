// Bug C: panicking index access on user-controlled position.
pub fn first_word(s: &str) -> &str {
    let words: Vec<&str> = s.split_whitespace().collect();
    words[0]
}
