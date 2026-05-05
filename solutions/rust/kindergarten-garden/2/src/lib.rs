pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_number: usize = student.chars().next().unwrap() as usize - 64;
    let mut result: Vec<&'static str> = Vec::new();
    let second_line: &str = diagram.split_whitespace().collect::<Vec<&str>>()[1];
    println!("{:?}", second_line);
    result.push(plant_to_text(diagram.chars().nth(student_number*2-2).unwrap()));
    result.push(plant_to_text(diagram.chars().nth(student_number*2-1).unwrap()));
    result.push(plant_to_text(second_line.chars().nth(student_number*2-2).unwrap()));
    result.push(plant_to_text(second_line.chars().nth(student_number*2-1).unwrap()));
    result
}

fn plant_to_text<'a>(plant_char: char) -> &'a str {
    match plant_char {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => "invalid",
    }
}
