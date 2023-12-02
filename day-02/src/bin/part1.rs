use std::fs;

fn main() {
    let file_path: &str = "./src/bin/input.txt";

    let input = &fs::read_to_string(file_path)
        .unwrap();

    let result = parse(input);
}

fn parse(input: &str) -> usize {
    let lines = input.lines();
    let mut games: Vec<Vec<(u32, u32, u32)>> = Vec::new();
    let mut sum = 0;
    for line in lines {
        games.push(parse_line(line));
    }
    for (i, e) in games.iter().enumerate() {
        if check_possible(e.to_vec()) {
            println!("Game {} is possible", i+1);
            sum += i+1;
        }
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
