use regex::Regex;

const INPUT: &str = include_str!("../inputs/day03.txt");

pub fn part1(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|caps| caps[1].parse::<usize>().unwrap() * caps[2].parse::<usize>().unwrap())
        .sum()
}

pub fn part2(input: &str) -> usize {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut result = 0;
    let mut enabled = true;

    for caps in re.captures_iter(input) {
        match caps[0].chars().nth(2) {
            Some('(') => {
                enabled = true;
                continue;
            }
            Some('n') => {
                enabled = false;
                continue;
            }
            _ => {
                if enabled {
                    result += caps[1].parse::<usize>().unwrap() * caps[2].parse::<usize>().unwrap()
                }
            }
        }
    }

    result
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(input));
    println!("{}", part2(input));
}
