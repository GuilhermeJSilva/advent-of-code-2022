use super::{AocSolution, SolutionProvider};
use std::sync;
mod day01;

pub struct Solutions2022;

impl Solutions2022 {
    pub fn new() -> Box<Self> {
        Box::new(Solutions2022 {})
    }
}

impl SolutionProvider for Solutions2022 {
    fn register(&self, solutions: &mut super::Solutions) {
        let sols: Vec<sync::Arc<dyn AocSolution>> = vec![day01::Day01::new()];

        sols.into_iter()
            .enumerate()
            .for_each(|(index, sol)| solutions.register_solution(2022, index as u8 + 1, sol));
    }
}
