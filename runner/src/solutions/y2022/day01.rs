use super::super::AocSolution;
use std::sync;

pub struct Day01;

impl Day01 {
    pub fn new() -> sync::Arc<Day01> {
        sync::Arc::new(Day01 {})
    }
}

fn parse_elfs(input: String) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .into_iter()
        .map(|elf| {
            elf.split("\n")
                .into_iter()
                .map(str::parse)
                .filter(Result::is_ok)
                .map(Result::unwrap)
                .collect()
        })
        .collect()
}

impl AocSolution for Day01 {
    fn solve_part1(&self, input: String) -> String {
        let elfs = parse_elfs(input);
        let max: u32 = elfs
            .into_iter()
            .map(|elf| elf.into_iter().sum())
            .max()
            .unwrap();
        return format!("{}", max);
    }

    fn solve_part2(&self, input: String) -> String {
        let elfs = parse_elfs(input);
        let mut total_elfs: Vec<u32> = elfs.into_iter().map(|elf| elf.into_iter().sum()).collect();
        total_elfs.sort_by(|a, b| b.cmp(a));
        let max: u32 = total_elfs.into_iter().take(3).sum();
        return format!("{}", max);
    }
}
