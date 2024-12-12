use memoize::memoize;

extern crate test;

const INPUT: &str = include_str!("../inputs/day11.txt");

#[memoize]
fn process(number: usize, times: usize) -> usize {
    if times == 0 {
        return 1;
    }

    if number == 0 {
        return process(1, times - 1);
    }

    let str = format!("{}", number);
    if str.len() % 2 == 0 {
        let left = str[0..str.len() / 2].parse::<usize>().unwrap();
        let right = str[str.len() / 2..].parse::<usize>().unwrap();
        process(left, times - 1) + process(right, times - 1)
    } else {
        process(number * 2024, times - 1)
    }
}

pub fn part1(input: &str) -> usize {
    input
        .split(' ')
        .map(|str| process(str.parse::<usize>().unwrap(), 25))
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .split(' ')
        .map(|str| process(str.parse::<usize>().unwrap(), 75))
        .sum()
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = include_str!("../test_inputs/day11.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 55312);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 81);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part1(input))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part2(input))
    }
}
