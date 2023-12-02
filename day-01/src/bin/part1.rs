use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let file_path: &str = "./src/bin/input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut num_list: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;
    
    for line in reader.lines() {
        let mut list: Vec<char> = Vec::new();

        for c in line.unwrap().chars() {
            if c.is_numeric() {
                list.push(c);
            }
        }

        if list.len() >= 2 {
            let mut n1: String = list[0].to_string();
            let n2: char = list[list.len()-1];
            n1.push(n2);
            let nt: u32 = n1.parse::<u32>().unwrap();
            num_list.push(nt);
        } else if list.len() == 1 {
            let n: u32 = list[0].to_digit(10).unwrap() * 11;
            num_list.push(n);
        }
    }

    for num in num_list {
        sum += num;
    }

    println!("Sum: {}", sum);

}
