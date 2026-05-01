pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut factors_list: Vec<u32> = Vec::new();
    for num in factors {
        if *num != 0 {
            let mut factor: u32 = 1;
            while num * factor < limit {
                factors_list.push(num * factor);
                factor += 1;
            }
        }
    }
    factors_list.sort();
    factors_list.dedup();
    factors_list.iter().sum()
}
