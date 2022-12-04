use super::super::AocSolution;
use std::sync;

pub struct Day02;

impl Day02 {
    pub fn new() -> sync::Arc<Day02> {
        sync::Arc::new(Day02 {})
    }
}

impl Day02 {
    pub fn from_opponent(play: char) -> u32 {
        match play {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => panic!("Wrong char"),
        }
    }

    pub fn from_player(play: char) -> u32 {
        match play {
            'X' => 0,
            'Y' => 1,
            'Z' => 2,
            _ => panic!("Wrong char"),
        }
    }

    pub fn to_result(result: char, opponent: u32) -> u32 {
        match result {
            'X' => (opponent + 2) % 3,
            'Y' => opponent,
            'Z' => (opponent + 1) % 3,
            _ => panic!("Wrong char"),
        }
    }

    pub fn shape_score(shape: u32) -> u32 {
        shape + 1
    }

    pub fn score(player: u32, opponent: u32) -> u32 {
        if player == opponent {
            return 3;
        } else if (player + 2) % 3 == opponent {
            return 6;
        }
        0
    }
}

impl AocSolution for Day02 {
    fn solve_part1(&self, input: String) -> String {
        let score: u32 = input
            .lines()
            .map(|game| {
                let opponent = Day02::from_opponent(game.chars().next().unwrap());
                let player = Day02::from_player(game.chars().nth(2).unwrap());
                Day02::shape_score(player) + Day02::score(player, opponent)
            })
            .sum();
        format!("{}", score)
    }

    fn solve_part2(&self, input: String) -> String {
        let score: u32 = input
            .lines()
            .map(|game| {
                let opponent = Day02::from_opponent(game.chars().next().unwrap());
                let player = Day02::to_result(game.chars().nth(2).unwrap(), opponent);
                Day02::shape_score(player) + Day02::score(player, opponent)
            })
            .sum();
        format!("{}", score)
    }
}
