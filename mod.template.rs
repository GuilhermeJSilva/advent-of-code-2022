use super::{AocSolution, SolutionProvider};
use std::sync;
{{mod_days}}

pub struct Solutions{{year}};

impl Solutions{{year}} {
    pub fn new() -> Box<Self> {
        Box::new(Solutions{{year}} {})
    }
}

impl SolutionProvider for Solutions{{year}} {
    fn register(&self, solutions: &mut super::Solutions) {
        let sols: Vec<sync::Arc<dyn AocSolution>> = vec![{{solutions}}];

        sols.into_iter()
            .enumerate()
            .for_each(|(index, sol)| solutions.register_solution({{year}}, index as u8 + 1, sol));
    }
}
