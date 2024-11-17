use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day {
    parts: Vec<Vec<String>>,
    max_y: i32,
    max_x: i32,
}

impl Day {
    pub fn new(input_file: &str) -> Self {
        let file = File::open(input_file).expect("should be able to open the file");
        let reader = BufReader::new(file);

        let mut parts = Vec::new();
        for line_res in reader.lines() {
            let line = line_res.expect("should be able to read the string");
            let mut row = Vec::new();
            for item in line.split("").filter(|&x| !x.is_empty()).collect::<Vec<&str>>() {
                row.push(item.to_string());
            }
            parts.push(row);
        }

        let max_y = parts.len() as i32;
        let max_x = parts[0].len() as i32;

        return Self {
            parts,
            max_y,
            max_x,
        };
    }

    fn check_symbol(&self, y: i32, x: i32) -> bool {
        if y < 0 || y + 1 > self.max_y || x < 0 || x + 1 > self.max_x {
            return false;
        }

        let y_index = y as usize;
        let x_index = x as usize;

        let item = self.parts[y_index][x_index].to_string();

        return item != "0" &&
            item != "1" &&
            item != "2" &&
            item != "3" &&
            item != "4" &&
            item != "5" &&
            item != "6" &&
            item != "7" &&
            item != "8" &&
            item != "9" &&
            item != ".";
    }

    fn check_number(&self, y: i32, x: i32) -> bool {
        if y < 0 || y + 1 > self.max_y || x < 0 || x + 1 > self.max_x {
            return false;
        }

        let y_index = y as usize;
        let x_index = x as usize;

        let item = self.parts[y_index][x_index].to_string();

        return item == "0" ||
            item == "1" ||
            item == "2" ||
            item == "3" ||
            item == "4" ||
            item == "5" ||
            item == "6" ||
            item == "7" ||
            item == "8" ||
            item == "9";
    }

    pub fn solve(&self) -> u64 {
        let mut total: u64 = Default::default();

        for (y_index, row) in self.parts.iter().enumerate() {
            let mut number: String = Default::default();
            let mut is_valid = false;
            for (x_index, item) in row.iter().enumerate() {
                let y = y_index as i32;
                let x = x_index as i32;

                if self.check_number(y, x) {
                    number += item;
                } else {
                    if !number.is_empty() {
                        if is_valid {
                            let number_int: u64 = number.parse().expect("string should contain only number");
                            total += number_int;

                            is_valid = Default::default();
                        }

                        number = Default::default();
                    }

                    continue;
                }

                if !is_valid {
                    is_valid = self.check_symbol(y - 1, x) ||
                        self.check_symbol(y - 1, x - 1) ||
                        self.check_symbol(y - 1, x + 1) ||
                        self.check_symbol(y, x - 1) ||
                        self.check_symbol(y, x + 1) ||
                        self.check_symbol(y + 1, x) ||
                        self.check_symbol(y + 1, x - 1) ||
                        self.check_symbol(y + 1, x + 1);
                }
            }

            if !number.is_empty() && is_valid {
                let number_int: u64 = number.parse().expect("string should contain only number");
                total += number_int;
            }
        }

        total
    }

    fn check_gear(&self, item: &String) -> bool {
        return item == "*";
    }

    fn get_north_gear_ratio(&self, y: i32, x: i32) -> (u64, u32) {
        let mut number: u64 = 1;
        let mut count: u32 = 0;
        if self.check_number(y - 1, x) {
            number = self.get_gear_ratio(y - 1, x);
            count += 1;
        } else {
            if self.check_number(y - 1, x - 1) {
                number *= self.get_gear_ratio(y - 1, x - 1);
                count += 1;
            }
            if self.check_number(y - 1, x + 1) {
                number *= self.get_gear_ratio(y - 1, x + 1);
                count += 1;
            }
        }

        return (number, count);
    }

    fn get_south_gear_ratio(&self, y: i32, x: i32) -> (u64, u32) {
        let mut number: u64 = 1;
        let mut count: u32 = 0;
        if self.check_number(y + 1, x) {
            number = self.get_gear_ratio(y + 1, x);
            count += 1;
        } else {
            if self.check_number(y + 1, x - 1) {
                number *= self.get_gear_ratio(y + 1, x - 1);
                count += 1;
            }
            if self.check_number(y + 1, x + 1) {
                number *= self.get_gear_ratio(y + 1, x + 1);
                count += 1;
            }
        }

        return (number, count);
    }

    fn get_west_gear_ratio(&self, y: i32, x: i32) -> (u64, u32) {
        let mut number: u64 = 1;
        let mut count: u32 = 0;
        if self.check_number(y, x - 1) {
            number = self.get_gear_ratio(y, x - 1);
            count += 1;
        }

        return (number, count);
    }

    fn get_east_gear_ratio(&self, y: i32, x: i32) -> (u64, u32) {
        let mut number: u64 = 1;
        let mut count: u32 = 0;
        if self.check_number(y, x + 1) {
            number = self.get_gear_ratio(y, x + 1);
            count += 1;
        }

        return (number, count);
    }

    fn get_gear_ratio(&self, y: i32, x: i32) -> u64 {
        let mut number: String = Default::default();
        let mut x_left = x;
        while self.check_number(y, x_left) {
            let item = &self.parts[y as usize][x_left as usize];
            number = item.to_string() + &*number;
            x_left -= 1;
        }
        x_left = x + 1;
        while self.check_number(y, x_left) {
            let item = &self.parts[y as usize][x_left as usize];
            number = number + &*item.to_string();
            x_left += 1;
        }

        return number.parse().expect("string should contain only number");
    }

    pub fn solve2(&self) -> u64 {
        let mut total: u64 = Default::default();

        for (y_index, row) in self.parts.iter().enumerate() {
            for (x_index, item) in row.iter().enumerate() {
                let mut number: u64 = 1;
                let y = y_index as i32;
                let x = x_index as i32;
                let mut count: u32 = Default::default();
                if self.check_gear(item) {
                    let (res, count_res) = self.get_north_gear_ratio(y, x);
                    if count_res > 0 {
                        count += count_res;
                        number *= res;
                    }

                    let (res, count_res) = self.get_south_gear_ratio(y, x);
                    if count_res > 0 {
                        count += count_res;
                        number *= res;
                    }

                    let (res, count_res) = self.get_west_gear_ratio(y, x);
                    if count_res > 0 {
                        count += count_res;
                        number *= res;
                    }
                    let (res, count_res) = self.get_east_gear_ratio(y, x);
                    if count_res > 0 {
                        count += count_res;
                        number *= res;
                    }

                    if count == 2 {
                        total += number;
                    }
                }
            }
        }

        total
    }
}
