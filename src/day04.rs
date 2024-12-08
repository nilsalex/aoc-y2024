extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day04.txt");

#[derive(Debug)]
struct Grid {
    num_rows: usize,
    num_cols: usize,
    cells: Vec<u8>,
}

impl Grid {
    fn from_bytes(bytes: &[u8]) -> Self {
        let num_cols = bytes.iter().take_while(|&&b| b != b'\n').count();
        let cells = bytes
            .iter()
            .filter(|&&b| b != b'\n')
            .cloned()
            .collect::<Vec<u8>>();
        let num_rows = cells.len() / num_cols;

        Grid {
            num_rows,
            num_cols,
            cells,
        }
    }

    fn get_cell(&self, row: isize, col: isize) -> u8 {
        if 0 <= row && row < self.num_rows as isize && 0 <= col && col < self.num_cols as isize {
            self.cells[row as usize * self.num_cols + col as usize]
        } else {
            b'.'
        }
    }
}

pub fn part1(input: &[u8]) -> usize {
    let grid = Grid::from_bytes(input);

    let mut result = 0;

    for row in 0..(grid.num_rows as isize) {
        for col in 0..(grid.num_cols as isize) {
            let candidates = [
                [(row, col), (row, col + 1), (row, col + 2), (row, col + 3)],
                [(row, col), (row, col - 1), (row, col - 2), (row, col - 3)],
                [(row, col), (row + 1, col), (row + 2, col), (row + 3, col)],
                [(row, col), (row - 1, col), (row - 2, col), (row - 3, col)],
                [
                    (row, col),
                    (row + 1, col + 1),
                    (row + 2, col + 2),
                    (row + 3, col + 3),
                ],
                [
                    (row, col),
                    (row - 1, col - 1),
                    (row - 2, col - 2),
                    (row - 3, col - 3),
                ],
                [
                    (row, col),
                    (row + 1, col - 1),
                    (row + 2, col - 2),
                    (row + 3, col - 3),
                ],
                [
                    (row, col),
                    (row - 1, col + 1),
                    (row - 2, col + 2),
                    (row - 3, col + 3),
                ],
            ];

            for candidate in candidates {
                let x = grid.get_cell(candidate[0].0, candidate[0].1);
                let m = grid.get_cell(candidate[1].0, candidate[1].1);
                let a = grid.get_cell(candidate[2].0, candidate[2].1);
                let s = grid.get_cell(candidate[3].0, candidate[3].1);

                if x == b'X' && m == b'M' && a == b'A' && s == b'S' {
                    result += 1
                }
            }
        }
    }

    result
}

pub fn part2(input: &[u8]) -> usize {
    let grid = Grid::from_bytes(input);

    let mut result = 0;

    for row in 0..(grid.num_rows as isize) {
        for col in 0..(grid.num_cols as isize) {
            if grid.get_cell(row, col) != b'A' {
                continue;
            }

            let mut corners = [
                grid.get_cell(row - 1, col - 1),
                grid.get_cell(row - 1, col + 1),
                grid.get_cell(row + 1, col - 1),
                grid.get_cell(row + 1, col + 1),
            ];

            if corners[0] == corners[3] || corners[1] == corners[2] {
                continue;
            }

            corners.sort();

            if corners == [b'M', b'M', b'S', b'S'] {
                result += 1;
            }
        }
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day04.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 18);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 9);
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
