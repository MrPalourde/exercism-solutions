pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut number: u64 = n;
    let mut steps: u64 = 0;
    while number != 1 {
        if number.is_multiple_of(2) {
            number /= 2;
        } else {
            number *= 3;
            number += 1;
        }
        steps += 1;
    }
    Some(steps)
}
