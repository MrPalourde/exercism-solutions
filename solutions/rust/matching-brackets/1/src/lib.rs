pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: Vec<char> = Vec::new();
    if string.is_empty() {
        return true; 
    };
    for chr in string.chars() {
        match chr {
            '{' => brackets.push('{'),
            '[' => brackets.push('['),
            '(' => brackets.push('('),
            '}' => {
                if brackets.last() != Some(&'{') {
                    return false;
                } else {
                    brackets.pop();
                }
            },
            ']' => {
                if brackets.last() != Some(&'[') {
                    return false;
                } else {
                    brackets.pop();
                }
            },
            ')' => {
                if brackets.last() != Some(&'(') {
                    return false;
                } else {
                    brackets.pop();
                }
            },
            _ => {}
        }
    }
    brackets.is_empty()
}
