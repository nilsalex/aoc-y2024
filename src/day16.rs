extern crate test;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

const INPUT: &[u8] = include_bytes!("../inputs/day16.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    const fn next(&self) -> [Self; 2] {
        match self {
            Self::Left => [Self::Up, Self::Down],
            Self::Right => [Self::Up, Self::Down],
            Self::Up => [Self::Left, Self::Right],
            Self::Down => [Self::Left, Self::Right],
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Wall,
    Empty,
}

#[derive(Debug)]
struct Grid {
    cols: usize,
    rows: usize,
    cells: Vec<Cell>,
    start: (isize, isize),
    end: (isize, isize),
}

impl Grid {
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut cells = vec![];
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (row, line) in bytes.split(|&b| b == b'\n').enumerate() {
            for (col, b) in line.iter().enumerate() {
                match b {
                    b'#' => cells.push(Cell::Wall),
                    b'.' => cells.push(Cell::Empty),
                    b'S' => {
                        cells.push(Cell::Empty);
                        start = (row as isize, col as isize)
                    }
                    b'E' => {
                        cells.push(Cell::Empty);
                        end = (row as isize, col as isize)
                    }
                    _ => panic!("invalid byte: {}", *b as char),
                }
            }
        }

        let cols = bytes.iter().take_while(|&&b| b != b'\n').count();
        let rows = cells.len() / cols;

        Self {
            cols,
            rows,
            cells,
            start,
            end,
        }
    }

    fn get(&self, pos: (isize, isize)) -> Cell {
        self.cells[self.to_index(pos)]
    }

    fn to_index(&self, pos: (isize, isize)) -> usize {
        pos.0 as usize * self.cols + pos.1 as usize
    }

    fn next(&self, pos: (isize, isize), dir: Dir) -> Option<(isize, isize)> {
        let next_pos = match dir {
            Dir::Left => (pos.0, pos.1 - 1),
            Dir::Right => (pos.0, pos.1 + 1),
            Dir::Up => (pos.0 - 1, pos.1),
            Dir::Down => (pos.0 + 1, pos.1),
        };

        if matches!(self.get(next_pos), Cell::Empty) {
            Some(next_pos)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: ((isize, isize), Dir),
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

fn to_dist_index(grid: &Grid, pos_and_dir: &((isize, isize), Dir)) -> usize {
    4 * grid.to_index(pos_and_dir.0) + pos_and_dir.1 as usize
}

fn edges(grid: &Grid, pos_and_dir: &((isize, isize), Dir)) -> Vec<State> {
    let mut result = Vec::with_capacity(3);
    let next_dirs = pos_and_dir.1.next();

    result.extend([
        State {
            cost: 1000,
            node: (pos_and_dir.0, next_dirs[0]),
        },
        State {
            cost: 1000,
            node: (pos_and_dir.0, next_dirs[1]),
        },
    ]);

    if let Some(next) = grid.next(pos_and_dir.0, pos_and_dir.1) {
        result.push(State {
            cost: 1,
            node: (next, pos_and_dir.1),
        });
    }

    result
}

fn dijkstra(
    grid: &Grid,
    start: ((isize, isize), Dir),
    end: ((isize, isize), Dir),
) -> Option<(usize, Vec<Option<usize>>)> {
    let mut dist = vec![usize::MAX; 4 * grid.cells.len()];
    let mut prev = vec![None; 4 * grid.cells.len()];
    let mut heap = BinaryHeap::new();

    dist[to_dist_index(grid, &start)] = 0;
    heap.push(State {
        cost: 0,
        node: start,
    });

    while let Some(State { cost, node }) = heap.pop() {
        if node == end {
            return Some((cost, prev));
        }

        let node_index = to_dist_index(grid, &node);

        if cost > dist[node_index] {
            continue;
        }

        for edge in edges(grid, &node) {
            let next = State {
                cost: cost + edge.cost,
                node: edge.node,
            };

            let next_index = to_dist_index(grid, &next.node);

            if next.cost < dist[next_index] {
                heap.push(next);
                dist[next_index] = next.cost;
                prev[next_index] = Some(node_index);
            }
        }
    }

    None
}

pub fn part1(input: &[u8]) -> usize {
    let grid = Grid::from_bytes(input);

    let start = (grid.start, Dir::Right);

    [Dir::Left, Dir::Right, Dir::Up, Dir::Down]
        .into_iter()
        .filter_map(|dir| dijkstra(&grid, start, (grid.end, dir)))
        .map(|res| res.0)
        .min()
        .unwrap()
}

pub fn part2(input: &[u8]) -> usize {
    0
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day16.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 7036);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 45);
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
