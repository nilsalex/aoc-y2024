extern crate test;

use std::collections::HashSet;
use std::iter::once;

const INPUT: &[u8] = include_bytes!("../inputs/day23.txt");

const DIM: usize = 26 * 26;

fn to_int(a: u8, b: u8) -> usize {
    (a - b'a') as usize * 26 + (b - b'a') as usize
}

fn from_int(i: usize) -> (u8, u8) {
    (b'a' + (i / 26) as u8, b'a' + (i % 26) as u8)
}

pub fn part1(input: &[u8]) -> usize {
    let mut adj = vec![vec![]; DIM];

    let mut start_nodes = HashSet::new();

    for line in input.split(|&b| b == b'\n') {
        let node_1_a = line[0];
        let node_1_b = line[1];

        let node_2_a = line[3];
        let node_2_b = line[4];

        let node_1 = to_int(node_1_a, node_1_b);
        let node_2 = to_int(node_2_a, node_2_b);

        adj[node_1].push(node_2);
        adj[node_2].push(node_1);

        if node_1_a == b't' {
            start_nodes.insert(node_1);
        }

        if node_2_a == b't' {
            start_nodes.insert(node_2);
        }
    }

    let cycles_1 = start_nodes
        .iter()
        .flat_map(|s| {
            adj[*s]
                .iter()
                .map(|n| vec![*s, *n])
                .collect::<Vec<Vec<usize>>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let cycles_2 = cycles_1
        .iter()
        .flat_map(|cycle| {
            adj[cycle[1]]
                .iter()
                .map(|n| {
                    cycle
                        .iter()
                        .cloned()
                        .chain(once(*n))
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let cycles_3 = cycles_2
        .iter()
        .flat_map(|cycle| {
            adj[cycle[2]]
                .iter()
                .map(|n| {
                    cycle
                        .iter()
                        .cloned()
                        .chain(once(*n))
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .filter(|cycle| cycle[0] == cycle[3])
        .map(|cycle| {
            let mut sorted = cycle.clone();
            sorted.sort();
            sorted.dedup();
            sorted
        })
        .collect::<HashSet<Vec<usize>>>();

    cycles_3.len()
}

fn bron_kerbosch(
    adj: &[Vec<usize>],
    r: HashSet<usize>,
    mut p: HashSet<usize>,
    mut x: HashSet<usize>,
    cliques: &mut Vec<HashSet<usize>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
    } else {
        let nodes = p.iter().cloned().collect::<Vec<usize>>();

        for v in nodes {
            let next: HashSet<usize> = adj[v].iter().cloned().collect();
            let mut next_r = r.clone();
            next_r.insert(v);
            let next_p = p.intersection(&next).cloned().collect();
            let next_x = x.intersection(&next).cloned().collect();
            bron_kerbosch(adj, next_r, next_p, next_x, cliques);
            p.remove(&v);
            x.insert(v);
        }
    }
}

pub fn part2(input: &[u8]) -> String {
    let mut adj = vec![vec![]; DIM];

    let mut nodes = HashSet::new();

    for line in input.split(|&b| b == b'\n') {
        let node_1_a = line[0];
        let node_1_b = line[1];

        let node_2_a = line[3];
        let node_2_b = line[4];

        let node_1 = to_int(node_1_a, node_1_b);
        let node_2 = to_int(node_2_a, node_2_b);

        adj[node_1].push(node_2);
        adj[node_2].push(node_1);

        nodes.insert(node_1);
        nodes.insert(node_2);
    }

    let p = nodes;
    let r = HashSet::new();
    let x = HashSet::new();
    let mut cliques = Vec::new();

    bron_kerbosch(&adj, r, p, x, &mut cliques);

    let max_component_members = cliques.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap();

    let mut result = max_component_members
        .iter()
        .map(|n| {
            let (a, b) = from_int(*n);
            format!("{}{}", a as char, b as char)
        })
        .collect::<Vec<String>>();
    result.sort();
    result.join(",")
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day23.txt");

    #[test]
    fn to_from_int() {
        assert_eq!(to_int(b'a', b'a'), 0);
        assert_eq!(to_int(b'a', b'b'), 1);
        assert_eq!(to_int(b'b', b'b'), 27);
        assert_eq!(from_int(to_int(b'x', b't')), (b'x', b't'));
        assert_eq!(to_int(from_int(5).0, from_int(5).1), 5);
        assert_eq!(to_int(from_int(1234).0, from_int(1234).1), 1234);
    }

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 7);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), "co,de,ka,ta".to_string());
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
