use std::fs;

fn main() {
    let file_path: &str = "./src/bin/input.txt";

    let input = &fs::read_to_string(file_path)
        .unwrap();

    println!("Result: {}", parse(input));
}

fn parse(input: &str) -> u32 {
    let empty: Vec<(u32, u32)> = vec![];
    let line_split: Vec<&str> = input.split("\n").collect();
    let check_char: Vec<(u32, u32)> = input.lines().enumerate().fold(empty, |mut acc, (i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c.is_ascii_punctuation() && c != '.' {
                acc.push((i as u32, j as u32));
            }
        });
        acc
    });
    let sum: u32 = check_char.iter().fold(0, |mut acc, (i, j)| {
        let mut checked: Vec<(u32, u32)> = Vec::new();
        let mut checks: Vec<(u32, u32)> = vec![
            (*i, *j + 1),
            (*i + 1, *j),
            (*i + 1, *j + 1),
        ];
        if *i > 0 && *j > 0 {
            checks.push((*i - 1, *j - 1));
        }
        if *i > 0 {
            checks.push((*i - 1, *j));
            checks.push((*i - 1, *j + 1));
        }
        if *j > 0 {
            checks.push((*i, *j - 1));
            checks.push((*i + 1, *j - 1));
        }
        for check in checks {
            if check.0 < line_split.len().try_into().unwrap() && check.1 < line_split[check.0 as usize].len().try_into().unwrap()  && line_split[check.0 as usize].to_string().chars().nth(check.1 as usize).unwrap().is_digit(10) {
                   acc += get_value(line_split[check.0 as usize], check, &mut checked); 
            }
        }
        acc
    });
    return sum;
}

fn get_value(line_split: &str, check: (u32, u32), checked: &mut Vec<(u32, u32)>) -> u32 {
    let mut as_str: String = "".to_string();
    for i in check.1..line_split.len() as u32 {
        let c = line_split.chars().nth(i as usize).unwrap();
        if !c.is_digit(10) {
            break;
        } else {
            if checked.contains(&(check.0, i)) {
                return 0;
            }
            as_str.push(c);
            checked.push((check.0, i));
        }
    }
    for i in (0..check.1).rev() {
        let c = line_split.chars().nth(i as usize).unwrap();
        if !c.is_digit(10) {
            break;
        } else {
            if checked.contains(&(check.0, i)) {
                return 0;
            }
            as_str.insert(0, c);
            checked.push((check.0, i));
        }
    }
    return as_str.parse::<u32>().unwrap();
}
