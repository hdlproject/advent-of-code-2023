mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day24;

use day7::solution;

fn main() {
    let day = solution::Day::new("./src/day7/input.txt");

    let solution = day.solve();

    println!("{}", solution);
    // 249956000 too low
    // 250977165 too high
    // 250254244
}
