use std::io::{self, Read, Write};
pub fn solve() {
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();

    let lantern_fishes: Vec<u8> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    writeln!(output, "{}", part1(&lantern_fishes)).unwrap();
    writeln!(output, "{}", part2(&lantern_fishes)).unwrap();
}

fn part1(lantern_fishes: &[u8]) -> usize {
    let mut local_fishes: Vec<u8> = lantern_fishes.to_vec();
    for _ in 0..80 {
        simulate(&mut local_fishes);
    }
    local_fishes.len()
}
fn part2(lantern_fishes: &[u8]) -> usize {
    simulate_faster(&lantern_fishes)
}

fn simulate(fishes: &mut Vec<u8>) {
    let length = fishes.len();
    for index in 0..length {
        let days = fishes[index];
        if days == 0 {
            fishes[index] = 6;
            fishes.push(8);
        } else {
            fishes[index] -= 1;
        }
    }
}

fn simulate_faster(input: &[u8]) -> usize {
    let mut fishes: [usize; 9] = [0; 9];
    input.iter().for_each(|&days| {
        fishes[days as usize] += 1;
    });
    for _ in 0..256 {
        let temp = fishes[0];
        for index in 1..9 {
            fishes[index - 1] = fishes[index];
        }
        fishes[8] = temp;
        fishes[6] += temp;
    }
    fishes.iter().sum()
}
