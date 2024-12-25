extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day15.txt");

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left,
    Right,
    Up,
    Down,
}

impl Instruction {
    fn from_byte(b: u8) -> Self {
        match b {
            b'<' => Self::Left,
            b'>' => Self::Right,
            b'^' => Self::Up,
            b'v' => Self::Down,
            _ => panic!("invalid byte"),
        }
    }

    fn get_dir(&self) -> (isize, isize) {
        match self {
            Self::Left => (0, -1),
            Self::Right => (0, 1),
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Wall,
    Box,
    Empty,
}

#[derive(Debug, Clone, Copy)]
enum WideCell {
    Wall,
    BoxLeft,
    BoxRight,
    Empty,
}

#[derive(Debug)]
struct Grid {
    cols: usize,
    cells: Vec<Cell>,
    robot: (isize, isize),
}

#[derive(Debug)]
struct WideGrid {
    cols: usize,
    cells: Vec<WideCell>,
    robot: (isize, isize),
}

impl Grid {
    fn from_bytes(bytes: &[u8]) -> Self {
        let lines = bytes.split(|&b| b == b'\n');
        let mut cells = vec![];
        let mut robot = (0, 0);

        for (row, line) in lines.enumerate() {
            if line.is_empty() {
                break;
            }

            for (col, b) in line.iter().enumerate() {
                match b {
                    b'#' => cells.push(Cell::Wall),
                    b'O' => cells.push(Cell::Box),
                    b'.' => cells.push(Cell::Empty),
                    b'@' => {
                        cells.push(Cell::Empty);
                        robot = (row as isize, col as isize)
                    }
                    _ => panic!("invalid byte"),
                }
            }
        }

        let cols = bytes.iter().take_while(|&&b| b != b'\n').count();

        Self { cols, cells, robot }
    }

    fn score(&self) -> usize {
        self.cells
            .iter()
            .enumerate()
            .map(|(ix, c)| match c {
                Cell::Box => {
                    let row = ix / self.cols;
                    let col = ix % self.cols;
                    100 * row + col
                }
                _ => 0,
            })
            .sum()
    }

    fn get(&self, pos: (isize, isize)) -> Cell {
        self.cells[pos.0 as usize * self.cols + pos.1 as usize]
    }

    fn set(&mut self, pos: (isize, isize), val: Cell) {
        self.cells[pos.0 as usize * self.cols + pos.1 as usize] = val
    }

    fn next_empty_pos(&self, pos: (isize, isize), dir: (isize, isize)) -> Option<(isize, isize)> {
        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
        match self.get(next_pos) {
            Cell::Wall => None,
            Cell::Empty => Some(next_pos),
            Cell::Box => self.next_empty_pos(next_pos, dir),
        }
    }

    fn move_robot(&mut self, dir: (isize, isize)) {
        let next_pos = (self.robot.0 + dir.0, self.robot.1 + dir.1);
        let next_val = self.get(next_pos);

        match next_val {
            Cell::Wall => {}
            Cell::Empty => self.robot = next_pos,
            Cell::Box => {
                if let Some(next_empty_pos) = self.next_empty_pos(next_pos, dir) {
                    self.robot = next_pos;
                    self.set(next_pos, Cell::Empty);
                    self.set(next_empty_pos, Cell::Box);
                }
            }
        }
    }
}

impl WideGrid {
    fn from_bytes(bytes: &[u8]) -> Self {
        let lines = bytes.split(|&b| b == b'\n');
        let mut cells = vec![];
        let mut robot = (0, 0);

        for (row, line) in lines.enumerate() {
            if line.is_empty() {
                break;
            }

            for (col, b) in line.iter().enumerate() {
                match b {
                    b'#' => {
                        cells.push(WideCell::Wall);
                        cells.push(WideCell::Wall);
                    }
                    b'O' => {
                        cells.push(WideCell::BoxLeft);
                        cells.push(WideCell::BoxRight);
                    }
                    b'.' => {
                        cells.push(WideCell::Empty);
                        cells.push(WideCell::Empty);
                    }
                    b'@' => {
                        cells.push(WideCell::Empty);
                        cells.push(WideCell::Empty);
                        robot = (row as isize, 2 * col as isize)
                    }
                    _ => panic!("invalid byte"),
                }
            }
        }

        let cols = 2 * bytes.iter().take_while(|&&b| b != b'\n').count();

        Self { cols, cells, robot }
    }

    fn score(&self) -> usize {
        self.cells
            .iter()
            .enumerate()
            .map(|(ix, c)| match c {
                WideCell::BoxLeft => {
                    let row = ix / self.cols;
                    let col = ix % self.cols;
                    100 * row + col
                }
                _ => 0,
            })
            .sum()
    }

    fn get(&self, pos: (isize, isize)) -> WideCell {
        self.cells[pos.0 as usize * self.cols + pos.1 as usize]
    }

    fn set(&mut self, pos: (isize, isize), val: WideCell) {
        self.cells[pos.0 as usize * self.cols + pos.1 as usize] = val
    }

    //fn next_empty_pos(&self, pos: (isize, isize), dir: (isize, isize)) -> Option<(isize, isize)> {
    //    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
    //    match self.get(next_pos) {
    //        Cell::Wall => None,
    //        Cell::Empty => Some(next_pos),
    //        Cell::Box => self.next_empty_pos(next_pos, dir),
    //    }
    //}

    fn move_robot(&mut self, dir: (isize, isize)) {
        let next_pos = (self.robot.0 + dir.0, self.robot.1 + dir.1);
        let next_val = self.get(next_pos);

        match next_val {
            WideCell::Wall => {}
            WideCell::Empty => self.robot = next_pos,
            WideCell::BoxLeft | WideCell::BoxRight => {
                //if dir.1 == 0 {
                //} else {
                //}
                //if let Some(next_empty_pos) = self.next_empty_pos(next_pos, dir) {
                //    self.robot = next_pos;
                //    self.set(next_pos, Cell::Empty);
                //    self.set(next_empty_pos, Cell::Box);
                //}
            }
        }
    }
}

pub fn part1(input: &[u8]) -> usize {
    let mut grid = Grid::from_bytes(input);

    let instructions = input
        .split(|&b| b == b'\n')
        .skip_while(|&line| !line.is_empty())
        .skip(1)
        .flat_map(|line| line.iter().map(|&b| Instruction::from_byte(b)));
    for instruction in instructions {
        grid.move_robot(instruction.get_dir());
    }
    grid.score()
}

pub fn part2(input: &[u8]) -> usize {
    let mut grid = WideGrid::from_bytes(input);

    let instructions = input
        .split(|&b| b == b'\n')
        .skip_while(|&line| !line.is_empty())
        .skip(1)
        .flat_map(|line| line.iter().map(|&b| Instruction::from_byte(b)));
    for instruction in instructions {
        grid.move_robot(instruction.get_dir());
    }
    grid.score()
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

    const TEST_INPUT_SMALL: &[u8] = include_bytes!("../test_inputs/day15_small.txt");
    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day15.txt");

    #[test]
    fn test_part1_small() {
        let input = TEST_INPUT_SMALL.trim_ascii_end();
        assert_eq!(part1(input), 2028);
    }

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 10092);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 9021);
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