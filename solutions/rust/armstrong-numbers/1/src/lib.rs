pub fn is_armstrong_number(num: u32) -> bool {
    let mut result: u32 = 0;
    let number_of_digit: usize = num.to_string().len();
    for i in num.to_string().chars() {
        let digit = i.to_digit(10).unwrap();
        result += digit.pow(number_of_digit as u32);
    }
    num == result
}
