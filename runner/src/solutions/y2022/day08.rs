use super::super::AocSolution;
use std::{str::FromStr, sync};

pub struct Day08;

impl Day08 {
    pub fn new() -> sync::Arc<Day08> {
        sync::Arc::new(Day08 {})
    }
}

struct Forest {
    x_lenght: usize,
    y_lenght: usize,
    trees: Vec<Vec<i64>>,
}

impl FromStr for Forest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees: Vec<Vec<i64>> = s
            .lines()
            .map(|line| line.chars().map(|c| c as i64 - '0' as i64).collect())
            .collect();
        Ok(Forest {
            x_lenght: trees.get(0).unwrap().len(),
            y_lenght: trees.len(),
            trees,
        })
    }
}

impl Forest {
    fn mark_visible<I>(&self, is_visible: &mut Vec<Vec<bool>>, y_range: I, x_range: I)
    where
        I: Iterator<Item = usize> + Clone,
    {
        let mut maximum_size = vec![-1; self.x_lenght];
        let mut first_line = true;
        for j in y_range {
            let tree_line = self.trees.get(j).unwrap();
            let mut x_maximum = -1;
            let mut first_col = true;
            for i in x_range.clone() {
                let tree = *tree_line.get(i).unwrap();
                let y_maximum = *maximum_size.get(i).unwrap();
                if x_maximum < tree || y_maximum < tree || first_col || first_line {
                    *is_visible.get_mut(j).unwrap().get_mut(i).unwrap() = true;
                }
                x_maximum = x_maximum.max(tree);
                *maximum_size.get_mut(i).unwrap() = y_maximum.max(tree);
                first_col = false;
            }
            first_line = false;
        }
    }

    fn count_visible(&self) -> usize {
        let mut is_visible: Vec<Vec<bool>> = vec![vec![false; self.x_lenght]; self.y_lenght];
        self.mark_visible(&mut is_visible, 0..self.x_lenght, 0..self.y_lenght);
        self.mark_visible(
            &mut is_visible,
            (0..self.x_lenght).rev(),
            (0..self.y_lenght).rev(),
        );

        is_visible.iter().flatten().filter(|tree| **tree).count()
    }

    fn max_scenic_score(&self) -> usize {
        (0..self.y_lenght)
            .flat_map(|i| (0..self.x_lenght).map(move |j| (i, j)))
            .map(|(i, j)| self.scenic_score(i, j))
            .max()
            .unwrap()
    }

    fn count_smaller_x<I>(&self, i: usize, j_iter: I, tree: i64) -> usize
    where
        I: Iterator<Item = usize> + Clone,
    {
        let row = self.trees.get(i).unwrap();
        j_iter
            .clone()
            .count()
            .min(j_iter.take_while(|j| *row.get(*j).unwrap() < tree).count() + 1)
    }

    fn count_smaller_y<I>(&self, i_iter: I, j: usize, tree: i64) -> usize
    where
        I: Iterator<Item = usize> + Clone,
    {
        i_iter.clone().count().min(
            i_iter
                .take_while(|i| *self.trees.get(*i).unwrap().get(j).unwrap() < tree)
                .count()
                + 1,
        )
    }

    fn scenic_score(&self, i: usize, j: usize) -> usize {
        let tree = *self.trees.get(i).unwrap().get(j).unwrap();
        self.count_smaller_x(i, j + 1..self.x_lenght, tree)
            * self.count_smaller_x(i, (0..j).rev(), tree)
            * self.count_smaller_y(i + 1..self.y_lenght, j, tree)
            * self.count_smaller_y((0..i).rev(), j, tree)
    }
}

impl AocSolution for Day08 {
    fn solve_part1(&self, input: String) -> String {
        format!(
            "{:?}",
            str::parse::<Forest>(&input).unwrap().count_visible()
        )
    }

    fn solve_part2(&self, input: String) -> String {
        format!(
            "{:?}",
            str::parse::<Forest>(&input).unwrap().max_scenic_score()
        )
    }
}

#[cfg(test)]
mod test {
    const INPUT: &str = "30373\n25512\n65332\n33549\n35390";
    use crate::solutions::{y2022::day08::Day08, AocSolution};
    #[test]
    fn part1() {
        let solution = Day08::new();
        assert_eq!(solution.solve_part1(INPUT.to_owned()), "21")
    }
    #[test]
    fn part2() {
        let solution = Day08::new();
        assert_eq!(solution.solve_part2(INPUT.to_owned()), "8")
    }
}
