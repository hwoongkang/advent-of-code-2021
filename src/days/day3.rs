use std::io::{self, Read, Write};
pub fn solve() {
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    writeln!(output, "{}", part1(&lines)).unwrap();
}

// ans: 4160394
fn part1(lines: &[&str]) -> u32 {
    let binary: Vec<Vec<bool>> = lines
        .iter()
        .map(|line| line.trim().chars().map(|ch| ch == '1').collect())
        .collect();
    let mut counts: Vec<u16> = vec![0; binary[0].len()];
    binary.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(ind, &is_one)| {
            if is_one {
                counts[ind] += 1;
            }
        });
    });
    let gamma_rate: Vec<bool> = counts
        .iter()
        .map(|&count| count > binary.len() as u16 / 2)
        .collect();
    let epsilon_rate: Vec<bool> = counts
        .iter()
        .map(|&count| count <= binary.len() as u16 / 2)
        .collect();
    binary_to_decimal(&gamma_rate) * binary_to_decimal(&epsilon_rate)
}

fn binary_to_decimal(binary: &[bool]) -> u32 {
    let mut pow = 1;
    let mut ans = 0;
    for &add in binary.iter().rev() {
        if add {
            ans += pow;
        }
        pow *= 2;
    }
    ans
}
