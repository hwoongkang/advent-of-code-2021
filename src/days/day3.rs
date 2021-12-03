use std::collections::HashSet;
use std::io::{self, Read, Write};

pub fn solve() {
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    writeln!(output, "{}", part1(&lines)).unwrap();
    writeln!(output, "{}", part2(&lines)).unwrap();
}

fn part1(lines: &[&str]) -> u32 {
    let length = lines[0].trim().len();
    let binaries: Vec<u16> = lines
        .iter()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect();
    let mut counts: Vec<u16> = vec![0; length];
    for binary in &binaries {
        for shift in 0..length {
            let predicate = 1 << shift;
            let is_one = (binary & predicate) ^ predicate;
            counts[shift] += if is_one == 0 { 0 } else { 1 };
        }
    }
    let length = binaries.len();
    let gamma_rate: Vec<bool> = counts
        .iter()
        .map(|&count| count > length as u16 / 2)
        .collect();
    let epsilon_rate: Vec<bool> = counts
        .iter()
        .map(|&count| count <= length as u16 / 2)
        .collect();
    binary_to_decimal(&gamma_rate) * binary_to_decimal(&epsilon_rate)
}

fn part2(lines: &[&str]) -> u32 {
    let length = lines[0].trim().len();
    let mut binaries: HashSet<u16> = lines
        .iter()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect();
    let mut bit = 1 << (length - 1);
    let mut oxygen = binaries.clone();
    while oxygen.len() > 1 {
        oxygen = filter(oxygen, bit, true);
        bit >>= 1;
    }
    let oxygen: u32 = *(oxygen.iter().next().unwrap()) as u32;

    bit = 1 << (length - 1);
    while binaries.len() > 1 {
        binaries = filter(binaries, bit, false);
        bit >>= 1;
    }
    let carbo: u32 = *(binaries.iter().next().unwrap()) as u32;
    oxygen * carbo
}

fn filter(set: HashSet<u16>, bit: u16, for_oxygen: bool) -> HashSet<u16> {
    let size = set.len();
    let count_ones: usize = set
        .iter()
        .filter_map(|&num| {
            let is_one = (num & bit) ^ bit == 0;
            if is_one {
                Some(())
            } else {
                None
            }
        })
        .count();
    let predicate: bool = if for_oxygen {
        count_ones * 2 >= size
    } else {
        count_ones * 2 < size
    };
    set.iter()
        .filter_map(|&num| {
            let is_one = (num & bit) ^ bit == 0;
            let can_return = !(predicate ^ is_one);
            if can_return {
                Some(num)
            } else {
                None
            }
        })
        .collect()
}

fn binary_to_decimal(binary: &[bool]) -> u32 {
    let mut pow = 1;
    let mut ans = 0;
    for &add in binary.iter() {
        if add {
            ans += pow;
        }
        pow *= 2;
    }
    ans
}
