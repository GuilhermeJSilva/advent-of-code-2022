use super::{AocSolution, SolutionProvider};
use std::sync;
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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub struct Solutions2022;

impl Solutions2022 {
    pub fn new() -> Box<Self> {
        Box::new(Solutions2022 {})
    }
}

impl SolutionProvider for Solutions2022 {
    fn register(&self, solutions: &mut super::Solutions) {
        let sols: Vec<sync::Arc<dyn AocSolution>> = vec![
            day01::Day01::new(),
            day02::Day02::new(),
            day03::Day03::new(),
            day04::Day04::new(),
            day05::Day05::new(),
            day06::Day06::new(),
            day07::Day07::new(),
            day08::Day08::new(),
            day09::Day09::new(),
            day10::Day10::new(),
            day11::Day11::new(),
            day12::Day12::new(),
            day13::Day13::new(),
            day14::Day14::new(),
            day15::Day15::new(),
            day16::Day16::new(),
            day17::Day17::new(),
            day18::Day18::new(),
            day19::Day19::new(),
            day20::Day20::new(),
            day21::Day21::new(),
            day22::Day22::new(),
            day23::Day23::new(),
            day24::Day24::new(),
            day25::Day25::new(),
        ];

        sols.into_iter()
            .enumerate()
            .for_each(|(index, sol)| solutions.register_solution(2022, index as u8 + 1, sol));
    }
}
