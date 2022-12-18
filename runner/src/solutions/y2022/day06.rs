use super::super::AocSolution;
use std::sync;

pub struct Day06;

impl Day06 {
    pub fn new() -> sync::Arc<Day06> {
        sync::Arc::new(Day06 {})
    }
}

impl AocSolution for Day06 {
    fn solve_part1(&self, input: String) -> String {
        self.solve(input, 4)
    }

    fn solve_part2(&self, input: String) -> String {
        self.solve(input, 14)
    }

}

impl Day06 {
    fn solve(&self, input: String, window_size: usize) -> String {
        let (size, _) = input
            .as_bytes()
            .windows(window_size)
            .enumerate()
            .find(|(_, char_window)| unique_bytes(char_window))
            .unwrap();
        format!("{:?}", size + window_size)
    }
    
}

fn unique_bytes(char_window: &[u8]) -> bool {
    char_window
        .iter()
        .enumerate()
        .all(|(i, c)| !char_window[i + 1..].contains(c))
}

#[cfg(test)]
mod test {
    const INPUT1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    use crate::solutions::{y2022::day06::Day06, AocSolution};
    #[test]
    fn part1() {
        let solution = Day06::new();
        assert_eq!(solution.solve_part1(INPUT1.to_owned()), "5");
    }
    #[test]
    fn part2() {
        let solution = Day06::new();
        assert_eq!(solution.solve_part2(INPUT1.to_owned()), "23");
    }
}
