extern crate test;

use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("../inputs/day25.txt");

fn parse_lock(bytes: &[u8]) -> [u8; 5] {
    let mut heights = [0; 5];

    for line in bytes.split(|&b| b == b'\n').skip(1).take(6) {
        for (i, b) in line.iter().enumerate() {
            if *b == b'#' {
                heights[i] += 1;
            }
        }
    }

    heights
}

fn parse_key(bytes: &[u8]) -> [u8; 5] {
    let mut heights = [5; 5];

    for line in bytes.split(|&b| b == b'\n').skip(1).take(6) {
        for (i, b) in line.iter().enumerate() {
            if *b == b'.' {
                heights[i] -= 1;
            }
        }
    }

    heights
}

pub fn part1(input: &[u8]) -> usize {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for chunk in input.chunks(6 * 7 + 1) {
        match chunk[0] {
            b'.' => {
                keys.push(parse_key(chunk));
            }
            b'#' => {
                locks.push(parse_lock(chunk));
            }
            _ => panic!(),
        }
    }

    keys.sort();
    keys.dedup();

    locks.sort();
    locks.dedup();

    keys.iter()
        .cartesian_product(locks.iter())
        .filter(|(key, lock)| key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 5))
        .count()
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day25.txt");

    #[test]
    fn test_parse_lock() {
        assert_eq!(
            parse_lock(b"#####\n.####\n.####\n.####\n.#.#.\n.#...\n.....\n\n"),
            [0, 5, 3, 4, 3]
        );
    }

    #[test]
    fn test_parse_key() {
        assert_eq!(
            parse_key(b".....\n#....\n#....\n#...#\n#.#.#\n#.###\n####"),
            [5, 0, 2, 1, 3]
        );
    }

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 3);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part1(input))
    }
}
