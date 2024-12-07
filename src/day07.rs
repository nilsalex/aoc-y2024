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

enum Operation {
    Add,
    Mul,
    Cat,
}

impl Operation {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
            Self::Cat => format!("{}{}", a, b).parse::<usize>().unwrap(),
        }
    }
}

fn dfs(state: (usize, &[usize]), operations: &[Operation], target: usize) -> bool {
    if state.0 > target {
        return false;
    }

    if state.1.is_empty() {
        return state.0 == target;
    }

    operations.iter().any(|op| {
        dfs(
            (op.apply(state.0, state.1[0]), &state.1[1..]),
            operations,
            target,
        )
    })
}

pub fn part1(input: &str) -> usize {
    let ops = &[Operation::Add, Operation::Mul];
    input
        .lines()
        .filter_map(|line| {
            let (target, values) = parse_line(line);
            if dfs((values[0], &values[1..]), ops, target) {
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
            if dfs((values[0], &values[1..]), ops, target) {
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
