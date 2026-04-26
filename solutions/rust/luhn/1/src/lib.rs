/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut string: String = String::new();
    for c in code.chars() {
        if c.is_digit(10) {
            string.push(c);
        } else if !c.is_whitespace() {
            return false;
        }
    }
    let digits: Vec<u32> = string.chars().map(|c| c.to_digit(10).unwrap()).collect();
    if digits.len() <= 1 { return false; }
    check_luhn(digits)
}

fn check_luhn(digits: Vec<u32>) -> bool {
    let mut sum: u32 = 0;
    for (i, mut number) in digits.into_iter().rev().enumerate() {
        if i % 2 == 1 {
            number *= 2;
            if number > 9 {
                number -= 9;
            }
        }
        sum += number;
    }
    sum % 10 == 0
}
