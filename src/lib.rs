#[macro_use]
extern crate aoc_runner_derive;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

pub fn parse_lines<T: std::str::FromStr>(input: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    input.lines().map(|line| line.parse().unwrap()).collect()
}

aoc_lib! { year = 2021 }
