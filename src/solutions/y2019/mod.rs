use super::{AocSolution, SolutionProvider};
use std::sync;
mod day01;

pub struct Solutions2019;

impl Solutions2019 {
    pub fn new() -> Box<Self> {
        Box::new(Solutions2019 {})
    }
}

impl SolutionProvider for Solutions2019 {
    fn register(&self, solutions: &mut super::Solutions) {
        let sols: Vec<sync::Arc<dyn AocSolution>> = vec![day01::Day01::new()];

        sols.into_iter()
            .enumerate()
            .for_each(|(index, sol)| solutions.register_solution(2019, index as u8 + 1, sol));
    }
}
