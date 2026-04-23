#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let first_list_len: usize = first_list.len();
    let second_list_len: usize = second_list.len();
    if first_list_len == 0 {
        if second_list_len == 0 {
            return Comparison::Equal;
        } else {
            return Comparison::Sublist;
        }
    } else if second_list_len == 0 {
        return Comparison::Superlist;
    }
    if first_list_len == second_list_len {
        for i in 0..first_list_len {
            if first_list.get(i) != second_list.get(i) {
                return Comparison::Unequal;
            }
        }
        return Comparison::Equal;
    } else if first_list_len < second_list_len {
        match second_list.windows(first_list_len).any(|w| w == first_list) {
            true => {
                return Comparison::Sublist;
            },
            false => {
                return Comparison::Unequal;
            }
        }
    } else {
        match first_list.windows(second_list_len).any(|w| w == second_list) {
            true => {
                return Comparison::Superlist;
            },
            false => {
                return Comparison::Unequal;
            }
        }
    }
}
