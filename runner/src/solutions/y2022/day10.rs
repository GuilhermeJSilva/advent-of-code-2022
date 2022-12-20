use super::super::AocSolution;
use std::{iter::once, str::FromStr, sync};

pub struct Day10;

impl Day10 {
    pub fn new() -> sync::Arc<Day10> {
        sync::Arc::new(Day10 {})
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(i64),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..4] {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Add(s[5..].parse().unwrap())),
            _ => Err("Instruction not recognised"),
        }
    }
}

#[derive(Debug)]
struct Computer {
    current_cycle: i64,
    register_x: i64,
}

fn get_computer_states(input: &'_ str) -> impl Iterator<Item = Computer> + '_ {
    input
        .lines()
        .flat_map(str::parse::<Instruction>)
        .scan(
            Computer {
                current_cycle: 1,
                register_x: 1,
            },
            |state, command| match command {
                Instruction::Noop => {
                    let clock_states = Some(vec![Computer {
                        current_cycle: state.current_cycle,
                        register_x: state.register_x,
                    }]);
                    state.current_cycle += 1;
                    clock_states
                }
                Instruction::Add(value) => {
                    let clock_states = Some(vec![
                        Computer {
                            current_cycle: state.current_cycle,
                            register_x: state.register_x,
                        },
                        Computer {
                            current_cycle: state.current_cycle + 1,
                            register_x: state.register_x,
                        },
                    ]);
                    state.current_cycle += 2;
                    state.register_x += value;
                    clock_states
                }
            },
        )
        .flat_map(|states| states.into_iter())
}

impl AocSolution for Day10 {
    fn solve_part1(&self, input: String) -> String {
        let sum: i64 = get_computer_states(&input)
            .filter(|state| (state.current_cycle + 20) % 40 == 0)
            .map(|state| state.register_x * state.current_cycle as i64)
            .sum();

        format!("{:?}", sum)
    }

    fn solve_part2(&self, input: String) -> String {
        get_computer_states(&input)
            // .inspect(|state| println!("{:?}", state))
            // .inspect(|state| println!("{:?}", ((state.current_cycle - 1) % 40) + 1))
            .map(|state| (((state.current_cycle - 1) % 40) - state.register_x).abs() <= 1)
            .map(|is_on| if is_on { '#' } else { '.' })
            .enumerate()
            .flat_map(|(i, c)| {
                if i % 40 == 0 && i != 0 {
                    Some('\n')
                } else {
                    None
                }
                .into_iter()
                .chain(once(c))
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    const INPUT: &str = include_str!("day_10.test.txt");
    use crate::solutions::{y2022::day10::Day10, AocSolution};

    #[test]
    fn part1() {
        let solution = Day10::new();
        assert_eq!(solution.solve_part1(INPUT.to_owned()), "13140")
    }

    #[test]
    fn part2() {
        let solution = Day10::new();
        let expected = "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....";
        assert_eq!(solution.solve_part2(INPUT.to_owned()), expected)
    }
}
