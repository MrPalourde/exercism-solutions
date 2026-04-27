pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song: String = String::new();
    for i in 0..take_down {
        println!("{start_bottles} ");
        let first_part: String = format!("{} green bottle{} hanging on the wall,\n",
                      get_number_in_letter(start_bottles-i),
                      is_plural(start_bottles-i)).to_string();
        song.push_str(&first_part);
        song.push_str(&first_part);

        let second_part: String = format!("And if one green bottle should accidentally fall,
There'll be {} green bottle{} hanging on the wall.\n\n",
                        get_number_in_letter(start_bottles-i-1).to_lowercase(),
                        is_plural(start_bottles-i-1)).to_string();
        song.push_str(&second_part);
    }
    return song;
}

fn is_plural(number: u32) -> String {
    if number == 1 {
        return String::from("");
    } else {
        return String::from("s");
    }
}

fn get_number_in_letter(number: u32) -> String {
    match number {
        10 => return String::from("Ten"),
        9 => return String::from("Nine"),
        8 => return String::from("Eight"),
        7 => return String::from("Seven"),
        6 => return String::from("Six"),
        5 => return String::from("Five"),
        4 => return String::from("Four"),
        3 => return String::from("Three"),
        2 => return String::from("Two"),
        1 => return String::from("One"),
        0 => return String::from("No"),
        _ => return String::from("Undefined")
    }
}
