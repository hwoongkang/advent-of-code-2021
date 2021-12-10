use std::fs;

fn char_map(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}
fn part2_map(ch: char) -> usize {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

pub fn solve() {
    let input = fs::read_to_string("src/inputs/day10.txt").expect("No input provided");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn matches(ch1: char, ch2: char) -> bool {
    match ch1 {
        '(' => ch2 == ')',
        '[' => ch2 == ']',
        '{' => ch2 == '}',
        '<' => ch2 == '>',
        _ => false,
    }
}

fn matching_bracket(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => ' ',
    }
}

fn is_corrupted(line: &str) -> Option<char> {
    let mut stack: Vec<char> = vec![];
    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => {
                if stack.is_empty() {
                    return Some(ch);
                }
                let last = stack.pop().unwrap();
                if !matches(last, ch) {
                    return Some(ch);
                }
            }
            _ => {}
        }
    }
    None
}

fn remaining(line: &str) -> Option<Vec<char>> {
    let mut stack: Vec<char> = vec![];
    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => {
                if stack.is_empty() {
                    return None;
                }
                let last = stack.pop().unwrap();
                if !matches(last, ch) {
                    return None;
                }
            }
            _ => {}
        }
    }
    Some(stack)
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| is_corrupted(line))
        .map(|ch| char_map(ch))
        .sum()
}

fn part2(input: &str) -> usize {
    let mut temp: Vec<usize> = input
        .lines()
        .filter_map(|line| remaining(line))
        .map(|brackets| {
            brackets
                .iter()
                .rev()
                .map(|&ch| matching_bracket(ch))
                .map(|ch| part2_map(ch))
                .fold(0, |acc, x| 5 * acc + x)
        })
        .collect();
    temp.sort();
    temp[temp.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        fs::read_to_string("src/samples/day10.txt").expect("No input provided")
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&get_input()), 26397);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(&get_input()), 288957);
    }
}
