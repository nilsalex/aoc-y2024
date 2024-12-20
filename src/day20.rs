extern crate test;

use std::collections::HashMap;

const INPUT: &[u8] = include_bytes!("../inputs/day20.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    fn is_empty(&self, pos: &(isize, isize)) -> bool {
        matches!(self.cells[self.to_index(pos)], Cell::Empty)
    }

    fn set_wall(&mut self, pos: &(isize, isize)) {
        let ix = self.to_index(pos);
        self.cells[ix] = Cell::Wall;
    }

    fn to_index(&self, pos: &(isize, isize)) -> usize {
        pos.0 as usize * self.cols + pos.1 as usize
    }

    fn trace_path(&mut self) -> Vec<(isize, isize)> {
        let mut path = vec![];
        let mut pos = self.start;

        loop {
            path.push(pos);
            if pos == self.end {
                return path;
            }
            let next_positions = self.next(pos);
            assert_eq!(next_positions.len(), 1);
            self.set_wall(&pos);
            pos = next_positions[0];
        }
    }

    fn next(&self, pos: (isize, isize)) -> Vec<(isize, isize)> {
        [
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ]
        .into_iter()
        .filter(|&(row, col)| {
            row >= 0 && row < self.rows as isize && col >= 0 && col < self.cols as isize
        })
        .filter(|p| self.is_empty(p))
        .collect::<Vec<(isize, isize)>>()
    }
}

fn solve(input: &[u8], cheat_length: usize) -> HashMap<usize, usize> {
    let mut grid = Grid::from_bytes(input);
    let path = grid.trace_path();
    let distance_from_start: HashMap<(isize, isize), usize> = path
        .iter()
        .cloned()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect();

    let mut result = HashMap::new();

    for (d0, (i, j)) in path.iter().enumerate() {
        for a in i - (cheat_length as isize)..i + (cheat_length as isize + 1) {
            let abs_diff = a.abs_diff(*i) as isize;
            for b in
                j - (cheat_length as isize - abs_diff)..j + (cheat_length as isize - abs_diff) + 1
            {
                if a == *i && b == *j {
                    continue;
                }
                if let Some(&d1) = distance_from_start.get(&(a, b)) {
                    if d1 <= d0 {
                        continue;
                    }
                    let cheat_dist = a.abs_diff(*i) + b.abs_diff(*j);
                    if d0 + cheat_dist < d1 {
                        let saved = d1 - d0 - cheat_dist;
                        result.entry(saved).and_modify(|c| *c += 1).or_insert(1);
                    }
                }
            }
        }
    }

    result
}

pub fn part1(input: &[u8]) -> usize {
    let result = solve(input, 2);
    result
        .into_iter()
        .filter(|&(k, _)| k >= 100)
        .map(|(_, v)| v)
        .sum()
}

pub fn part2(input: &[u8]) -> usize {
    let result = solve(input, 20);
    result
        .into_iter()
        .filter(|&(k, _)| k >= 100)
        .map(|(_, v)| v)
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day20.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        let mut result = solve(input, 2).into_iter().collect::<Vec<_>>();
        result.sort();
        assert_eq!(result, [
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1)
        ]);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        let mut result = solve(input, 20)
            .into_iter()
            .filter(|&(k, _)| k >= 50)
            .collect::<Vec<_>>();
        result.sort();
        assert_eq!(result, [
            (50, 32),
            (52, 31),
            (54, 29),
            (56, 39),
            (58, 25),
            (60, 23),
            (62, 20),
            (64, 19),
            (66, 12),
            (68, 14),
            (70, 12),
            (72, 22),
            (74, 4),
            (76, 3)
        ]);
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
