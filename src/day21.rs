extern crate test;

use memoize::memoize;
use std::iter::once;

const INPUT: &[u8] = include_bytes!("../inputs/day21.txt");

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    A,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Num {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Key {
    Dir(Dir),
    Num(Num),
}

impl Key {
    fn to_coords(self) -> (isize, isize) {
        match self {
            Self::Dir(dir) => match dir {
                Dir::Up => (0, 1),
                Dir::Down => (1, 1),
                Dir::Left => (1, 0),
                Dir::Right => (1, 2),
                Dir::A => (0, 2),
            },
            Self::Num(num) => match num {
                Num::Zero => (3, 1),
                Num::One => (2, 0),
                Num::Two => (2, 1),
                Num::Three => (2, 2),
                Num::Four => (1, 0),
                Num::Five => (1, 1),
                Num::Six => (1, 2),
                Num::Seven => (0, 0),
                Num::Eight => (0, 1),
                Num::Nine => (0, 2),
                Num::A => (3, 2),
            },
        }
    }

    fn is_forbidden(&self, edge: &(isize, isize)) -> bool {
        match self {
            Self::Num(_) => *edge == (3, 0),
            Self::Dir(_) => *edge == (0, 0),
        }
    }

    fn steps_to(&self, to: &Self) -> Vec<Vec<Dir>> {
        let this_coords = self.to_coords();
        let to_coords = to.to_coords();
        let d = (to_coords.0 - this_coords.0, to_coords.1 - this_coords.1);

        let ver = match d.0 {
            0 => None,
            1.. => Some((Dir::Down, d.0 as usize)),
            _ => Some((Dir::Up, d.0.unsigned_abs())),
        };
        let hor = match d.1 {
            0 => None,
            1.. => Some((Dir::Right, d.1 as usize)),
            _ => Some((Dir::Left, d.1.unsigned_abs())),
        };

        match ver {
            None => match hor {
                None => vec![vec![]],
                Some((x, i)) => vec![(0..i).map(|_| x).collect()],
            },
            Some((x, i)) => match hor {
                None => vec![(0..i).map(|_| x).collect()],
                Some((y, j)) => {
                    let mut result = vec![];

                    let edge_1 = (this_coords.0 + d.0, this_coords.1);
                    let edge_2 = (this_coords.0, this_coords.1 + d.1);

                    if !self.is_forbidden(&edge_1) {
                        result.push(
                            (0..i)
                                .map(|_| x)
                                .chain((0..j).map(|_| y))
                                .collect::<Vec<_>>(),
                        );
                    }

                    if !self.is_forbidden(&edge_2) {
                        result.push(
                            (0..j)
                                .map(|_| y)
                                .chain((0..i).map(|_| x))
                                .collect::<Vec<_>>(),
                        );
                    }

                    result
                }
            },
        }
    }
}

#[memoize]
fn steps_for_robot(from: Key, to: Key, robot: usize) -> usize {
    if robot == 0 {
        1
    } else {
        let paths = from.steps_to(&to);
        paths
            .iter()
            .map(|path| {
                let mut s = 0;
                let mut prev = Dir::A;
                for p in path.iter().chain(once(&Dir::A)) {
                    s += steps_for_robot(Key::Dir(prev), Key::Dir(*p), robot - 1);
                    prev = *p;
                }
                s
            })
            .min()
            .unwrap()
    }
}

impl Num {
    fn from_byte(byte: u8) -> Self {
        match byte {
            b'0' => Self::Zero,
            b'1' => Self::One,
            b'2' => Self::Two,
            b'3' => Self::Three,
            b'4' => Self::Four,
            b'5' => Self::Five,
            b'6' => Self::Six,
            b'7' => Self::Seven,
            b'8' => Self::Eight,
            b'9' => Self::Nine,
            b'A' => Self::A,
            _ => panic!(),
        }
    }

    fn to_value(self) -> Option<usize> {
        match self {
            Num::Zero => Some(0),
            Num::One => Some(1),
            Num::Two => Some(2),
            Num::Three => Some(3),
            Num::Four => Some(4),
            Num::Five => Some(5),
            Num::Six => Some(6),
            Num::Seven => Some(7),
            Num::Eight => Some(8),
            Num::Nine => Some(9),
            Num::A => None,
        }
    }
}

pub fn part1(input: &[u8]) -> usize {
    input
        .split(|&b| b == b'\n')
        .map(|line| {
            let digits = line
                .iter()
                .map(|&b| Num::from_byte(b))
                .collect::<Vec<Num>>();
            let mut prev = Num::A;
            let mut min_steps = 0;
            for digit in digits.iter() {
                min_steps += steps_for_robot(Key::Num(prev), Key::Num(*digit), 3);
                prev = *digit;
            }
            let num = digits
                .iter()
                .filter_map(|d| d.to_value())
                .fold(0, |acc, v| acc * 10 + v);
            min_steps * num
        })
        .sum()
}

pub fn part2(input: &[u8]) -> usize {
    input
        .split(|&b| b == b'\n')
        .map(|line| {
            let digits = line
                .iter()
                .map(|&b| Num::from_byte(b))
                .collect::<Vec<Num>>();
            let mut prev = Num::A;
            let mut min_steps = 0;
            for digit in digits.iter() {
                min_steps += steps_for_robot(Key::Num(prev), Key::Num(*digit), 26);
                prev = *digit;
            }
            let num = digits
                .iter()
                .filter_map(|d| d.to_value())
                .fold(0, |acc, v| acc * 10 + v);
            min_steps * num
        })
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day21.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 126384);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 154115708116294);
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
