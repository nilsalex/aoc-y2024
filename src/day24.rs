extern crate test;

use std::collections::HashMap;
use std::collections::VecDeque;

const INPUT: &[u8] = include_bytes!("../inputs/day24.txt");

type Gate = [u8; 3];

#[derive(Debug, Clone)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn len(&self) -> usize {
        match self {
            Op::And => 3,
            Op::Or => 2,
            Op::Xor => 3,
        }
    }

    fn apply(&self, in_1: bool, in_2: bool) -> bool {
        match self {
            Op::And => in_1 & in_2,
            Op::Or => in_1 | in_2,
            Op::Xor => in_1 ^ in_2,
        }
    }
}

fn prepare_values(x: usize, y: usize, len: usize) -> HashMap<Gate, bool> {
    let mut values = HashMap::new();

    for i in 0..len {
        let d1 = ((i / 10) as u8) + b'0';
        let d2 = ((i % 10) as u8) + b'0';
        values.insert([b'x', d1, d2], (x & (1 << i)) != 0);
        values.insert([b'y', d1, d2], (y & (1 << i)) != 0);
    }

    values
}

fn get_output(values: &HashMap<Gate, bool>, len: usize) -> usize {
    let mut output = 0;

    for i in (0..len).rev() {
        let d1 = ((i / 10) as u8) + b'0';
        let d2 = ((i % 10) as u8) + b'0';
        let value = values.get(&[b'z', d1, d2]).unwrap();
        output <<= 1;
        output += *value as usize;
    }

    output
}

fn run(gates: &HashMap<Gate, (Op, Gate, Gate)>, values: &mut HashMap<Gate, bool>, len: usize) {
    let mut queue = (0..len)
        .map(|i| {
            let d1 = ((i / 10) as u8) + b'0';
            let d2 = ((i % 10) as u8) + b'0';
            [b'z', d1, d2]
        })
        .collect::<VecDeque<_>>();

    while let Some(gate) = queue.pop_front() {
        let (op, in_1, in_2) = gates.get(&gate).unwrap();
        let val_1 = values.get(in_1);
        let val_2 = values.get(in_2);

        if val_1.is_none() {
            queue.push_back(*in_1);
        }

        if val_2.is_none() {
            queue.push_back(*in_2);
        }

        match val_1.is_some() && val_2.is_some() {
            true => {
                values.insert(gate, op.apply(*val_1.unwrap(), *val_2.unwrap()));
            }
            false => {
                queue.push_back(gate);
            }
        }
    }
}

pub fn part1(input: &[u8]) -> usize {
    let mut values = HashMap::new();
    let mut gates = HashMap::new();
    let mut outputs = vec![];

    let mut lines = input.split(|&b| b == b'\n');

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let gate: Gate = [line[0], line[1], line[2]];
        let value = match line.last() {
            Some(b'0') => false,
            Some(b'1') => true,
            _ => panic!(),
        };
        values.insert(gate, value);
    }

    for line in lines {
        let op = match line[4] {
            b'A' => Op::And,
            b'O' => Op::Or,
            b'X' => Op::Xor,
            _ => panic!(),
        };
        let d = op.len();
        let in_1: Gate = [line[0], line[1], line[2]];
        let in_2: Gate = [line[5 + d], line[6 + d], line[7 + d]];
        let out: Gate = [line[12 + d], line[13 + d], line[14 + d]];

        gates.insert(out, (op, in_1, in_2));

        if out[0] == b'z' {
            outputs.push(out);
        }
    }

    run(&gates, &mut values, outputs.len());

    get_output(&values, outputs.len())
}

pub fn part2(input: &[u8]) -> usize {
    let mut gates = HashMap::new();

    let mut lines = input.split(|&b| b == b'\n');

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
    }

    for line in lines {
        let op = match line[4] {
            b'A' => Op::And,
            b'O' => Op::Or,
            b'X' => Op::Xor,
            _ => panic!(),
        };
        let d = op.len();
        let in_1: Gate = [line[0], line[1], line[2]];
        let in_2: Gate = [line[5 + d], line[6 + d], line[7 + d]];
        let out: Gate = [line[12 + d], line[13 + d], line[14 + d]];

        gates.insert(out, (op, in_1, in_2));
    }

    let gate_tst = (*gates.get(b"tst").unwrap()).clone();
    let gate_z05 = (*gates.get(b"z05").unwrap()).clone();
    gates.insert([b't', b's', b't'], gate_z05);
    gates.insert([b'z', b'0', b'5'], gate_tst);

    let gate_sps = (*gates.get(b"sps").unwrap()).clone();
    let gate_z11 = (*gates.get(b"z11").unwrap()).clone();
    gates.insert([b's', b'p', b's'], gate_z11);
    gates.insert([b'z', b'1', b'1'], gate_sps);

    let gate_frt = (*gates.get(b"frt").unwrap()).clone();
    let gate_z23 = (*gates.get(b"z23").unwrap()).clone();
    gates.insert([b'f', b'r', b't'], gate_z23);
    gates.insert([b'z', b'2', b'3'], gate_frt);

    let gate_cgh = (*gates.get(b"cgh").unwrap()).clone();
    let gate_pmd = (*gates.get(b"pmd").unwrap()).clone();
    gates.insert([b'p', b'm', b'd'], gate_cgh);
    gates.insert([b'c', b'g', b'h'], gate_pmd);

    for i in 0..=44 {
        let x = 1 << i;
        let y = 0;
        let mut values = prepare_values(x, y, 45);
        run(&gates, &mut values, 46);
        let z = get_output(&values, 46);
        if x + y == z {
            continue;
        }
        println!("#### x bit {} ####", i);
        println!("x : {:0>46b}", x);
        println!("y : {:0>46b}", y);
        println!("z : {:0>46b}", z);
    }

    for i in 0..=44 {
        let x = 0;
        let y = 1 << i;
        let mut values = prepare_values(x, y, 45);
        run(&gates, &mut values, 46);
        let z = get_output(&values, 46);
        if x + y == z {
            continue;
        }
        println!("#### y bit {} ####", i);
        println!("x : {:0>46b}", x);
        println!("y : {:0>46b}", y);
        println!("z : {:0>46b}", z);
    }

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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day24.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 2024);
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
