extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day01.txt");

const POWERS_OF_TEN: [i32; 6] = [1, 10, 100, 1000, 10000, 100000];

fn i32_from_bytes(bytes: &[u8]) -> i32 {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as i32 * POWERS_OF_TEN[ix]
    })
}

pub fn part1(input: &[u8]) -> i32 {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    input.split(|c| *c == b'\n').for_each(|line| {
        let mut iter = line.split(|c| *c == b' ');
        let a = i32_from_bytes(iter.next().unwrap());
        let b = i32_from_bytes(iter.nth(2).unwrap());

        vec1.push(a);
        vec2.push(b);
    });

    vec1.sort();
    vec2.sort();

    let mut result = 0;

    for i in 0..vec1.len() {
        result += (vec1[i] - vec2[i]).abs();
    }

    result
}

pub fn part2(input: &[u8]) -> i32 {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    input.split(|c| *c == b'\n').for_each(|line| {
        let mut iter = line.split(|c| *c == b' ');
        let a = i32_from_bytes(iter.next().unwrap());
        let b = i32_from_bytes(iter.nth(2).unwrap());

        vec1.push(a);
        vec2.push(b);
    });

    vec1.sort();
    vec2.sort();

    let mut result = 0;

    for i in vec1 {
        let n = vec2.iter().filter(|&&x| x == i).count();
        result += i * (n as i32)
    }

    result
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day01.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 31);
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
