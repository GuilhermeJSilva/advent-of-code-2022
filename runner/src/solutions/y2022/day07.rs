use super::super::AocSolution;
use std::{collections::HashMap, str::FromStr, sync};

pub struct Day07;

impl Day07 {
    pub fn new() -> sync::Arc<Day07> {
        sync::Arc::new(Day07 {})
    }
}

struct Folder {
    sub_dirs: HashMap<String, Folder>,
    files: HashMap<String, u64>,
}

impl Folder {
    fn new() -> Self {
        Folder {
            sub_dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn with_files(files: HashMap<String, u64>) -> Self {
        Folder {
            sub_dirs: HashMap::new(),
            files,
        }
    }

    fn add_files(&mut self, children: &[&String], files: HashMap<String, u64>) {
        if children.is_empty() {
            self.files = files;
        } else if children.len() == 1 {
            self.sub_dirs
                .insert(children[0].to_owned(), Folder::with_files(files));
        } else {
            match self.sub_dirs.contains_key(children[0]) {
                true => (),
                false => {
                    self.sub_dirs.insert(children[0].to_owned(), Folder::new());
                }
            }
            self.sub_dirs
                .get_mut(children[0])
                .unwrap()
                .add_files(&children[1..], files);
        }
    }
}

impl From<Vec<Command>> for Folder {
    fn from(commands: Vec<Command>) -> Self {
        let mut working_dir = Vec::new();
        let mut root_dir = Folder::new();
        for command in &commands {
            match command {
                Command::Cd(cd_type) => {
                    match cd_type {
                        CdType::Root => working_dir.truncate(0),
                        CdType::Parent => {
                            working_dir.pop();
                        }
                        CdType::Child(child_dir) => working_dir.push(child_dir),
                    };
                }
                Command::Ls(files) => {
                    root_dir.add_files(working_dir.as_slice(), files.to_owned());
                }
            };
        }
        root_dir
    }
}

enum Command {
    Cd(CdType),
    Ls(HashMap<String, u64>),
}

enum CdType {
    Root,
    Parent,
    Child(String),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_line = s.lines().take(1).next().unwrap();
        let command_name = first_line.split_whitespace().next().unwrap();
        match command_name {
            "ls" => Ok(Command::Ls(to_files(s.split_once("\n").unwrap().1))),
            "cd" => Ok(Command::Cd(first_line.parse().unwrap())),
            _ => Err("Command not supported"),
        }
    }
}

impl FromStr for CdType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ').unwrap().1 {
            "/" => Ok(CdType::Root),
            ".." => Ok(CdType::Parent),
            child => Ok(CdType::Child(child.to_owned())),
        }
    }
}

fn to_files(s: &str) -> HashMap<String, u64> {
    let mut files = HashMap::new();
    s.lines().for_each(|line| {
        if let Some((first_arg, second_arg)) = line.split_once(' ') {
            match first_arg {
                "dir" => None,
                _ => files.insert(second_arg.to_owned(), first_arg.parse().unwrap()),
            };
        }
    });
    files
}

impl Folder {
    fn visit<T, G>(&self, reduce_chilren: &G) -> T
    where
        G: Fn(u64, Vec<T>) -> T,
    {
        let iter = self
            .sub_dirs
            .values()
            .map(&|folder: &Folder| folder.visit(reduce_chilren))
            .collect();
        let files = self.files.values().sum();
        reduce_chilren(files, iter)
    }
}

struct SumMax {
    current_sum: u64,
    conditional_sum: u64,
}

impl SumMax {
    fn reduce_chilren(file_size: u64, children: Vec<SumMax>) -> SumMax {
        let mut current_sum = file_size;
        let mut conditional_sum = 0;
        for child in children {
            current_sum += child.current_sum;
            conditional_sum += child.conditional_sum;
        }
        if current_sum <= 100000 {
            conditional_sum += current_sum;
        }
        SumMax {
            current_sum,
            conditional_sum,
        }
    }
}

struct MinimumDelete {
    current_sum: u64,
    min_deletion: u64,
}

impl AocSolution for Day07 {
    fn solve_part1(&self, input: String) -> String {
        let commands: Vec<Command> = input
            .split("$ ")
            .filter(|line| !line.is_empty())
            .flat_map(str::parse::<Command>)
            .collect();
        let directory = Folder::from(commands);
        format!(
            "{:?}",
            directory.visit(&SumMax::reduce_chilren).conditional_sum
        )
    }

    fn solve_part2(&self, input: String) -> String {
        let commands: Vec<Command> = input
            .split("$ ")
            .filter(|line| !line.is_empty())
            .flat_map(str::parse::<Command>)
            .collect();
        let directory = Folder::from(commands);
        let total_size: u64 =
            directory.visit(&|file_size, children| file_size + children.iter().sum::<u64>());
        let min_delete = 30_000_000 - (70_000_000 - total_size);
        
        let res: MinimumDelete = directory.visit(&|file_size, children: Vec<MinimumDelete>| {
            let mut current_sum = file_size;
            let mut min_deletion = u64::MAX;
            for child in children {
                current_sum += child.current_sum;
                min_deletion = min_deletion.min(child.min_deletion);
            }
            if current_sum >= min_delete {
                min_deletion = min_deletion.min(current_sum);
            }
            return MinimumDelete {
                current_sum,
                min_deletion,
            };
        });
        format!("{:?}", res.min_deletion)
    }
}

#[cfg(test)]
mod test {
    const INPUT1: &str = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
    use crate::solutions::{y2022::day07::Day07, AocSolution};
    #[test]
    fn part1() {
        let solution = Day07::new();
        assert_eq!(solution.solve_part1(INPUT1.to_owned()), "95437");
    }
    #[test]
    fn part2() {
        let solution = Day07::new();
        assert_eq!(solution.solve_part2(INPUT1.to_owned()), "24933642");
    }
}
