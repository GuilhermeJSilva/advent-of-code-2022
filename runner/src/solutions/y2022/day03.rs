use super::super::AocSolution;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    sync,
};

pub struct Day03;

impl Day03 {
    pub fn new() -> sync::Arc<Day03> {
        sync::Arc::new(Day03 {})
    }
}

struct Compartment {
    item_count: HashMap<char, usize>,
}

impl From<&str> for Compartment {
    fn from(input: &str) -> Self {
        Compartment {
            item_count: input.chars().fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            }),
        }
    }
}

impl Compartment {
    fn keys(&self) -> HashSet<char> {
        HashMap::keys(&self.item_count).copied().collect()
    }
}

pub struct Rucksack {
    components: [Compartment; 2],
}

impl From<&str> for Rucksack {
    fn from(input: &str) -> Self {
        Rucksack {
            components: [
                Compartment::from(&input[..input.len() / 2]),
                Compartment::from(&input[input.len() / 2..]),
            ],
        }
    }
}

impl Rucksack {
    fn unique_key(self) -> Option<char> {
        self.components[0]
            .keys()
            .intersection(&self.components[1].keys())
            .next()
            .copied()
    }

    fn keys(self) -> HashSet<char> {
        self.components
            .iter()
            .map(Compartment::keys)
            .fold(HashSet::new(), |mut set, comp| {
                set.extend(comp.iter());
                set
            })
    }
}

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

impl AocSolution for Day03 {
    fn solve_part1(&self, input: String) -> String {
        let res: u32 = input
            .lines()
            .map(Rucksack::from)
            .flat_map(Rucksack::unique_key)
            .map(priority)
            .sum();

        return format!("{}", res);
    }

    fn solve_part2(&self, input: String) -> String {
        let res: u32 = input
            .lines()
            .map(Rucksack::from)
            .map(Rucksack::keys)
            .batching(|it| {
                it.take(3)
                    .reduce(|a, b| a.intersection(&b).copied().collect())
            })
            .map(|collection| collection.into_iter().next().unwrap())
            .map(priority)
            .sum();
        return format!("{}", res);
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::y2022::day03::Day03;
    use crate::solutions::AocSolution;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
    #[test]
    fn part1() {
        let solver = Day03 {};
        assert_eq!(solver.solve_part1(INPUT.into()), "157");
    }

    #[test]
    fn part2() {
        let solver = Day03 {};
        assert_eq!(solver.solve_part2(INPUT.into()), "70");
    }
}
