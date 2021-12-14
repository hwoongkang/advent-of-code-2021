use std::collections::HashMap;
use std::fs;

struct Polymer {
    chain: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl Polymer {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let chain = lines.next().unwrap().chars().collect();
        let rules = lines
            .skip(1)
            .map(|line| {
                let mut rule = line.split("->");
                let mut left = rule.next().unwrap().trim().chars();
                let a = left.next().unwrap();
                let b = left.next().unwrap();
                let right = rule.next().unwrap().trim().chars().next().unwrap();
                ((a, b), right)
            })
            .collect();

        Polymer { chain, rules }
    }

    fn tick(&mut self) {
        let mut new_chain: Vec<char> = vec![];
        for i in 0..self.chain.len() - 1 {
            let first = self.chain[i];
            let second = self.chain[i + 1];
            new_chain.push(first);
            if let Some(mid) = self.rules.get(&(first, second)) {
                new_chain.push(*mid);
            }
        }
        new_chain.push(self.chain[self.chain.len() - 1]);
        self.chain = new_chain;
    }

    fn faster_tick(&self, num_ticks: usize) -> usize {
        let parsed_rules: HashMap<(char, char), [(char, char); 2]> = self
            .rules
            .iter()
            .map(|(key, value)| {
                let &(a, b) = key;
                let &c = value;
                (*key, [(a, c), (c, b)])
            })
            .collect();

        let mut state: HashMap<(char, char), usize> = HashMap::new();
        for i in 1..self.chain.len() {
            let prev = self.chain[i - 1];
            let now = self.chain[i];
            *state.entry((prev, now)).or_insert(0) += 1;
        }
        for _ in 0..num_ticks {
            let mut new_state: HashMap<(char, char), usize> = HashMap::new();
            for (key, value) in state.iter() {
                if let Some(rule) = parsed_rules.get(key) {
                    let &[(a, b), (c, d)] = rule;
                    *new_state.entry((a, b)).or_insert(0) += *value;
                    *new_state.entry((c, d)).or_insert(0) += *value;
                }
            }
            state = new_state;
        }
        let mut count: HashMap<char, usize> = HashMap::new();
        for (key, value) in state.iter() {
            let &(a, b) = key;
            *count.entry(a).or_insert(0) += *value;
            *count.entry(b).or_insert(0) += *value;
        }
        count.entry(self.chain[0]).and_modify(|num| *num += 1);
        count
            .entry(self.chain[self.chain.len() - 1])
            .and_modify(|num| *num += 1);
        let count: Vec<usize> = count.into_values().collect();
        (count.iter().max().unwrap() - count.iter().min().unwrap()) / 2
    }

    fn part1(&self) -> usize {
        let mut count: HashMap<char, usize> = HashMap::new();
        for c in self.chain.iter() {
            *count.entry(*c).or_insert(0) += 1;
        }
        let count: Vec<usize> = count.into_values().collect();
        count.iter().max().unwrap() - count.iter().min().unwrap()
    }
}

pub fn solve() {
    let input = fs::read_to_string("src/inputs/day14.txt").expect("No input provided");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut polymer = Polymer::new(input);
    for _ in 0..10 {
        polymer.tick();
    }
    polymer.part1()
}

fn part2(input: &str) -> usize {
    let polymer = Polymer::new(input);
    polymer.faster_tick(40)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        fs::read_to_string("src/samples/day14.txt").expect("No input provided")
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&get_input()), 1588);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&get_input()), 2188189693529);
    }
}
