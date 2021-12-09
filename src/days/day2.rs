use std::fs;

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

pub fn solve() {
    let input = fs::read_to_string("inputs/day1.txt").expect("Input not provided");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let lines = input.lines();
    let commands: Vec<Command> = parse_lines(lines);

    let (horz, vert) = simulate(&commands);

    horz * vert
}

fn part2(input: &str) -> i32 {
    let lines = input.lines();
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

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        use super::*;
        let input = fs::read_to_string("src/samples/day2.txt").expect("Input not provided");
        assert_eq!(part1(&input), 150);
    }

    #[test]
    fn part2() {
        use super::*;
        let input = fs::read_to_string("src/samples/day2.txt").expect("Input not provided");
        assert_eq!(part2(&input), 900);
    }
}
