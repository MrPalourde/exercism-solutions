use std::str;

pub fn series(digits: &str, len: usize) -> Vec<String> {
    let groups_window = digits.as_bytes().windows(len).enumerate();
    let mut groups: Vec<String> = Vec::new();
    for group in groups_window {
        let current_group: String = unsafe {
            str::from_utf8_unchecked(group.1).to_string()
        };
        groups.push(current_group);
    }
    return groups;
}
