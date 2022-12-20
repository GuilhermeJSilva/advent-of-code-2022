use super::super::AocSolution;
use std::{collections::HashSet, str::FromStr, sync};

pub struct Day09;

impl Day09 {
    pub fn new() -> sync::Arc<Day09> {
        sync::Arc::new(Day09 {})
    }
}

struct Movement(Direction, usize);
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Movement {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, steps) = s.split_once(' ').unwrap();
        let steps = steps.parse::<usize>().unwrap();
        match direction {
            "U" => Ok(Movement(Direction::Up, steps)),
            "D" => Ok(Movement(Direction::Down, steps)),
            "L" => Ok(Movement(Direction::Left, steps)),
            "R" => Ok(Movement(Direction::Right, steps)),
            _ => Err("Direction unknown"),
        }
    }
}

struct Snake {
    parts: Vec<(i64, i64)>,
}

impl Snake {
    pub fn new(size: usize) -> Self {
        assert!(size > 1);
        Snake {
            parts: vec![(0, 0); size],
        }
    }

    pub fn movement(&mut self, mov: Movement) -> HashSet<(i64, i64)> {
        let steps = mov.1;
        let mut tails = HashSet::with_capacity(steps);
        let (x_step, y_step) = match mov.0 {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        for _ in 0..steps {
            let prev_pos = *self.parts.first().unwrap();
            *self.parts.get_mut(0).unwrap() = (prev_pos.0 + x_step, prev_pos.1 + y_step);
            for i in 1..self.parts.len() {
                if Snake::is_distant(self.parts.get(i).unwrap(), self.parts.get(i - 1).unwrap()) {
                    let (x_body_step, y_body_step) =
                        Snake::get_step(self.parts.get(i).unwrap(), self.parts.get(i - 1).unwrap());
                    self.parts.get_mut(i).unwrap().0 += x_body_step;
                    self.parts.get_mut(i).unwrap().1 += y_body_step;
                } else {
                    break;
                }
            }
            tails.insert(*self.parts.last().unwrap());
        }
        tails
    }

    fn is_distant(part: &(i64, i64), prev_part: &(i64, i64)) -> bool {
        (part.0 - prev_part.0).abs() > 1 || (part.1 - prev_part.1).abs() > 1
    }

    fn get_step(part: &(i64, i64), prev_part: &(i64, i64)) -> (i64, i64) {
        if part.0 == prev_part.0 || part.1 == prev_part.1 {
            (
                (-part.0 + prev_part.0).signum() * ((part.0 - prev_part.0).abs() - 1),
                (-part.1 + prev_part.1).signum() * ((part.1 - prev_part.1).abs() - 1),
            )
        } else {
            (
                (-part.0 + prev_part.0).signum(),
                (-part.1 + prev_part.1).signum(),
            )
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        let board_size = 30usize;
        let mut board = vec![vec!['.'; board_size]; board_size];
        for i in 0..self.parts.len() {
            let (pos_x, pos_y) = self.parts.get(i).unwrap();
            let pos_x = (pos_x + board_size as i64 / 2) as usize;
            let pos_y = (pos_y + board_size as i64 / 2) as usize;
            *board.get_mut(pos_y).unwrap().get_mut(pos_x).unwrap() = ('0' as u8 + i as u8) as char;
        }
        for line in board {
            println!("{:?}", line);
        }
        println!();
    }
}

impl AocSolution for Day09 {
    fn solve_part1(&self, input: String) -> String {
        let mut snake = Snake::new(2);
        let movements = input.lines().flat_map(str::parse::<Movement>);
        let positions = movements
            .flat_map(move |movement| snake.movement(movement).into_iter())
            .collect::<HashSet<(i64, i64)>>();

        format!("{:?}", positions.len())
    }

    fn solve_part2(&self, input: String) -> String {
        let mut snake = Snake::new(10);
        let movements = input.lines().flat_map(str::parse::<Movement>);
        let positions = movements
            .flat_map(move |movement| snake.movement(movement).into_iter())
            .collect::<HashSet<(i64, i64)>>();

        format!("{:?}", positions.len())
    }
}

#[cfg(test)]
mod test {
    const INPUT: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    const INPUT_2: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
    use crate::solutions::{y2022::day09::Day09, AocSolution};

    #[test]
    fn part1() {
        let solution = Day09::new();
        assert_eq!(solution.solve_part1(INPUT.to_owned()), "13")
    }

    #[test]
    fn part2() {
        let solution = Day09::new();
        assert_eq!(solution.solve_part2(INPUT.to_owned()), "1");
        assert_eq!(solution.solve_part2(INPUT_2.to_owned()), "36")
    }
}
