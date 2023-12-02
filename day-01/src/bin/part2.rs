use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file_path: &str = "./src/bin/input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut num_list: Vec<u32> = Vec::new();
    let str_list: Vec<&str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut sum: u32 = 0;
    
    for line in reader.lines() {
        let mut list: Vec<char> = Vec::new();
        let mut list1: Vec<char> = Vec::new();
        let mut list2: Vec<char> = Vec::new();

        let mut line1 = line.unwrap();
        let mut line2 = line1.clone();
        let mut first_word: &str = "";
        let mut last_word: &str = "";
        loop {
            let mut i = 0;
            let mut min: usize = 100;
            let mut replaced_word = false;
            while i < 9 {
                let n = line1.find(str_list[i]);
                if n != None {
                    let n = n.unwrap();
                    if n < min {
                        min = n;
                        first_word = str_list[i];
                        replaced_word = true;
                    }
                }
                i += 1;
            }
            if replaced_word {
                let word_to_num = &((str_list.iter().position(|&x| x == first_word).unwrap())+1).to_string();
                line1 = line1.replace(first_word, word_to_num);
            } else {
                break;
            }
        }

        loop {
            let mut i = 0;
            let mut max: usize = 0;
            let mut replaced_word = false;
            while i < 9 {
                let n = line2.rfind(str_list[i]);
                if n != None {
                    let n = n.unwrap();
                    if n > max {
                        max = n;
                        last_word = str_list[i];
                        replaced_word = true;
                    }
                }
                i += 1;
            }
            if replaced_word {
                let word_to_num = &((str_list.iter().position(|&x| x == last_word).unwrap())+1).to_string();
                line2 = line2.replace(last_word, word_to_num);
            } else {
                break;
            }
        }

        for c in line1.chars() {
            if c.is_numeric() {
                list1.push(c);
            }
        }
        
        for c in line2.chars() {
            if c.is_numeric() {
                list2.push(c);
            }
        }

        if list1[list1.len()-1] != list2[list2.len()-1] {
            list = list1.clone();
            list[list1.len()-1] = list2[list2.len()-1];
        } else {
            list = list1;
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
