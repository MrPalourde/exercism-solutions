pub fn egg_count(display_value: u32) -> usize {
    let mut result: usize = 0;
    let binary: String = format!("{:#010b}", display_value);
    for i in binary.chars() {
        if i == '1' { result += 1; }
    }
    result
}
