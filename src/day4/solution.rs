use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

pub struct Day {
    winning_numbers: Vec<HashMap<i32, bool>>,
    numbers: Vec<Vec<i32>>,
}

impl Day {
    pub fn new(input_file: &str) -> Self {
        let file = File::open(input_file).expect("should be able to open the file");
        let reader = BufReader::new(file);

        let mut winning_numbers = Vec::new();
        let mut numbers = Vec::new();
        for line_res in reader.lines() {
            let line = line_res.expect("should be able to read the string");

            let cards = line.split(":").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
            let number_str = cards[1].split("|").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
            let winning_number_str = number_str[0].split(" ").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
            let your_number_str = number_str[1].split(" ").filter(|x| !x.is_empty()).collect::<Vec<&str>>();

            let mut winning_number: HashMap<i32, bool> = HashMap::new();
            for item in winning_number_str {
                winning_number.insert(
                    item.parse::<i32>().expect("should be number"),
                    true,
                );
            }
            winning_numbers.push(winning_number);

            let mut your_number: Vec<i32> = Vec::new();
            for item in your_number_str {
                your_number.push(
                    item.parse::<i32>().expect("should be number"),
                );
            }
            numbers.push(your_number);
        }


        Self {
            winning_numbers,
            numbers,
        }
    }

    pub fn solve(&self) -> i64 {
        let mut total: i64 = Default::default();

        for (index, card) in self.numbers.iter().enumerate() {
            let mut n_match: i32 = -1;
            for number in card {
                if self.winning_numbers[index].contains_key(number) {
                    n_match += 1;
                }
            }
            if n_match != -1 {
                total += 2_i64.pow(n_match as u32)
            }
        }

        total
    }

    pub fn solve2(&self) -> i64 {
        let mut total: i64 = Default::default();

        let mut copies: Vec<i64> = Default::default();
        for _ in &self.numbers {
            copies.push(1);
        }

        for (index, card) in self.numbers.iter().enumerate() {
            let mut n_match: i32 = -1;
            for number in card {
                if self.winning_numbers[index].contains_key(number) {
                    n_match += 1;
                }
            }

            if n_match != -1 {
                for i in index + 1..(index + 1) + (n_match as usize + 1) {
                    copies[i] += copies[index];
                }
            }
            total += copies[index]
        }

        total
    }
}
