mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day24;

use day9::solution;

fn main() {
    let day = solution::Day::new("./src/day9/input.txt");

    let solution = day.solve2();

    println!("{}", solution);
}
