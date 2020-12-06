#[macro_use]
extern crate aoc2020;
use aoc2020::Solution;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    for day in std::env::args()
        .skip(1)
        .filter_map(|s| s.parse::<u8>().ok())
    {
        let (res1, res2) = match day {
            1 => day01::Solver::solve(),
            2 => day02::Solver::solve(),
            3 => day03::Solver::solve(),
            4 => day04::Solver::solve(),
            5 => day05::Solver::solve(),
            6 => day06::Solver::solve(),
            x => unimplemented!("Day {} is not yet implemented", x),
        };

        println!("Day {:02} Part 1:\t{:?}", day, res1);
        println!("Day {:02} Part 2:\t{:?}", day, res2);
    }
}
