use itertools::Itertools;

use super::super::AocSolution;
use std::{str::FromStr, sync};

pub struct Day05;

impl Day05 {
    pub fn new() -> sync::Arc<Day05> {
        sync::Arc::new(Day05 {})
    }
}

#[derive(Clone, Copy)]
struct Block(char);
struct Towers {
    towers: Vec<Vec<Block>>,
}

impl FromStr for Towers {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let last_line = s.lines().rev().next().expect("Input has n lines");
        let size = last_line
            .split(' ')
            .rev()
            .inspect(|v| println!("{:?}", v)).find(|v| !v.is_empty())
            .map(str::parse::<usize>)
            .expect("Last line is empty")
            .expect("Last element is a nunber");
        let mut towers: Vec<Vec<Block>> = Vec::with_capacity(size);
        for _ in 0..size {
            towers.push(Vec::new());
        }

        s.lines().rev().skip(1).for_each(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .for_each(|(i, c)| towers.get_mut(i).unwrap().push(Block(c)))
        });

        Ok(Towers { towers })
    }
}

impl Towers {
    pub fn apply(&mut self, move_op: MoveOpertaion) {
        let extension: Vec<Block> = self
            .towers
            .get(move_op.origin)
            .unwrap()
            .iter()
            .rev()
            .take(move_op.number)
            .copied()
            .collect();
        self.towers
            .get_mut(move_op.target)
            .unwrap()
            .extend(extension);
        let origin = self.towers.get_mut(move_op.origin).unwrap();
        origin.truncate(origin.len() - move_op.number.min(origin.len()));
    }

    pub fn apply_multiple(&mut self, move_op: MoveOpertaion) {
        let extension: Vec<Block> = self
            .towers
            .get(move_op.origin)
            .unwrap()
            .iter()
            .rev()
            .take(move_op.number)
            .rev()
            .copied()
            .collect();
        self.towers
            .get_mut(move_op.target)
            .unwrap()
            .extend(extension);
        let origin = self.towers.get_mut(move_op.origin).unwrap();
        origin.truncate(origin.len() - move_op.number.min(origin.len()));
    }

    pub fn tops(&self) -> String {
        self.towers.iter().map(|t| t.last().unwrap().0).collect()
    }
}

struct MoveOpertaion {
    number: usize,
    origin: usize,
    target: usize,
}

impl FromStr for MoveOpertaion {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((number, origin, target)) = s
            .split(' ')
            .map(str::parse::<usize>)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect_tuple()
        {
            return Ok(MoveOpertaion {
                number,
                origin: origin - 1,
                target: target - 1,
            });
        }
        Err("Line does not containt 3 values")
    }
}

impl AocSolution for Day05 {
    fn solve_part1(&self, input: String) -> String {
        let (tower, moves) = input.split_once("\n\n").unwrap();
        let mut tower = str::parse::<Towers>(tower).unwrap();
        moves
            .lines()
            .map(str::parse::<MoveOpertaion>)
            .map(Result::unwrap)
            .for_each(|move_op| tower.apply(move_op));
        tower.tops()
    }

    fn solve_part2(&self, input: String) -> String {
        let (tower, moves) = input.split_once("\n\n").unwrap();
        let mut tower = str::parse::<Towers>(tower).unwrap();
        moves
            .lines()
            .map(str::parse::<MoveOpertaion>)
            .map(Result::unwrap)
            .for_each(|move_op| tower.apply_multiple(move_op));
        tower.tops()
    }
}
