pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut prime_num: u32 = 0;
    let mut current_number: u32 = 2;
    let mut is_prime: bool;
    while prime_num != n {
        current_number += 1;
        is_prime = true;
        for i in 2..current_number {
            if current_number.is_multiple_of(i) && i != 1 && i != current_number {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            prime_num += 1;
        }
    }
    return current_number;
}
