use std::io::{self, Read, Write};

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

pub fn solve() {
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();

    // writeln!(output, "{}", part1(input.lines())).unwrap();
    writeln!(output, "{}", part2(input.lines())).unwrap();
}

fn part1(lines: std::str::Lines) -> i32 {
    let commands: Vec<Command> = parse_lines(lines);

    let (horz, vert) = simulate(&commands);

    horz * vert
}

fn part2(lines: std::str::Lines) -> i32 {
    let commands: Vec<Command> = parse_lines(lines);

    let (horz, vert) = simulate2(&commands);

    horz * vert
}

fn parse_lines(lines: std::str::Lines) -> Vec<Command> {
    lines
        .map(|line| {
            let mut words = line.split_whitespace();
            let command = words.next().unwrap();
            let num: i32 = words.next().unwrap().parse().unwrap();
            match command {
                "forward" => Command::Forward(num),
                "down" => Command::Down(num),
                _ => Command::Up(num),
            }
        })
        .collect()
}

fn simulate(commands: &[Command]) -> (i32, i32) {
    let mut horizontal = 0;
    let mut depth = 0;
    for command in commands {
        match command {
            Command::Forward(num) => horizontal += num,
            Command::Down(num) => depth += num,
            Command::Up(num) => depth -= num,
        }
    }
    (horizontal, depth)
}

fn simulate2(commands: &[Command]) -> (i32, i32) {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;
    for command in commands {
        match command {
            Command::Forward(num) => {
                horizontal += num;
                depth += aim * num;
            }
            Command::Down(num) => aim += num,
            Command::Up(num) => aim -= num,
        }
    }
    (horizontal, depth)
}
