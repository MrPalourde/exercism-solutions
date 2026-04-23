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
        match contain_in(&first_list, &second_list) {
            true => {
                return Comparison::Sublist;
            },
            false => {
                return Comparison::Unequal;
            }
        }
    } else {
        match contain_in(&second_list, &first_list) {
            true => {
                return Comparison::Superlist;
            },
            false => {
                return Comparison::Unequal;
            }
        }
    }
}


// first_list is the list with less numbers in it ; so if this function return true, first_list is a sublist of second_list
fn contain_in(first_list: &[i32], second_list: &[i32]) -> bool {
    let mut equal: bool = false;
    for mut i in 0..first_list.len() {
        let number_i: i32 = *first_list.get(i).unwrap();
        for j in 0..second_list.len() {
            if number_i == *second_list.get(j).unwrap() {
                equal = true;
                i+=1
            } else if j == second_list.len() {
                return false;
            }
        }
    }
    return true;
}
