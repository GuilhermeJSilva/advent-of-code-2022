use std::collections::HashMap;
use std::sync;
mod y2019;
mod y2022;

type SolutionMap = HashMap<String, sync::Arc<dyn AocSolution>>;

pub trait AocSolution {
    fn solve_part1(&self, input: String) -> String;
    fn solve_part2(&self, input: String) -> String;
}

pub trait SolutionProvider {
    fn register(&self, solutions: &mut Solutions);
}

pub struct Solutions {
    solutions: SolutionMap,
}

impl Solutions {
    pub fn new() -> Self {
        let providers: Vec<Box<dyn SolutionProvider>> =
            vec![y2019::Solutions2019::new(), y2022::Solutions2022::new()];
        let mut solutions = Solutions {
            solutions: HashMap::new(),
        };
        providers
            .iter()
            .for_each(|provider| provider.register(&mut solutions));
        return solutions;
    }

    pub fn get_solution(&self, year: u16, day: u8) -> sync::Arc<dyn AocSolution> {
        return self
            .solutions
            .get(&Self::get_sol_key(year, day))
            .expect(&format!("Solution for day {} of {} not found!", year, day))
            .clone();
    }

    fn register_solution(&mut self, year: u16, day: u8, solution: sync::Arc<dyn AocSolution>) {
        self.solutions
            .insert(Self::get_sol_key(year, day), solution);
    }

    fn get_sol_key(year: u16, day: u8) -> String {
        return format!("{}d{}", year, day);
    }
}
