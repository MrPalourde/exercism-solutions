pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut vec = Vec::new();
    let line = garden.len();
    if line == 0 {
        return vec;
    }
    let column = garden[0].split(',').count();
    for i in 0..line*column {
        vec.push(garden[i].to_string());
    }
    for i in 0..vec.len() {
        let mut string: String = "".to_string();
        for j in 0..vec[i].len() {
            if *vec[i].as_bytes().get(j).unwrap() == 42 {
                string.push('*');
            } else {
                let nb_flowers: String = {
                    let mut numbers: u32 = 0;
                    for di in -1..=1 {
                        for dj in -1..=1 {
                            if di == 0 && dj == 0 {
                                continue;
                            }
                            let ni = i as isize + di;
                            let nj = j as isize + dj;

                            if ni >= 0 && ni < vec.len() as isize && nj >= 0 && nj < vec[i].len() as isize {
                                let ni = ni as usize;
                                let nj = nj as usize;
                                if *vec[ni].as_bytes().get(nj).unwrap() == 42 {
                                    numbers += 1;
                                }
                            }
                        }
                    }
                    numbers.to_string() 
                };
                let mut c = nb_flowers.chars().next().unwrap();
                if c == '0' {
                    c = ' ';
                }
                string.push(c);
            }
        }
        vec[i] = string;
    }
    return vec;
}
