use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day {
    histories: Vec<Vec<i64>>,
}

impl Day {
    pub fn new(input_file: &str) -> Day {
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines();

        let mut histories: Vec<Vec<i64>> = Default::default();
        for line in lines {
            let line_res = line.unwrap();
            let parsed_line: Vec<i64> = line_res
                .split(" ")
                .filter_map(|x| x.parse::<i64>().ok())
                .collect();
            histories.push(parsed_line);
        }

        Day { histories }
    }

    pub fn solve(&self) -> u64 {
        let mut nexts: Vec<i64> = Default::default();

        for history in &self.histories {
            let mut last_numbers: Vec<i64> = Default::default();
            last_numbers.push(*history.last().unwrap());

            let mut diffs: Vec<i64> = history.clone();
            while !diffs.iter().all(|&x| x == 0) {
                diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect();

                last_numbers.push(*diffs.last().unwrap());
            }

            let last_number_sum = last_numbers.iter().sum();
            nexts.push(last_number_sum);
        }

        let total: i64 = nexts.iter().sum();
        total as u64
    }

    pub fn solve2(&self) -> u64 {
        let mut prevs: Vec<i64> = Default::default();

        for history in &self.histories {
            let mut first_numbers: Vec<i64> = Default::default();
            first_numbers.push(*history.first().unwrap());

            let mut diffs: Vec<i64> = history.clone();
            while !diffs.iter().all(|&x| x == 0) {
                diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect();

                first_numbers.insert(0, *diffs.first().unwrap());
            }

            let first_number_fold = first_numbers
                .iter()
                .skip(1)
                .fold(first_numbers[0], |acc, &x| x - acc);
            prevs.push(first_number_fold);
        }

        let total: i64 = prevs.iter().sum();
        total as u64
    }
}
