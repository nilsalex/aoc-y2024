extern crate test;

use std::collections::HashSet;
use std::collections::VecDeque;

const INPUT: &[u8] = include_bytes!("../inputs/day12.txt");

struct Grid {
    rows: usize,
    cols: usize,
    cells: Vec<u8>,
}

impl Grid {
    fn from_bytes(bytes: &[u8]) -> Self {
        let cols = bytes.iter().take_while(|&b| *b != b'\n').count();
        let cells = bytes
            .iter()
            .filter(|&b| *b != b'\n')
            .cloned()
            .collect::<Vec<u8>>();
        let rows = cells.len() / cols;
        Self { rows, cols, cells }
    }

    fn to_coords(&self, index: usize) -> (usize, usize) {
        (index / self.cols, index % self.cols)
    }

    fn to_index(&self, coords: &(usize, usize)) -> usize {
        coords.0 * self.cols + coords.1
    }

    fn up(&self, index: usize) -> Option<(usize, u8)> {
        let (row, col) = self.to_coords(index);

        if row > 0 {
            let ix = self.to_index(&(row - 1, col));
            Some((ix, self.cells[ix]))
        } else {
            None
        }
    }

    fn down(&self, index: usize) -> Option<(usize, u8)> {
        let (row, col) = self.to_coords(index);

        if row < self.rows - 1 {
            let ix = self.to_index(&(row + 1, col));
            Some((ix, self.cells[ix]))
        } else {
            None
        }
    }

    fn left(&self, index: usize) -> Option<(usize, u8)> {
        let (row, col) = self.to_coords(index);

        if col > 0 {
            let ix = self.to_index(&(row, col - 1));
            Some((ix, self.cells[ix]))
        } else {
            None
        }
    }

    fn right(&self, index: usize) -> Option<(usize, u8)> {
        let (row, col) = self.to_coords(index);

        if col < self.cols - 1 {
            let ix = self.to_index(&(row, col + 1));
            Some((ix, self.cells[ix]))
        } else {
            None
        }
    }

    fn up_left(&self, index: usize) -> Option<(usize, u8)> {
        let (row, col) = self.to_coords(index);

        if row > 0 && col > 0 {
            let ix = self.to_index(&(row - 1, col - 1));
            Some((ix, self.cells[ix]))
        } else {
            None
        }
    }

    fn up_right(&self, index: usize) -> Option<(usize, u8)> {
        let (row, col) = self.to_coords(index);

        if row > 0 && col < self.cols - 1 {
            let ix = self.to_index(&(row - 1, col + 1));
            Some((ix, self.cells[ix]))
        } else {
            None
        }
    }

    fn down_left(&self, index: usize) -> Option<(usize, u8)> {
        let (row, col) = self.to_coords(index);

        if row < self.rows - 1 && col > 0 {
            let ix = self.to_index(&(row + 1, col - 1));
            Some((ix, self.cells[ix]))
        } else {
            None
        }
    }

    fn down_right(&self, index: usize) -> Option<(usize, u8)> {
        let (row, col) = self.to_coords(index);

        if row < self.rows - 1 && col < self.cols - 1 {
            let ix = self.to_index(&(row + 1, col + 1));
            Some((ix, self.cells[ix]))
        } else {
            None
        }
    }

    fn flood(&self, index: usize) -> (HashSet<usize>, usize, usize, usize) {
        let mut visited = HashSet::new();
        let mut corner_counts: Vec<usize> = vec![0; 5];
        let mut area = 0;
        let mut perimeter = 0;

        let mut queue = VecDeque::from([index]);

        while let Some(ix) = queue.pop_front() {
            if visited.contains(&ix) {
                continue;
            }

            visited.insert(ix);
            area += 1;

            let value = self.cells[ix];

            let up = self.up(ix);
            let down = self.down(ix);
            let left = self.left(ix);
            let right = self.right(ix);

            let neighs = [up, down, left, right]
                .into_iter()
                .flatten()
                .filter(|&(_, v)| v == value)
                .collect::<Vec<_>>();

            perimeter += 4 - neighs.len();

            match neighs.len() {
                0 => corner_counts[4] += 1,
                1 => corner_counts[3] += 1,
                2 => match neighs[0].0.abs_diff(neighs[1].0) {
                    val if val == 2 * self.cols || val == 2 => {}
                    _ => corner_counts[2] += 1,
                },
                _ => {}
            };

            match up {
                Some((_, v)) if v == value => {
                    match left {
                        Some((_, v)) if v == value => match self.up_left(ix) {
                            Some((_, v)) if v != value => corner_counts[2] += 1,
                            _ => {}
                        },
                        _ => {}
                    };
                    match right {
                        Some((_, v)) if v == value => match self.up_right(ix) {
                            Some((_, v)) if v != value => corner_counts[2] += 1,
                            _ => {}
                        },
                        _ => {}
                    };
                }
                _ => {}
            };

            match down {
                Some((_, v)) if v == value => {
                    match left {
                        Some((_, v)) if v == value => match self.down_left(ix) {
                            Some((_, v)) if v != value => corner_counts[2] += 1,
                            _ => {}
                        },
                        _ => {}
                    };
                    match right {
                        Some((_, v)) if v == value => match self.down_right(ix) {
                            Some((_, v)) if v != value => corner_counts[2] += 1,
                            _ => {}
                        },
                        _ => {}
                    };
                }
                _ => {}
            };

            for (neigh, _) in neighs {
                if !visited.contains(&neigh) {
                    queue.push_back(neigh);
                }
            }
        }

        let sides = if corner_counts[4] > 0 {
            4
        } else {
            2 * corner_counts[2] + 3 * corner_counts[3] - corner_counts[2] - corner_counts[3]
        };

        (visited, area, perimeter, sides)
    }
}

pub fn part1(input: &[u8]) -> usize {
    let grid = Grid::from_bytes(input);

    let mut result = 0;

    let mut visited = HashSet::new();
    for index in 0..grid.cells.len() {
        if visited.contains(&index) {
            continue;
        }

        let (visited_new, area, perimeter, _) = grid.flood(index);
        visited.extend(visited_new);
        result += area * perimeter;
    }

    result
}

pub fn part2(input: &[u8]) -> usize {
    let grid = Grid::from_bytes(input);

    let mut result = 0;

    let mut visited = HashSet::new();
    for index in 0..grid.cells.len() {
        if visited.contains(&index) {
            continue;
        }

        let (visited_new, area, _, sides) = grid.flood(index);

        visited.extend(visited_new);
        result += area * sides;
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day12.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 1930);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 1206);
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
