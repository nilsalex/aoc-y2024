extern crate test;

use std::collections::VecDeque;

const INPUT: &[u8] = include_bytes!("../inputs/day10.txt");

struct Grid {
    rows: usize,
    cols: usize,
    cells: Vec<u8>,
}

impl Grid {
    fn from_bytes(bytes: &[u8]) -> Self {
        let cols = bytes.iter().take_while(|&&b| b != b'\n').count();
        let cells = bytes
            .iter()
            .filter(|&&b| b != b'\n')
            .map(|&b| b - b'0')
            .collect::<Vec<_>>();
        let rows = cells.len() / cols;

        Self { rows, cols, cells }
    }

    fn to_coords(&self, index: usize) -> (usize, usize) {
        (index / self.cols, index % self.cols)
    }

    fn to_index(&self, coords: (usize, usize)) -> usize {
        coords.0 * self.cols + coords.1
    }

    fn get(&self, coords: (usize, usize)) -> u8 {
        self.cells[self.to_index(coords)]
    }

    fn get_zeros(&self) -> impl Iterator<Item = (usize, usize)> {
        self.cells
            .iter()
            .enumerate()
            .filter(|&(_, c)| *c == 0)
            .map(|(i, _)| self.to_coords(i))
    }

    fn is_final(&self, coords: (usize, usize)) -> bool {
        self.get(coords) == 9
    }

    fn next(&self, coords: (usize, usize)) -> Vec<(usize, usize)> {
        let val = self.get(coords);
        let mut result = vec![];
        if coords.0 > 0 && self.get((coords.0 - 1, coords.1)) == val + 1 {
            result.push((coords.0 - 1, coords.1))
        }
        if coords.1 > 0 && self.get((coords.0, coords.1 - 1)) == val + 1 {
            result.push((coords.0, coords.1 - 1))
        }
        if coords.0 < self.rows - 1 && self.get((coords.0 + 1, coords.1)) == val + 1 {
            result.push((coords.0 + 1, coords.1))
        }
        if coords.1 < self.cols - 1 && self.get((coords.0, coords.1 + 1)) == val + 1 {
            result.push((coords.0, coords.1 + 1))
        }
        result
    }

    fn bfs_without_trail(&self, start: (usize, usize)) -> usize {
        let mut found = vec![0; self.cells.len()];
        let mut queue = VecDeque::from([start]);

        while let Some(node) = queue.pop_front() {
            let next = self.next(node);

            for n in next {
                if self.is_final(n) {
                    found[self.to_index(n)] += 1;
                } else {
                    queue.push_back(n);
                }
            }
        }

        found.iter().filter(|&&b| b > 0).count()
    }

    fn reconstruct_paths(
        cur_paths: Vec<Vec<usize>>,
        predecessors: &Vec<Vec<usize>>,
    ) -> Vec<Vec<usize>> {
        cur_paths
            .into_iter()
            .flat_map(|cur_path| {
                let cur_node = cur_path.last().unwrap();
                let preds = &predecessors[*cur_node];
                if preds.is_empty() {
                    return vec![cur_path];
                }

                let next_paths = preds
                    .iter()
                    .map(|p| {
                        let mut next_path = Vec::with_capacity(cur_path.len() + 1);
                        next_path.extend_from_slice(&cur_path);
                        next_path.push(*p);
                        next_path
                    })
                    .collect::<Vec<Vec<usize>>>();
                Self::reconstruct_paths(next_paths, predecessors)
            })
            .collect::<Vec<Vec<usize>>>()
    }

    fn bfs(&self, start: (usize, usize)) -> usize {
        let mut found = vec![0; self.cells.len()];
        let mut predecessors = vec![vec![]; self.cells.len()];
        let mut queue = VecDeque::from([start]);

        while let Some(node) = queue.pop_front() {
            let next = self.next(node);

            for n in next {
                predecessors[self.to_index(n)].push(self.to_index(node));
                if self.is_final(n) {
                    found[self.to_index(n)] += 1;
                } else {
                    queue.push_back(n);
                }
            }
        }

        predecessors.iter_mut().for_each(|preds| {
            preds.sort();
            preds.dedup();
        });

        let path_heads = found
            .iter()
            .enumerate()
            .filter_map(|(i, &b)| if b > 0 { Some(vec![i]) } else { None })
            .collect::<Vec<Vec<usize>>>();

        let mut paths = Self::reconstruct_paths(path_heads, &predecessors);

        paths.sort();
        paths.dedup();
        paths.len()
    }
}

pub fn part1(input: &[u8]) -> usize {
    let grid = Grid::from_bytes(input);
    grid.get_zeros().map(|n| grid.bfs_without_trail(n)).sum()
}

pub fn part2(input: &[u8]) -> usize {
    let grid = Grid::from_bytes(input);
    grid.get_zeros().map(|n| grid.bfs(n)).sum()
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day10.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 36);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 81);
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
