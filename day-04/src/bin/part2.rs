use std::{fs, collections::HashMap};

fn main() {
    let file_path: &str = "./src/bin/input.txt";

    let input = &fs::read_to_string(file_path)
        .unwrap();

    println!("Result: {}", parse(input));
}


fn parse(input: &str) -> u32 {
    let mut card_to_copy = HashMap::new();
    card_to_copy.insert(1, 1);
    let mut cards: Vec<(Vec<u32>, Vec<u32>)> = input
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
    cards.remove(0);
    let sum = cards
        .iter()
        .enumerate()
        .map(|(i, x)| {
            card_to_copy.entry(i as u32 + 1).or_insert(1);
            get_cards(&x.0, &x.1, &mut card_to_copy, i as u32 + 1)
        })
        .sum::<u32>();
    return sum;
}

fn get_cards(c1: &Vec<u32>, c2: &Vec<u32>, card_to_copy: &mut HashMap<u32, u32>, card_num: u32) -> u32 {
    let mut matches = 0;
    for i in 0..c2.len() {
        if c1.contains(&c2[i]) {
            matches += 1;
        }
    }
    for i in 0..matches {
        let n = i + card_num + 1;
        *card_to_copy.entry(n as u32).or_insert(1) += card_to_copy[&card_num];
    }
    return card_to_copy[&card_num];
}
