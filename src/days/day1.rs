use std::io::{self, Read, Write};
pub fn solve() {
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();

    let depths: Vec<u16> = input.lines().map(|line| line.parse().unwrap()).collect();

    // writeln!(output, "{}", part1(&depths)).unwrap();
    writeln!(output, "{}", part2(&depths)).unwrap();
}

fn part1(input: &[u16]) -> usize {
    let mut prev = u16::MAX;
    let mut ans: usize = 0;
    for &depth in input.iter() {
        if depth > prev {
            ans += 1;
        }
        prev = depth;
    }
    ans
}

fn part2(input: &[u16]) -> usize {
    let mut ans = 0;
    let mut prev = u16::MAX;

    for ind in 0..input.len() - 2 {
        let sum = input[ind] + input[ind + 1] + input[ind + 2];
        if sum > prev {
            ans += 1;
        }
        prev = sum;
    }

    ans
}
