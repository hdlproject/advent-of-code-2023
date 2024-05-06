use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

pub struct Day {
    times: Vec<u64>,
    distances: Vec<u64>,
    merged_time: u64,
    merged_distance: u64,
}

impl Day {
    pub fn new(input_file: &str) -> Self {
        let file = File::open(input_file).expect("should be able to open the file");
        let reader = BufReader::new(file);

        let mut times = Vec::new();
        let mut distances = Vec::new();
        let mut merged_time: String = String::default();
        let mut merged_distance: String = String::default();
        for line_res in reader.lines() {
            let line = line_res.expect("should be able to read the string");

            let parts = line.split(" ").filter(|x| !x.is_empty()).collect::<Vec<&str>>();

            match parts[0] {
                "Time:" => {
                    for part in &parts[1..] {
                        times.push(part.parse::<u64>().expect("should number"));
                        merged_time += part;
                    }
                }
                "Distance:" => {
                    for part in &parts[1..] {
                        distances.push(part.parse::<u64>().expect("should number"));
                        merged_distance += part;
                    }
                }
                _ => {}
            }
        }


        Self {
            times,
            distances,
            merged_time: merged_time.parse::<u64>().expect("should number"),
            merged_distance: merged_distance.parse::<u64>().expect("should number"),
        }
    }

    pub fn solve(&self) -> i64 {
        let mut total: i64 = 1;

        for (index, time) in self.times.iter().enumerate() {
            let mut sub_total: i64 = Default::default();
            for i in 0..time + 1 {
                let calculated_distance = (time - i) * i;
                if calculated_distance > self.distances[index] {
                    sub_total += 1;
                }
            }
            total *= sub_total;
        }

        total
    }

    pub fn solve2(&self) -> i64 {
        let mut total: i64 = 1;

        let mut left_pointer: u64 = 0;
        let mut right_pointer: u64 = self.merged_time;
        let mut is_left_found: bool = false;
        let mut is_right_found: bool = false;
        while !(is_left_found && is_right_found) {
            if !is_left_found {
                let calculated_distance = (self.merged_time - left_pointer) * left_pointer;
                if calculated_distance > self.merged_distance {
                    is_left_found = true;
                } else {
                    left_pointer += 1;
                }
            }
            if !is_right_found {
                let calculated_distance = (self.merged_time - right_pointer) * right_pointer;
                if calculated_distance > self.merged_distance {
                    is_right_found = true;
                } else {
                    right_pointer -= 1;
                }
            }
        }

        total = (right_pointer - left_pointer) as i64 + 1;
        total
    }
}
