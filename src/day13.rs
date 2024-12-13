extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day13.txt");

const POWERS_OF_TEN: [isize; 6] = [1, 10, 100, 1000, 10000, 100000];

fn isize_from_bytes(bytes: &[u8]) -> isize {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as isize * POWERS_OF_TEN[ix]
    })
}

const COST_A: usize = 3;
const COST_B: usize = 1;

const OFFSET: isize = 10000000000000;

fn pair_from_bytes(bytes: &[u8]) -> (isize, isize) {
    let mut iter = bytes.split(|&b| b == b',');
    let left = iter.next().unwrap();
    let right = iter.next().unwrap();

    let x = isize_from_bytes(left.split(|&b| b == b'+' || b == b'=').nth(1).unwrap());
    let y = isize_from_bytes(right.split(|&b| b == b'+' || b == b'=').nth(1).unwrap());
    (x, y)
}

/*
 *
 *  i * a1 + j * b1 = p1
 *  i * a2 + j * b2 = p2
 *
 * i * (a1 * a2) + j * (a2 * b1) = a2 * p1
 * i * (a1 * a2) + j * (a1 * b2) = a1 * p2
 *
 * j * [ a1 * b2 - a2 * b1 ] = a1 * p2 - a2 * p1
 *
 * j = [ a1 * p2 - a2 * p1 ] / [ a1 * b2 - a2 * b1 ]
 *
 * i = [ p1 - j * b1 ] / a1
 *
 * i = p1 / a1 - [ b1 * p2 - b1 * a2 * p1 / a1 ] / [ a1 * b2 - a2 * b1 ]
 *
 * i = [ b2 * p1 - b1 * p2 ] / [ a1 * b2 - a2 * b1 ]
 *
 *
 * */

fn solve(input: &[u8], offset: isize) -> usize {
    let mut iter = input.split(|&b| b == b'\n');

    let mut result = 0;

    while let Some(line) = iter.next() {
        let (a_x, a_y) = pair_from_bytes(line);
        let (b_x, b_y) = pair_from_bytes(iter.next().unwrap());
        let (p_x, p_y) = pair_from_bytes(iter.next().unwrap());
        let (p_x, p_y) = (p_x + offset, p_y + offset);

        let det = a_x * b_y - a_y * b_x;
        let alpha = b_y * p_x - b_x * p_y;
        let beta = a_x * p_y - a_y * p_x;

        if alpha % det == 0 && beta % det == 0 {
            let i = alpha / det;
            let j = beta / det;

            println!("i: {}, j: {}", i, j);

            if i >= 0 && j >= 0 {
                result += i as usize * COST_A;
                result += j as usize * COST_B;
            }
        }

        if iter.next().is_none() {
            break;
        }
    }

    result
}

pub fn part1(input: &[u8]) -> usize {
    solve(input, 0)
}

pub fn part2(input: &[u8]) -> usize {
    solve(input, OFFSET)
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day13.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 480);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 875318608908);
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
