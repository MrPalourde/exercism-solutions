pub fn abbreviate(phrase: &str) -> String {
    let mut result: String = String::new();
    if !phrase.is_empty() {
        result.push(phrase.chars().next().unwrap());
    }
    for (index, c) in phrase.char_indices() {
        if c.is_whitespace() || c == '-' || c == '_' {
            let chr: char = phrase.chars().nth(index+1).unwrap();
            if !chr.is_whitespace() && chr != '-' && chr != '_' {
                result.push(chr)
            }
        } else if index > 1 && c.is_uppercase() {
            let prechar: char = phrase.chars().nth(index-1).unwrap();
            if prechar.is_alphabetic() && prechar.is_lowercase() {
                result.push(phrase.chars().nth(index).unwrap());
            }
        }
    }
    result.to_uppercase()
}
