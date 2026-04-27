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
    song
}

fn is_plural(number: u32) -> String {
    if number == 1 {
        String::from("")
    } else {
        String::from("s")
    }
}

fn get_number_in_letter(number: u32) -> String {
    match number {
        10 => String::from("Ten"),
        9 => String::from("Nine"),
        8 => String::from("Eight"),
        7 => String::from("Seven"),
        6 => String::from("Six"),
        5 => String::from("Five"),
        4 => String::from("Four"),
        3 => String::from("Three"),
        2 => String::from("Two"),
        1 => String::from("One"),
        0 => String::from("No"),
        _ => String::from("Undefined")
    }
}
