use super::super::AocSolution;
use std::sync;

pub struct Day04;

impl Day04 {
    pub fn new() -> sync::Arc<Day04> {
        sync::Arc::new(Day04 {})
    }
}

#[derive(Debug)]
struct Interval {
    start: u32,
    end: u32,
}

impl From<&str> for Interval {
    fn from(input: &str) -> Self {
        let values: Vec<u32> = input.splitn(2, "-").flat_map(str::parse::<u32>).collect();
        Interval {
            start: values[0],
            end: values[1],
        }
    }
}

impl Interval {
    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlap(&self, other: &Self) -> Option<Interval> {
        let interval_overlap = Interval {
            start: self.start.max(other.start),
            end: self.end.min(other.end),
        };
        if interval_overlap.start <= interval_overlap.end && self.contains(&interval_overlap) && other.contains(&interval_overlap) {
            return Some(interval_overlap);
        }
        None
    }
}

impl AocSolution for Day04 {
    fn solve_part1(&self, input: String) -> String {
        let res = input
            .lines()
            .map(|pair| {
                pair.splitn(2, ",")
                    .map(Interval::from)
                    .collect::<Vec<Interval>>()
            })
            .filter(|intervals| {
                intervals[0].contains(&intervals[1]) || intervals[1].contains(&intervals[0])
            })
            .count();

        format!("{:?}", res)
    }

    fn solve_part2(&self, input: String) -> String {
        let res = input
            .lines()
            .map(|pair| {
                pair.splitn(2, ",")
                    .map(Interval::from)
                    .collect::<Vec<Interval>>()
            })
            .flat_map(|intervals| intervals[0].overlap(&intervals[1]))
            .count();

        format!("{:?}", res)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::y2022::day04::Day04;
    use crate::solutions::AocSolution;
    const INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
    #[test]
    fn part2() {
        let solver = Day04 {};
        assert_eq!(solver.solve_part2(INPUT.into()), "4");
    }
}

