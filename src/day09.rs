extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day09.txt");

pub fn part1(input: &[u8]) -> usize {
    let mut filesystem = Vec::new();
    let mut empty_stack = Vec::new();
    let mut non_empty_stack = Vec::new();

    let mut cur_file = 0;

    let mut iter = input.iter();
    let mut cur_pos = 0;

    while let Some(byte) = iter.next() {
        for _ in 0..(byte - b'0') as usize {
            filesystem.push(Some(cur_file));
            non_empty_stack.push(cur_pos);
            cur_pos += 1;
        }

        if let Some(byte) = iter.next() {
            for _ in 0..(byte - b'0') as usize {
                filesystem.push(None);
                empty_stack.push(cur_pos);
                cur_pos += 1;
            }
        } else {
            break;
        }

        cur_file += 1;
    }

    empty_stack.reverse();

    while let Some(empty_pos) = empty_stack.pop() {
        if let Some(non_empty_pos) = non_empty_stack.pop() {
            if non_empty_pos < empty_pos {
                break;
            }
            filesystem.swap(empty_pos, non_empty_pos);
        } else {
            break;
        }
    }

    let mut result = 0;

    for (i, v) in filesystem.iter().enumerate() {
        if let Some(val) = v {
            result += i * val;
        } else {
            break;
        }
    }

    result
}

pub fn part2(input: &[u8]) -> usize {
    let mut files = Vec::new();

    let mut cur_file = 0;

    let mut iter = input.iter();
    let mut cur_pos = 0;

    while let Some(byte) = iter.next() {
        let count = (byte - b'0') as usize;
        files.push((cur_file, cur_pos, count));

        cur_pos += count;

        if let Some(byte) = iter.next() {
            let count = (byte - b'0') as usize;
            cur_pos += count;
        } else {
            break;
        }

        cur_file += 1;
    }

    let reversed = files.iter().cloned().rev().collect::<Vec<_>>();

    for (file_num, start, count) in reversed.iter() {
        let mut new_pos = None;

        for i in 0..files.len() - 1 {
            let (_, start_1, count_1) = files[i];
            let (_, start_2, _) = files[i + 1];
            if start_1 >= *start {
                break;
            }
            let gap = start_2 - (start_1 + count_1);
            if *count <= gap {
                new_pos = Some((i, start_1 + count_1));
                break;
            }
        }

        if let Some((new_pos, new_start)) = new_pos {
            let mut new_files = Vec::new();

            for (i, file) in files.iter().enumerate() {
                if file.0 == *file_num {
                    continue;
                }
                new_files.push(*file);
                if i == new_pos {
                    new_files.push((*file_num, new_start, *count));
                }
            }

            std::mem::swap(&mut files, &mut new_files);
        }
    }

    let mut result = 0;

    for (file_num, start, count) in files {
        for i in 0..count {
            result += file_num * (start + i);
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day09.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 1928);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 2858);
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
