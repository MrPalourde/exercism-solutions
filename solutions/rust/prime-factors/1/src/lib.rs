pub fn factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut number: u64 = n;
    while number.is_multiple_of(2) {
        result.push(2);
        number /= 2;
    }
    let mut d: u64 = 3;
    while d * d <= number {
        while number.is_multiple_of(d) {
            result.push(d);
            number /= d;
        }
        d += 2;
    }
    if number > 1 {
        result.push(number);
    }
    result
}
