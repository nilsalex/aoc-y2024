extern crate test;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

const INPUT: &[u8] = include_bytes!("../inputs/day18.txt");

const POWERS_OF_TEN: [usize; 3] = [1, 10, 100];

fn usize_from_bytes(bytes: &[u8]) -> usize {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as usize * POWERS_OF_TEN[ix]
    })
}

fn next(grid_size: usize, corrupted: &[u8], pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if pos.0 > 0 {
        let candidate = (pos.0 - 1, pos.1);
        if corrupted[candidate.0 * grid_size + candidate.1] == 0 {
            result.push(candidate);
        }
    }

    if pos.0 < grid_size - 1 {
        let candidate = (pos.0 + 1, pos.1);
        if corrupted[candidate.0 * grid_size + candidate.1] == 0 {
            result.push(candidate);
        }
    }

    if pos.1 > 0 {
        let candidate = (pos.0, pos.1 - 1);
        if corrupted[candidate.0 * grid_size + candidate.1] == 0 {
            result.push(candidate);
        }
    }

    if pos.1 < grid_size - 1 {
        let candidate = (pos.0, pos.1 + 1);
        if corrupted[candidate.0 * grid_size + candidate.1] == 0 {
            result.push(candidate);
        }
    }

    result
}

fn next2(
    grid_size: usize,
    corrupted: &[usize],
    num: usize,
    pos: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if pos.0 > 0 {
        let candidate = (pos.0 - 1, pos.1);
        if corrupted[candidate.0 * grid_size + candidate.1] > num {
            result.push(candidate);
        }
    }

    if pos.0 < grid_size - 1 {
        let candidate = (pos.0 + 1, pos.1);
        if corrupted[candidate.0 * grid_size + candidate.1] > num {
            result.push(candidate);
        }
    }

    if pos.1 > 0 {
        let candidate = (pos.0, pos.1 - 1);
        if corrupted[candidate.0 * grid_size + candidate.1] > num {
            result.push(candidate);
        }
    }

    if pos.1 < grid_size - 1 {
        let candidate = (pos.0, pos.1 + 1);
        if corrupted[candidate.0 * grid_size + candidate.1] > num {
            result.push(candidate);
        }
    }

    result
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid_size: usize, corrupted: &[u8]) -> Option<usize> {
    let mut dist = vec![usize::MAX; grid_size * grid_size];
    let mut heap = BinaryHeap::new();

    dist[0] = 0;
    heap.push(State {
        cost: 0,
        node: (0, 0),
    });

    while let Some(State { cost, node }) = heap.pop() {
        if node == (grid_size - 1, grid_size - 1) {
            return Some(cost);
        }

        if cost > dist[node.0 * grid_size + node.1] {
            continue;
        }

        for next_node in next(grid_size, corrupted, node) {
            let next_state = State {
                cost: cost + 1,
                node: next_node,
            };

            if next_state.cost < dist[next_node.0 * grid_size + next_node.1] {
                heap.push(next_state);
                dist[next_node.0 * grid_size + next_node.1] = next_state.cost;
            }
        }
    }

    None
}

fn dijkstra2(grid_size: usize, corrupted: &[usize], num: usize) -> Option<usize> {
    let mut dist = vec![usize::MAX; grid_size * grid_size];
    let mut heap = BinaryHeap::new();

    dist[0] = 0;
    heap.push(State {
        cost: 0,
        node: (0, 0),
    });

    while let Some(State { cost, node }) = heap.pop() {
        if node == (grid_size - 1, grid_size - 1) {
            return Some(cost);
        }

        if cost > dist[node.0 * grid_size + node.1] {
            continue;
        }

        for next_node in next2(grid_size, corrupted, num, node) {
            let next_state = State {
                cost: cost + 1,
                node: next_node,
            };

            if next_state.cost < dist[next_node.0 * grid_size + next_node.1] {
                heap.push(next_state);
                dist[next_node.0 * grid_size + next_node.1] = next_state.cost;
            }
        }
    }

    None
}

fn part1_with_parameters(input: &[u8], grid_size: usize, steps: usize) -> usize {
    let mut corrupted = vec![0; grid_size * grid_size];

    input
        .split(|&b| b == b'\n')
        .take(steps)
        .map(|line| {
            let mut iter = line.split(|&b| b == b',');
            let col = usize_from_bytes(iter.next().unwrap());
            let row = usize_from_bytes(iter.next().unwrap());
            row * grid_size + col
        })
        .for_each(|ix| corrupted[ix] = 1);

    dijkstra(grid_size, &corrupted).unwrap()
}

pub fn part1(input: &[u8]) -> usize {
    part1_with_parameters(input, 71, 1024)
}

fn part2_with_parameters(input: &[u8], grid_size: usize) -> (usize, usize) {
    let mut corrupted = vec![0; grid_size * grid_size];
    let mut bytes = vec![];

    for (num, line) in input.split(|&b| b == b'\n').enumerate() {
        let mut iter = line.split(|&b| b == b',');
        let col = usize_from_bytes(iter.next().unwrap());
        let row = usize_from_bytes(iter.next().unwrap());
        let ix = row * grid_size + col;

        if corrupted[ix] == 0 {
            corrupted[ix] = num;
        }

        bytes.push((col, row));
    }

    let mut good = 0;
    let mut bad = corrupted.len() - 1;

    for line in corrupted.chunks(grid_size) {
        for c in line {
            print!(" {:02} ", c);
        }
        println!();
    }

    while bad - good > 1 {
        let mid = good + (bad - good) / 2;
        println!("testing {}", mid);
        match dijkstra2(grid_size, &corrupted, mid) {
            Some(_) => {
                println!("  good!");
                good = mid;
            }
            _ => {
                println!("  bad!");
                bad = mid;
            }
        };
    }

    bytes[bad]
}

pub fn part2(input: &[u8]) -> String {
    let (col, row) = part2_with_parameters(input, 71);
    format!("{},{}", col, row)
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day18.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1_with_parameters(input, 7, 12), 22);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2_with_parameters(input, 7), (6, 1));
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
