#![feature(int_abs_diff)]

#[macro_use]
extern crate aoc_runner_derive;

pub mod day01;
pub mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

pub fn parse_lines<T: std::str::FromStr>(input: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    input.lines().map(|line| line.parse().unwrap()).collect()
}

aoc_lib! { year = 2021 }
