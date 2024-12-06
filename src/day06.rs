use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("../inputs/day06.txt");

#[derive(Clone)]
struct Grid {
    cells: HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Guard {
    position: (isize, isize),
    direction: Direction,
}

impl Grid {
    fn from_bytes(bytes: &[u8]) -> Self {
        let cols = bytes.iter().take_while(|&&b| b != b'\n').count();
        let rows = bytes.split(|&b| b == b'\n').count();
        let cells = bytes
            .split(|&b| b == b'\n')
            .enumerate()
            .flat_map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .filter(|&(_, &c)| c == b'#')
                    .map(move |(j, _)| (i, j))
            })
            .collect::<HashSet<(usize, usize)>>();
        Self { cells, rows, cols }
    }

    fn is_out_of_bounds(&self, pos: &(isize, isize)) -> bool {
        pos.0 < 0 || pos.0 >= self.rows as isize || pos.1 < 0 || pos.1 >= self.cols as isize
    }

    fn is_wall(&self, pos: &(isize, isize)) -> bool {
        self.cells.contains(&(pos.0 as usize, pos.1 as usize))
    }

    fn cloned_with_extra_cell(&self, pos: &(usize, usize)) -> Option<Self> {
        if self.cells.contains(pos) {
            return None;
        }
        let mut cloned = self.clone();
        cloned.cells.insert(*pos);
        Some(cloned)
    }
}

impl Direction {
    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            b'^' => Some(Self::Up),
            b'v' => Some(Self::Down),
            b'<' => Some(Self::Left),
            b'>' => Some(Self::Right),
            _ => None,
        }
    }

    fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl Guard {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        for (i, line) in bytes.split(|&b| b == b'\n').enumerate() {
            for (j, &c) in line.iter().enumerate() {
                if let Some(direction) = Direction::from_byte(c) {
                    return Some(Self {
                        position: (i as isize, j as isize),
                        direction,
                    });
                }
            }
        }
        None
    }

    fn next_pos(&self) -> (isize, isize) {
        match self.direction {
            Direction::Up => (self.position.0 - 1, self.position.1),
            Direction::Down => (self.position.0 + 1, self.position.1),
            Direction::Left => (self.position.0, self.position.1 - 1),
            Direction::Right => (self.position.0, self.position.1 + 1),
        }
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.next();
    }
}

pub fn part1(input: &[u8]) -> usize {
    let grid = Grid::from_bytes(input);
    let mut guard = Guard::from_bytes(input).unwrap();

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert(guard.position);

    loop {
        let next_pos = guard.next_pos();
        if grid.is_out_of_bounds(&next_pos) {
            return visited.len();
        }
        if grid.is_wall(&next_pos) {
            guard.turn_right();
            continue;
        }
        guard.position = next_pos;
        visited.insert(next_pos);
    }
}

pub fn part2(input: &[u8]) -> usize {
    let grid = Grid::from_bytes(input);
    let guard = Guard::from_bytes(input).unwrap();

    let mut initial_guard = guard.clone();
    let mut initial_path: Vec<Guard> = vec![initial_guard.clone()];

    loop {
        let next_pos = initial_guard.next_pos();
        if grid.is_out_of_bounds(&next_pos) {
            break;
        }
        if grid.is_wall(&next_pos) {
            initial_guard.turn_right();
            continue;
        }
        initial_guard.position = next_pos;
        initial_path.push(initial_guard.clone());
    }

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut initial_path_deduplicated: Vec<Guard> = Vec::new();
    for guard in initial_path.iter().skip(1) {
        if visited.insert(guard.position) {
            initial_path_deduplicated.push(guard.clone());
        }
    }

    let mut result = 0;

    for initial_guard in initial_path_deduplicated {
        let position_in_initial_path = initial_path
            .iter()
            .position(|g| *g == initial_guard)
            .unwrap();
        let extra_cell_pos = initial_guard.position;
        let grid = grid
            .cloned_with_extra_cell(&(extra_cell_pos.0 as usize, extra_cell_pos.1 as usize))
            .unwrap();
        let mut guard = initial_path[position_in_initial_path - 1].clone();
        let mut visited = initial_path[0..position_in_initial_path]
            .iter()
            .cloned()
            .collect::<HashSet<Guard>>();
        visited.insert(guard.clone());
        loop {
            let next_pos = guard.next_pos();
            if grid.is_out_of_bounds(&next_pos) {
                break;
            }
            if grid.is_wall(&next_pos) {
                guard.turn_right();
                continue;
            }
            guard.position = next_pos;
            if !visited.insert(guard.clone()) {
                result += 1;
                break;
            };
        }
    }

    result
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(input));
    println!("{}", part2(input));
}
