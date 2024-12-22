extern crate test;

use rayon::prelude::*;
use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("../inputs/day22.txt");

const POWERS_OF_TEN: [usize; 10] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];

const MASK: usize = (1 << 24) - 1;

fn usize_from_bytes(bytes: &[u8]) -> usize {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as usize * POWERS_OF_TEN[ix]
    })
}

fn next(mut x: usize) -> usize {
    x = ((x << 6) ^ x) & MASK;
    x = ((x >> 5) ^ x) & MASK;
    x = ((x << 11) ^ x) & MASK;

    x
}

pub fn part1(input: &[u8]) -> usize {
    input
        .split(|&b| b == b'\n')
        .map(usize_from_bytes)
        .map(|x| (0..2000).fold(x, |acc, _| next(acc)))
        .sum()
}

pub fn part2(input: &[u8]) -> usize {
    let prices = input
        .split(|&b| b == b'\n')
        .map(usize_from_bytes)
        .map(|x0| {
            let mut v = Vec::with_capacity(2000);
            let mut x = x0;
            for _ in 0..2000 {
                v.push((x % 10) as i8);
                x = next(x);
            }
            v
        })
        .collect::<Vec<_>>();

    let diffs = prices
        .iter()
        .map(|seq| seq.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut combos = HashSet::new();

    for seq in diffs.iter() {
        for w in seq.windows(4) {
            combos.insert(w);
        }
    }

    combos
        .par_iter()
        .map(|combo| {
            (0..prices.len())
                .filter_map(|i| {
                    diffs[i]
                        .windows(combo.len())
                        .position(|w| w == *combo)
                        .map(|pos| prices[i][pos + 4] as usize)
                })
                .sum()
        })
        .max()
        .unwrap()
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test::Bencher;

    const TEST_INPUT_1: &[u8] = include_bytes!("../test_inputs/day22_part1.txt");
    const TEST_INPUT_2: &[u8] = include_bytes!("../test_inputs/day22_part2.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT_1.trim_ascii_end();
        assert_eq!(part1(input), 37327623);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT_2.trim_ascii_end();
        assert_eq!(part2(input), 23);
    }

    //#[bench]
    //fn bench_part1(b: &mut Bencher) {
    //    let input = INPUT.trim_ascii_end();
    //    b.iter(|| part1(input))
    //}
    //
    //#[bench]
    //fn bench_part2(b: &mut Bencher) {
    //    let input = INPUT.trim_ascii_end();
    //    b.iter(|| part2(input))
    //}
}
