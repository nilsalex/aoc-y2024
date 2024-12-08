const INPUT: &str = include_str!("../inputs/day07.txt");

fn parse_line(line: &str) -> (usize, Vec<usize>) {
    let mut iter = line.split(": ");
    let left_value = iter.next().unwrap().parse::<usize>().unwrap();
    let right_values = iter
        .next()
        .unwrap()
        .split(' ')
        .map(|group| group.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    (left_value, right_values)
}

#[derive(Copy, Clone)]
enum Operation {
    Add,
    Mul,
    Cat,
}

impl Operation {
    fn unapply(&self, left: usize, right: usize) -> Option<usize> {
        match self {
            Self::Add => {
                if right >= left {
                    Some(right - left)
                } else {
                    None
                }
            }
            Self::Mul => {
                if right % left == 0 {
                    Some(right / left)
                } else {
                    None
                }
            }
            Self::Cat => {
                let left_str = left.to_string();
                let left_str_bytes = left_str.as_bytes();
                let right_str = right.to_string();
                let right_str_bytes = right_str.as_bytes();
                if right_str_bytes.len() > left_str_bytes.len()
                    && right_str_bytes.ends_with(left_str_bytes)
                {
                    let new_str = std::str::from_utf8(
                        &right_str_bytes[0..right_str_bytes.len() - left_str_bytes.len()],
                    )
                    .unwrap();
                    Some(new_str.parse::<usize>().unwrap())
                } else {
                    None
                }
            }
        }
    }
}

struct State<'a> {
    target: usize,
    values: &'a [usize],
}

impl State<'_> {
    fn is_final(&self) -> bool {
        self.values.len() == 1 && self.values[0] == self.target
    }

    fn next(&self, op: Operation) -> Option<Self> {
        self.values.split_last().and_then(|(&val, rest)| {
            op.unapply(val, self.target).map(|new_target| Self {
                target: new_target,
                values: rest,
            })
        })
    }
}

fn dfs(state: &State, operations: &[Operation]) -> bool {
    if state.is_final() {
        return true;
    }

    operations
        .iter()
        .filter_map(|&op| state.next(op))
        .any(|s| dfs(&s, operations))
}

pub fn part1(input: &str) -> usize {
    let ops = &[Operation::Add, Operation::Mul];
    input
        .lines()
        .filter_map(|line| {
            let (target, values) = parse_line(line);
            if dfs(
                &State {
                    target,
                    values: &values,
                },
                ops,
            ) {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let ops = &[Operation::Add, Operation::Mul, Operation::Cat];
    input
        .lines()
        .filter_map(|line| {
            let (target, values) = parse_line(line);
            if dfs(
                &State {
                    target,
                    values: &values,
                },
                ops,
            ) {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(input));
    println!("{}", part2(input));
}
