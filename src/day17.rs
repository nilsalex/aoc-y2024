extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day17.txt");

const POWERS_OF_TEN: [usize; 10] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];

fn usize_from_bytes(bytes: &[u8]) -> usize {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as usize * POWERS_OF_TEN[ix]
    })
}

fn combo(operand: u8, a: usize, b: usize, c: usize) -> usize {
    match operand {
        0..=3 => operand as usize,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!(),
    }
}

fn literal(operand: u8) -> usize {
    operand as usize
}

fn run_once(program: &[u8], a: usize) -> (usize, usize) {
    let mut a = a;
    let mut b = 0;
    let mut c = 0;

    let mut ix = 0;

    while ix < program.len() {
        let op = program[ix];
        let operand = program[ix + 1];

        match op {
            0 => a = a >> combo(operand, a, b, c),
            1 => {
                b ^= literal(operand);
            }
            2 => {
                b = combo(operand, a, b, c) & 0b111;
            }
            3 => {
                if a != 0 {
                    ix = literal(operand);
                    continue;
                }
            }
            4 => {
                b ^= c;
            }
            5 => {
                let out_val = combo(operand, a, b, c) & 0b111;
                return (out_val, a);
            }
            6 => b = a >> combo(operand, a, b, c),
            7 => c = a >> combo(operand, a, b, c),
            _ => panic!(),
        }

        ix += 2;
    }

    panic!();
}

fn next(a: usize, x: usize, program: &[u8]) -> Vec<usize> {
    let a = a << 3;

    (0..8)
        .map(|i| a ^ i)
        .filter(|&b| run_once(program, b).0 == x)
        .collect::<Vec<_>>()
}

fn dfs(a: usize, depth: usize, program: &[u8]) -> Option<usize> {
    if depth == program.len() {
        return Some(a);
    }

    let target = program[program.len() - depth - 1];

    next(a, target as usize, program)
        .iter()
        .flat_map(|n| dfs(*n, depth + 1, program))
        .next()
}

pub fn part1(input: &[u8]) -> String {
    let mut lines = input.split(|&b| b == b'\n');

    let mut a = usize_from_bytes(lines.next().unwrap().split(|&b| b == b' ').nth(2).unwrap());
    let mut b = usize_from_bytes(lines.next().unwrap().split(|&b| b == b' ').nth(2).unwrap());
    let mut c = usize_from_bytes(lines.next().unwrap().split(|&b| b == b' ').nth(2).unwrap());

    let program = lines
        .nth(1)
        .unwrap()
        .split(|&b| b == b' ')
        .nth(1)
        .unwrap()
        .split(|&b| b == b',')
        .map(|bs| bs[0] - b'0')
        .collect::<Vec<u8>>();

    let mut ix = 0;
    let mut out = vec![];

    while ix < program.len() {
        let op = program[ix];
        let operand = program[ix + 1];

        match op {
            0 => a = a >> combo(operand, a, b, c),
            1 => {
                b ^= literal(operand);
            }
            2 => {
                b = combo(operand, a, b, c) % 8;
            }
            3 => {
                if a != 0 {
                    ix = literal(operand);
                    continue;
                }
            }
            4 => {
                b ^= c;
            }
            5 => {
                out.push((combo(operand, a, b, c) % 8) as u8);
            }
            6 => b = a >> combo(operand, a, b, c),
            7 => c = a >> combo(operand, a, b, c),
            _ => panic!(),
        }

        ix += 2;
    }

    out.iter()
        .map(|o| o.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn part2(input: &[u8]) -> usize {
    let program = input
        .split(|&b| b == b'\n')
        .nth(4)
        .unwrap()
        .split(|&b| b == b' ')
        .nth(1)
        .unwrap()
        .split(|&b| b == b',')
        .map(|bs| bs[0] - b'0')
        .collect::<Vec<u8>>();

    dfs(0, 0, &program).unwrap()
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

    const TEST_INPUT_1: &[u8] = include_bytes!("../test_inputs/day17_part1.txt");
    const TEST_INPUT_2: &[u8] = include_bytes!("../test_inputs/day17_part2.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT_1.trim_ascii_end();
        assert_eq!(part1(input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT_2.trim_ascii_end();
        assert_eq!(part2(input), 117440);
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
