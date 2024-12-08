extern crate test;

use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("../inputs/day08.txt");

pub fn part1(input: &[u8]) -> usize {
    let mut all_positions: Vec<Vec<(isize, isize)>> = vec![vec![]; 256];
    let cols = input.iter().take_while(|&&b| b != b'\n').count() as isize;
    let rows = input.split(|&b| b == b'\n').count() as isize;

    for (row, line) in input.split(|&b| b == b'\n').enumerate() {
        for (col, &c) in line.iter().enumerate() {
            match c {
                b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => {
                    all_positions[c as usize].push((row as isize, col as isize))
                }
                _ => {}
            }
        }
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for positions in all_positions.into_iter().filter(|v| !v.is_empty()) {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];

                let (r3, c3) = (r1 + r1 - r2, c1 + c1 - c2);
                let (r4, c4) = (r2 + r2 - r1, c2 + c2 - c1);

                if r3 >= 0 && r3 < rows && c3 >= 0 && c3 < cols {
                    antinodes.insert((r3, c3));
                }

                if r4 >= 0 && r4 < rows && c4 >= 0 && c4 < cols {
                    antinodes.insert((r4, c4));
                }
            }
        }
    }

    antinodes.len()
}

pub fn part2(input: &[u8]) -> usize {
    let mut all_positions: Vec<Vec<(isize, isize)>> = vec![vec![]; 256];
    let cols = input.iter().take_while(|&&b| b != b'\n').count() as isize;
    let rows = input.split(|&b| b == b'\n').count() as isize;

    for (row, line) in input.split(|&b| b == b'\n').enumerate() {
        for (col, &c) in line.iter().enumerate() {
            match c {
                b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => {
                    all_positions[c as usize].push((row as isize, col as isize))
                }
                _ => {}
            }
        }
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for positions in all_positions.into_iter().filter(|v| !v.is_empty()) {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];

                let (mut r, mut c) = (r1, c1);

                while r >= 0 && r < rows && c >= 0 && c < cols {
                    antinodes.insert((r, c));
                    r += r1 - r2;
                    c += c1 - c2;
                }

                let (mut r, mut c) = (r2, c2);

                while r >= 0 && r < rows && c >= 0 && c < cols {
                    antinodes.insert((r, c));
                    r += r2 - r1;
                    c += c2 - c1;
                }
            }
        }
    }

    antinodes.len()
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day08.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 14);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 34);
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
