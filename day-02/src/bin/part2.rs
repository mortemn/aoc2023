use std::{fs, cmp};

fn main() {
    let file_path: &str = "./src/bin/input.txt";

    let input = &fs::read_to_string(file_path)
        .unwrap();

    println!("Result: {}", parse(input));
}

fn parse(input: &str) -> u32 {
    let lines = input.lines();
    let mut games: Vec<Vec<(u32, u32, u32)>> = Vec::new();
    let mut sum = 0;
    for line in lines {
        games.push(parse_line(line));
    }
    for (_, e) in games.iter().enumerate() {
        sum += get_power(e);
    }
    return sum;
}

fn parse_line(line: &str) -> Vec<(u32, u32, u32)> {
    let mut rounds: Vec<(u32, u32, u32)> = Vec::new();
    let index = line.to_string().chars().into_iter().position(|x| x == ':').unwrap();
    let removed = line.to_string().drain(index+1..).collect::<String>();
    let split_line = removed.split(";")
        .map(|x| x.split(",").collect::<Vec<&str>>());
    for set in split_line {
        let mut res: (u32, u32, u32) = (0, 0, 0);
        for colour in set {
             if colour.contains("blue") { res.0 = get_num(colour)};
             if colour.contains("red") { res.1 = get_num(colour)};
             if colour.contains("green") { res.2 = get_num(colour)};
        }
        rounds.push(res);
    }
    return rounds;
}

fn get_num(colour: &str) -> u32 {
    let mut num = String::new();
    for c in colour.chars() {
        if c.is_digit(10) {
            num.push(c);
        }
    }
    return num.parse::<u32>().unwrap();
}

fn check_possible(game: Vec<(u32, u32, u32)>) -> bool {
    let mut possible = true;
    for round in game {
        if round.0 > 14 || round.1 > 12 || round.2 > 13 {
            possible = false;
        }
    }
    return possible;
}

fn get_power(game: &Vec<(u32, u32, u32)>) -> u32 {
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;
    for round in game {
        blue = cmp::max(blue, round.0);
        red = cmp::max(red, round.1);
        green = cmp::max(green, round.2);
    }
    return blue * red * green;
}
