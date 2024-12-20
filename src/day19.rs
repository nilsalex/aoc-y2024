extern crate test;

use std::collections::HashMap;
use std::sync::Mutex;

use memoize::lazy_static::lazy_static;

const INPUT: &[u8] = include_bytes!("../inputs/day19.txt");

fn is_subslice<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    if haystack.len() < needle.len() {
        return false;
    }
    for i in 0..needle.len() {
        if haystack[i] != needle[i] {
            return false;
        }
    }
    true
}

lazy_static! {
    static ref MEMO: Mutex<HashMap<Vec<u8>, bool>> = Mutex::new(HashMap::new());
    static ref MEMO_2: Mutex<HashMap<Vec<u8>, usize>> = Mutex::new(HashMap::new());
}

fn can_build(towel: &[u8], towels: &[&[u8]]) -> bool {
    if towel.is_empty() {
        return true;
    }

    {
        let memo = MEMO.lock().unwrap();
        if let Some(memoized) = memo.get(towel) {
            return *memoized;
        }
    }

    let result = towels
        .iter()
        .filter(|&t| is_subslice(towel, t))
        .any(|&t| can_build(&towel[t.len()..], towels));
    let mut memo = MEMO.lock().unwrap();
    memo.insert(towel.to_vec(), result);
    result
}

pub fn part1(input: &[u8]) -> usize {
    let mut lines = input.split(|&b| b == b'\n');

    let towels = lines
        .next()
        .unwrap()
        .split(|&b| b == b',')
        .map(|bs| if bs[0] == b' ' { &bs[1..] } else { bs })
        .collect::<Vec<&[u8]>>();

    lines.next();

    let mut result = 0;
    for towel in lines {
        if can_build(towel, &towels) {
            result += 1;
        }
    }

    result
}

fn can_build_how_many(towel: &[u8], towels: &[&[u8]]) -> usize {
    if towel.is_empty() {
        return 1;
    }

    {
        let memo = MEMO_2.lock().unwrap();
        if let Some(memoized) = memo.get(towel) {
            return *memoized;
        }
    }

    let result = towels
        .iter()
        .filter(|&t| is_subslice(towel, t))
        .map(|&t| can_build_how_many(&towel[t.len()..], towels))
        .sum();
    let mut memo = MEMO_2.lock().unwrap();
    memo.insert(towel.to_vec(), result);
    result
}

pub fn part2(input: &[u8]) -> usize {
    let mut lines = input.split(|&b| b == b'\n');

    let towels = lines
        .next()
        .unwrap()
        .split(|&b| b == b',')
        .map(|bs| if bs[0] == b' ' { &bs[1..] } else { bs })
        .collect::<Vec<&[u8]>>();

    lines.next();

    let mut result = 0;
    for towel in lines {
        result += can_build_how_many(towel, &towels)
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day19.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 16);
    }

    #[test]
    fn test_is_subslice() {
        assert!(is_subslice(&[1, 2, 3], &[1, 2]));
        assert!(!is_subslice(&[2, 1, 3], &[1, 2]));
        assert!(!is_subslice(&[1, 2], &[1, 2, 3]));
        assert!(is_subslice(&[1, 2], &[1, 2]));
        assert!(is_subslice::<u8>(&[], &[]));
        assert!(is_subslice(&[1, 2], &[]));
        assert!(!is_subslice(&[], &[1, 2]));
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
