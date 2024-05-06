use std::char::MAX;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day {
    number_strings: [&'static str; 9],
    number_symbols: [&'static str; 9],
    input: Vec<String>,
}

impl Day {
    pub fn new(input_file: &str) -> Self {
        let file = File::open(input_file).expect("should be able to open the file");
        let reader = BufReader::new(file);

        let mut input_vec = Vec::new();
        for line in reader.lines() {
            input_vec.push(line.expect("should be able to read the string"))
        }

        Self {
            number_strings: ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"],
            number_symbols: ["1", "2", "3", "4", "5", "6", "7", "8", "9"],
            input: input_vec,
        }
    }

    pub fn solve(&self) -> u64 {
        let mut total: u64 = Default::default();
        for line in &self.input {
            let chars = line.split("").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();
            let mut left = 0;
            let mut right = line.len() - 1;
            let mut number_str: String = Default::default();

            while left < line.len() {
                let left_res = chars[left].parse::<u64>();
                match left_res {
                    Ok(_) => {
                        number_str.push_str(chars[left]);
                        break;
                    }
                    _ => {
                        left += 1;
                    }
                };
            }

            while right >= 0 {
                let right_res = chars[right].parse::<u64>();
                match right_res {
                    Ok(_) => {
                        number_str.push_str(chars[right]);
                        break;
                    }
                    _ => {
                        if right == 0 {
                            break;
                        }
                        right -= 1;
                    }
                };
            }

            let number = number_str.parse::<u64>().unwrap();
            total += number;
        }

        total
    }

    pub fn solve2(&self) -> u64 {
        let mut total: u64 = Default::default();
        for line in &self.input {
            let chars = line.split("").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();
            let mut left = 0;
            let mut right = line.len() - 1;
            let mut number_str: String = Default::default();

            while left < line.len() {
                let left_res = chars[left].parse::<u64>();
                match left_res {
                    Ok(_) => {
                        break;
                    }
                    _ => {
                        left += 1;
                    }
                };
            }

            while right >= 0 {
                let right_res = chars[right].parse::<u64>();
                match right_res {
                    Ok(_) => {
                        break;
                    }
                    _ => {
                        if right == 0 {
                            break;
                        }
                        right -= 1;
                    }
                };
            }

            let mut left_hardcoded_index = line.len();
            let mut right_hardcoded_index = 0;
            let mut left_hardcoded_number = "";
            let mut right_hardcoded_number = "";
            for (index, item) in self.number_strings.iter().enumerate() {
                let finding: Vec<_> = line.match_indices(item).collect();
                for (i_finding, _) in finding {
                    if i_finding < left_hardcoded_index {
                        left_hardcoded_index = i_finding;
                        left_hardcoded_number = self.number_symbols[index]
                    }
                    if i_finding > right_hardcoded_index {
                        right_hardcoded_index = i_finding;
                        right_hardcoded_number = self.number_symbols[index]
                    }
                }
            }

            if left_hardcoded_index < left {
                number_str.push_str(left_hardcoded_number);
            } else {
                number_str.push_str(chars[left]);
            }

            if right_hardcoded_index > right {
                number_str.push_str(right_hardcoded_number);
            } else {
                number_str.push_str(chars[right]);
            }

            let number = number_str.parse::<u64>().unwrap();
            total += number;
        }

        total
    }
}
