use std::collections::{HashMap, HashSet};
use std::fs;

struct Cave {
    is_small: bool,
    neighbors: Vec<String>,
}

impl Cave {
    fn new(name: &str) -> Self {
        let is_small = name
            .chars()
            .map(|ch| ch.is_ascii_lowercase())
            .fold(true, |acc, x| acc && x);
        Cave {
            neighbors: Vec::new(),
            is_small,
        }
    }
}

struct CaveMap {
    caves: HashMap<String, Cave>,
}

impl CaveMap {
    fn new(input: &str) -> Self {
        let mut caves = HashMap::new();
        input.lines().for_each(|line| {
            let mut names = line.split("-");
            let name1 = names.next().unwrap().to_string();
            let name2 = names.next().unwrap().to_string();
            if caves.get(&name1).is_none() {
                caves.insert(name1.clone(), Cave::new(&name1));
            }
            if caves.get(&name2).is_none() {
                caves.insert(name2.clone(), Cave::new(&name2));
            }
            caves.get_mut(&name1).unwrap().neighbors.push(name2.clone());
            caves.get_mut(&name2).unwrap().neighbors.push(name1.clone());
        });
        CaveMap { caves }
    }
    fn part1(&self) -> usize {
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert("start".to_string());
        self.dfs("start", &mut visited)
    }
    fn part2(&self) -> usize {
        let mut visited: HashSet<String> = HashSet::new();
        let mut consumed = false;
        self.dfs_2("start", &mut visited, &mut consumed)
    }
    fn dfs(&self, start: &str, visited: &mut HashSet<String>) -> usize {
        let mut count = 0;
        for neighbor in self.caves[start].neighbors.iter() {
            if neighbor == "end" {
                count += 1;
                continue;
            }
            if visited.contains(neighbor) {
                continue;
            }
            if self.caves[neighbor].is_small {
                visited.insert(neighbor.clone());
            }
            count += self.dfs(neighbor, visited);
        }

        if start != "start" {
            visited.remove(start);
        }
        count
    }

    fn dfs_2(&self, start: &str, visited: &mut HashSet<String>, consumed: &mut bool) -> usize {
        let mut count = 0;
        let mut flag = true;
        if visited.contains(start) {
            if *consumed {
                return 0;
            } else {
                flag = false;
                *consumed = true;
            }
        }
        if self.caves[start].is_small {
            visited.insert(start.to_string());
        }

        for neighbor in self.caves[start].neighbors.iter() {
            if neighbor == "end" {
                count += 1;
                continue;
            }
            if neighbor == "start" {
                continue;
            }
            count += self.dfs_2(neighbor, visited, consumed);
        }

        if flag {
            visited.remove(start);
        } else {
            *consumed = false;
        }
        count
    }
}

pub fn solve() {
    let input = fs::read_to_string("src/inputs/day12.txt").expect("No input provided");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> usize {
    let cave_map = CaveMap::new(input);
    cave_map.part1()
}

fn part2(input: &str) -> usize {
    let cave_map = CaveMap::new(input);
    cave_map.part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input_1 = fs::read_to_string("src/samples/day12-1.txt").expect("No input provided");
        let input_2 = fs::read_to_string("src/samples/day12-2.txt").expect("No input provided");
        let input_3 = fs::read_to_string("src/samples/day12-3.txt").expect("No input provided");

        assert_eq!(super::part1(&input_1), 10);
        assert_eq!(super::part1(&input_2), 19);
        assert_eq!(super::part1(&input_3), 226);
    }
    #[test]
    fn part2() {
        let input_1 = fs::read_to_string("src/samples/day12-1.txt").expect("No input provided");
        let input_2 = fs::read_to_string("src/samples/day12-2.txt").expect("No input provided");
        let input_3 = fs::read_to_string("src/samples/day12-3.txt").expect("No input provided");

        assert_eq!(super::part2(&input_1), 36);
        assert_eq!(super::part2(&input_2), 103);
        assert_eq!(super::part2(&input_3), 3509);
    }
}
