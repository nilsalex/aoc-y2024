extern crate test;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

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

    const fn as_coords(&self) -> (isize, isize) {
        match self {
            Self::Left => (0, -1),
            Self::Right => (0, 1),
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
        }
    }

    const fn as_int(&self) -> usize {
        match self {
            Self::Left => 0,
            Self::Right => 1,
            Self::Up => 2,
            Self::Down => 3,
        }
    }

    const fn from_int(i: usize) -> Self {
        match i {
            0 => Self::Left,
            1 => Self::Right,
            2 => Self::Up,
            3 => Self::Down,
            _ => panic!(),
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

        Self {
            cols,
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
) -> Option<usize> {
    let mut dist = vec![usize::MAX; 4 * grid.cells.len()];
    let mut heap = BinaryHeap::new();

    dist[to_dist_index(grid, &start)] = 0;
    heap.push(State {
        cost: 0,
        node: start,
    });

    while let Some(State { cost, node }) = heap.pop() {
        if node == end {
            return Some(cost);
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
        .min()
        .unwrap()
}

pub fn part2(input: &[u8]) -> usize {
    let grid = input.split(|&b| b == b'\n').collect::<Vec<&[u8]>>();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut junctions = vec![];
    for row in 1..grid.len() - 1 {
        for col in 1..grid[0].len() - 1 {
            let pos = (row as isize, col as isize);
            match grid[row][col] {
                b'S' => {
                    start = pos;
                }
                b'E' => {
                    end = pos;
                }
                b'.' => {
                    if [
                        (pos.0 - 1, pos.1),
                        (pos.0 + 1, pos.1),
                        (pos.0, pos.1 - 1),
                        (pos.0, pos.1 + 1),
                    ]
                    .iter()
                    .filter(|&&(r, c)| grid[r as usize][c as usize] == b'.')
                    .count()
                        > 2
                    {
                        junctions.push(pos);
                    }
                }
                b'#' => {}
                _ => panic!(),
            }
        }
    }

    junctions.push(start);
    junctions.push(end);

    let num_nodes = junctions.len() * 4;

    let mut d = vec![usize::MAX; num_nodes * num_nodes];
    let mut adj = vec![vec![]; num_nodes];

    for i in 0..junctions.len() {
        for d1 in [Dir::Left, Dir::Right, Dir::Up, Dir::Down].iter() {
            let i_with_dir1 = i * 4 + d1.as_int();
            d[i_with_dir1 * num_nodes + i_with_dir1] = 0;
            adj[i_with_dir1].push((i_with_dir1, 0));
            for d2 in [Dir::Left, Dir::Right, Dir::Up, Dir::Down].iter() {
                if d1 == d2 {
                    continue;
                }
                let i_with_dir2 = i * 4 + d2.as_int();
                d[i_with_dir1 * num_nodes + i_with_dir2] = 1000;
                adj[i_with_dir1].push((i_with_dir2, 0));
            }
        }
    }

    for i in 0..junctions.len() {
        let coords = junctions[i];
        for dir in [Dir::Left, Dir::Right, Dir::Up, Dir::Down].iter() {
            let dir_coords = dir.as_coords();
            let mut state = (
                (coords.0 + dir_coords.0, coords.1 + dir_coords.1),
                *dir,
                1,
                1,
            );

            if grid[state.0.0 as usize][state.0.1 as usize] == b'#' {
                continue;
            }

            loop {
                let last_pos = state.0;
                let last_dir = state.1;
                let last_steps = state.2;
                let last_cells = state.3;

                if let Some(j) = junctions.iter().position(|&j| j == last_pos) {
                    let i_with_dir = i * 4 + dir.as_int();
                    let j_with_dir = j * 4 + last_dir.as_int();
                    d[i_with_dir * num_nodes + j_with_dir] = last_steps;
                    adj[i_with_dir].push((j_with_dir, last_cells));
                    break;
                }

                let last_dir_coords = last_dir.as_coords();

                let straight = (
                    (
                        last_pos.0 + last_dir_coords.0,
                        last_pos.1 + last_dir_coords.1,
                    ),
                    last_dir,
                    last_steps + 1,
                    last_cells + 1,
                );

                let next_dirs = last_dir.next();
                let turn_1 = (
                    (
                        last_pos.0 + next_dirs[0].as_coords().0,
                        last_pos.1 + next_dirs[0].as_coords().1,
                    ),
                    next_dirs[0],
                    last_steps + 1001,
                    last_cells + 1,
                );
                let turn_2 = (
                    (
                        last_pos.0 + next_dirs[1].as_coords().0,
                        last_pos.1 + next_dirs[1].as_coords().1,
                    ),
                    next_dirs[1],
                    last_steps + 1001,
                    last_cells + 1,
                );

                let next_states = &[straight, turn_1, turn_2]
                    .into_iter()
                    .filter(|&(p, _, _, _)| {
                        grid[p.0 as usize][p.1 as usize] == b'.'
                            || grid[p.0 as usize][p.1 as usize] == b'E'
                    })
                    .collect::<Vec<_>>();

                assert!(next_states.len() <= 1);

                if next_states.is_empty() {
                    break;
                }

                state = next_states[0];
            }
        }
    }

    let start_ix = (junctions.len() - 2) * 4 + Dir::Right.as_int();

    println!("{}", num_nodes);

    // floyd-warshall
    for k in 0..num_nodes {
        if k % 10 == 0 {
            println!("{}", k);
        }
        for i in 0..num_nodes {
            for j in 0..num_nodes {
                let ij = i * num_nodes + j;
                let ik = i * num_nodes + k;
                let kj = k * num_nodes + j;
                if let Some(checked_sum) = d[ik].checked_add(d[kj]) {
                    if checked_sum < d[ij] {
                        d[ij] = checked_sum;
                    }
                }
            }
        }
    }

    let mut min_dist = usize::MAX;
    let mut end_ix = 0;

    for dir in [Dir::Left, Dir::Right, Dir::Up, Dir::Down].into_iter() {
        let this_end_ix = (junctions.len() - 1) * 4 + dir.as_int();
        let end_dist = d[start_ix * num_nodes + this_end_ix];
        if min_dist > end_dist {
            min_dist = end_dist;
            end_ix = this_end_ix;
            break;
        }
    }

    let mut visited = HashSet::new();
    let mut visited_cells = 1;
    let mut added = HashSet::new();
    let mut targets = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start_ix);

    // bfs for all nodes with minimum distance between start and end, add segment length from adj to visited cells
    while let Some(node) = queue.pop_front() {
        if end_ix == node {
            continue;
        }

        if visited.contains(&node) {
            continue;
        }

        let next = adj[node]
            .iter()
            .cloned()
            .filter(|(n, _)| node != *n)
            .filter(|(n, _)| {
                let start_dist = d[start_ix * num_nodes + n];
                let end_dist = d[n * num_nodes + end_ix];
                start_dist + end_dist == min_dist
            })
            .collect::<Vec<(usize, usize)>>();
        for (n, l) in next {
            let to_add = ((n / 4).min(node / 4), (n / 4).max(node / 4));
            if !added.contains(&to_add) {
                visited_cells += l;
                if l > 0 {
                    if !targets.contains(&(n / 4)) {
                        targets.insert(n / 4);
                    } else {
                        visited_cells -= 1;
                    }
                }
                queue.push_back(n);
                added.insert(to_add);
                if l != 0 {
                    println!(
                        "from {:?}, {:?} to {:?}, {:?}: {}",
                        junctions[node / 4],
                        Dir::from_int(node % 4),
                        junctions[n / 4],
                        Dir::from_int(n % 4),
                        l
                    );
                }
            }
        }

        visited.insert(node);
    }

    visited_cells
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test::Bencher;

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

    //#[bench]
    //fn bench_part1(b: &mut Bencher) {
    //    let input = INPUT.trim_ascii_end();
    //    b.iter(|| part1(input))
    //}
    //
    //#[bench]
    //fn bench_part2(b: &mut Bencher) {
    //    let input = INPUT.trim_ascii_end();
    //    b.iter(|| part2(input))
    //}
}
