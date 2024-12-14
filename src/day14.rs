extern crate test;

use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/day14.txt");

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

#[derive(Debug)]
struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}

impl Robot {
    fn from_line(line: &str) -> Self {
        let mut iter = line.split(['=', ',', ' ']);
        let px = iter.nth(1).unwrap().parse().unwrap();
        let py = iter.next().unwrap().parse().unwrap();
        let vx = iter.nth(1).unwrap().parse().unwrap();
        let vy = iter.next().unwrap().parse().unwrap();

        Self {
            pos: (px, py),
            vel: (vx, vy),
        }
    }

    fn advance(&mut self, steps: isize, bounds: (isize, isize)) {
        let x = (self.pos.0 + self.vel.0 * steps).rem_euclid(bounds.0);
        let y = (self.pos.1 + self.vel.1 * steps).rem_euclid(bounds.1);

        self.pos = (x, y);
    }

    fn get_quadrant(&self, bounds: (isize, isize)) -> Option<usize> {
        #[allow(clippy::comparison_chain)]
        if self.pos.0 < bounds.0 / 2 {
            if self.pos.1 < bounds.1 / 2 {
                Some(0)
            } else if self.pos.1 > bounds.1 / 2 {
                Some(1)
            } else {
                None
            }
        } else if self.pos.0 > bounds.0 / 2 {
            if self.pos.1 < bounds.1 / 2 {
                Some(2)
            } else if self.pos.1 > bounds.1 / 2 {
                Some(3)
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn part1_with_bounds(input: &str, width: isize, height: isize) -> usize {
    let mut counts = [0; 4];
    for line in input.lines() {
        let mut robot = Robot::from_line(line);
        println!("{:?}", robot);
        robot.advance(100, (width, height));
        println!("{:?}", robot);
        if let Some(quadrant) = robot.get_quadrant((width, height)) {
            counts[quadrant] += 1;
        }
    }
    counts.iter().product()
}

pub fn part2_with_bounds(input: &str, width: isize, height: isize) -> usize {
    let mut robots: Vec<Robot> = input.lines().map(Robot::from_line).collect();
    let mut steps = 0;

    loop {
        steps += 1;

        let mut distinct_positions = HashSet::new();
        for robot in robots.iter_mut() {
            robot.advance(1, (width, height));
            distinct_positions.insert(robot.pos);
        }

        if distinct_positions.len() == robots.len() {
            return steps;
        }
    }
}

pub fn part1(input: &str) -> usize {
    part1_with_bounds(input, WIDTH, HEIGHT)
}

pub fn part2(input: &str) -> usize {
    part2_with_bounds(input, WIDTH, HEIGHT)
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

    const TEST_INPUT: &str = include_str!("../test_inputs/day14.txt");

    const WIDTH_TEST: isize = 11;
    const HEIGHT_TEST: isize = 7;

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1_with_bounds(input, WIDTH_TEST, HEIGHT_TEST), 12);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2_with_bounds(input, WIDTH_TEST, HEIGHT_TEST), 1);
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
