use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("../inputs/day05.txt");

const POWERS_OF_TEN: [u8; 2] = [1, 10];

fn u8_from_bytes(bytes: &[u8]) -> u8 {
    bytes
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (ix, x)| acc + (x & 0x0f) * POWERS_OF_TEN[ix])
}

pub fn top_sort(graph: &[(u8, u8)]) -> Vec<u8> {
    let mut vertices: Vec<u8> = graph.iter().fold(Vec::new(), |mut acc, (a, b)| {
        acc.push(*a);
        acc.push(*b);
        acc
    });
    vertices.sort();
    vertices.dedup();

    let mut initial = vertices
        .iter()
        .filter(|&v| !graph.iter().any(|&(_, b)| v == &b))
        .cloned()
        .collect::<Vec<u8>>();
    let mut sorted = Vec::new();

    let mut graph = Vec::from(graph);

    while let Some(n) = initial.pop() {
        sorted.push(n);
        for v in &vertices {
            if !graph.contains(&(n, *v)) {
                continue;
            }
            graph = graph
                .iter()
                .cloned()
                .filter(|&edge| edge != (n, *v))
                .collect();
            if !graph.iter().any(|&(_, b)| b == *v) {
                initial.push(*v);
            }
        }
    }

    sorted
}

pub fn part1(input: &[u8]) -> usize {
    let mut lines = input.split(|&b| b == b'\n');

    let mut rules: HashSet<(u8, u8)> = HashSet::new();

    loop {
        let edge = lines.next().unwrap();
        if edge.is_empty() {
            break;
        }
        let a = u8_from_bytes(&edge[0..2]);
        let b = u8_from_bytes(&edge[3..5]);
        rules.insert((a, b));
    }

    let mut sequences = Vec::new();

    for sequence in lines {
        sequences.push(
            sequence
                .split(|&b| b == b',')
                .map(u8_from_bytes)
                .collect::<Vec<u8>>(),
        );
    }

    let mut result = 0;

    'outer: for sequence in sequences {
        for i in 0..sequence.len() {
            for j in i + 1..sequence.len() {
                if rules.contains(&(sequence[j], sequence[i])) {
                    continue 'outer;
                }
            }
        }
        result += sequence[sequence.len() / 2] as usize;
    }

    result
}

pub fn part2(input: &[u8]) -> usize {
    let mut lines = input.split(|&b| b == b'\n');

    let mut graph = Vec::new();

    loop {
        let edge = lines.next().unwrap();
        if edge.is_empty() {
            break;
        }
        let a = u8_from_bytes(&edge[0..2]);
        let b = u8_from_bytes(&edge[3..5]);
        graph.push((a, b));
    }

    let mut sequences = Vec::new();

    for sequence in lines {
        sequences.push(
            sequence
                .split(|&b| b == b',')
                .map(u8_from_bytes)
                .collect::<Vec<u8>>(),
        );
    }

    let mut result = 0;

    for sequence in sequences {
        let subgraph = graph
            .iter()
            .filter(|&(a, b)| sequence.contains(a) && sequence.contains(b))
            .cloned()
            .collect::<Vec<(u8, u8)>>();

        let sorted = top_sort(&subgraph);
        if sorted != sequence {
            result += sorted[sorted.len() / 2] as usize;
        }
    }

    result
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(input));
    println!("{}", part2(input));
}
