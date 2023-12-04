use std::fs;

fn main() {
    let file_path: &str = "./src/bin/input.txt";

    let input = &fs::read_to_string(file_path)
        .unwrap();

    println!("Result: {}", parse(input));
}


fn parse(input: &str) -> u32 {
    let cards: Vec<(Vec<u32>, Vec<u32>)> = input
        .lines()
        .fold(vec![(vec![], vec![])], |mut acc, line| {
            let line = line[(line.find(':').unwrap() + 2)..].to_string();
            let arr: Vec<Vec<u32>> = line.split('|')
                .map(|x| { 
                    x.split(' ')
                    .filter_map(|x| x.parse::<u32>().ok())
                    .collect::<Vec<u32>>()})
                .collect();
            acc.push((arr[0].clone(), arr[1].clone()));
            acc
        });
    let sum: u32 = cards.iter()
        .map(|card| {
            compare(&card.0, &card.1)
        })
        .sum();
    return sum;
}

fn compare(c1: &Vec<u32>, c2: &Vec<u32>) -> u32 {
    let mut n = 0;
    for i in 0..c2.len() {
        if c1.contains(&c2[i]) {
            n += 1;
        } 
    }
    if n == 0 {
        return 0;
    } else {
        return 2u32.pow(n-1);
    }
}
