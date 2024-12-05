const INPUT: &[u8] = include_bytes!("../inputs/day02.txt");

const POWERS_OF_TEN: [u32; 6] = [1, 10, 100, 1000, 10000, 100000];

fn u32_from_bytes(bytes: &[u8]) -> u32 {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as u32 * POWERS_OF_TEN[ix]
    })
}

pub fn part1(input: &[u8]) -> usize {
    input
        .split(|c| *c == b'\n')
        .filter(|line| {
            let mut vec = line
                .split(|c| *c == b' ')
                .map(u32_from_bytes)
                .collect::<Vec<u32>>();

            if vec[1] < vec[0] {
                vec.reverse();
            }

            for i in 0..vec.len() - 1 {
                if vec[i + 1] < vec[i] + 1 {
                    return false;
                }

                if vec[i + 1] > vec[i] + 3 {
                    return false;
                }
            }

            true
        })
        .count()
}

pub fn part2(input: &[u8]) -> usize {
    input
        .split(|c| *c == b'\n')
        .filter(|line| {
            let vec = line
                .split(|c| *c == b' ')
                .map(u32_from_bytes)
                .collect::<Vec<u32>>();

            'outer: for d in 0..vec.len() {
                let mut vec2 = vec
                    .iter()
                    .take(d)
                    .chain(vec.iter().skip(d + 1))
                    .cloned()
                    .collect::<Vec<u32>>();

                if vec2[1] < vec2[0] {
                    vec2.reverse();
                }

                for i in 0..vec2.len() - 1 {
                    if vec2[i + 1] < vec2[i] + 1 {
                        continue 'outer;
                    }

                    if vec2[i + 1] > vec2[i] + 3 {
                        continue 'outer;
                    }
                }
                return true;
            }

            false
        })
        .count()
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(input));
    println!("{}", part2(input));
}
