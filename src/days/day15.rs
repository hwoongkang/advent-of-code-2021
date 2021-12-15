use std::collections::BinaryHeap;
use std::fs;

#[derive(Eq, PartialEq)]
struct State {
    point: Point,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Point(usize, usize);

struct Map {
    risk_level: Vec<Vec<usize>>,
    dest: Point,
}

impl Map {
    fn new(input: &str) -> Self {
        let risk_level: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        Map {
            dest: Point(risk_level.len() - 1, risk_level[0].len() - 1),
            risk_level,
        }
    }

    fn part2_new(input: &str) -> Self {
        let risk_level_temp: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        let mut risk_level: Vec<Vec<usize>> = vec![];

        for down in 0..5 {
            for row in risk_level_temp.iter() {
                let new_row: Vec<usize> = (0..5)
                    .map(|right| {
                        let plus = down + right;
                        let temp: Vec<usize> = row
                            .iter()
                            .map(|prev| {
                                let mut new_num = prev + plus;
                                if new_num > 9 {
                                    new_num = new_num % 9;
                                }
                                new_num
                            })
                            .collect();
                        temp
                    })
                    .flatten()
                    .collect();
                risk_level.push(new_row);
            }
        }
        Map {
            dest: Point(risk_level.len() - 1, risk_level[0].len() - 1),
            risk_level,
        }
    }

    fn part1(&self) -> usize {
        let mut heap: BinaryHeap<State> = BinaryHeap::new();
        let mut previous_risk: Vec<Vec<usize>> = self
            .risk_level
            .iter()
            .map(|row| row.iter().map(|_| usize::MAX).collect())
            .collect();
        heap.push(State {
            point: Point(0, 0),
            cost: 0,
        });
        previous_risk[0][0] = self.risk_level[0][0];
        while let Some(State { point, cost }) = heap.pop() {
            if point == self.dest {
                return cost;
            }
            for neighbor in self.neighbors(&point) {
                let prev_cost = previous_risk[neighbor.0][neighbor.1];
                let new_cost = cost + self.risk_level[neighbor.0][neighbor.1];
                if prev_cost > new_cost {
                    previous_risk[neighbor.0][neighbor.1] = new_cost;
                    heap.push(State {
                        point: neighbor,
                        cost: new_cost,
                    });
                }
            }
        }
        0
    }

    fn neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = vec![];
        if point.0 > 0 {
            neighbors.push(Point(point.0 - 1, point.1));
        }
        if point.0 < self.dest.0 {
            neighbors.push(Point(point.0 + 1, point.1));
        }
        if point.1 > 0 {
            neighbors.push(Point(point.0, point.1 - 1));
        }
        if point.1 < self.dest.1 {
            neighbors.push(Point(point.0, point.1 + 1));
        }
        neighbors
    }
}

pub fn solve() {
    let input = fs::read_to_string("src/inputs/day15.txt").expect("No input provided");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> usize {
    let map = Map::new(input);
    map.part1()
}

fn part2(input: &str) -> usize {
    let map = Map::part2_new(input);
    map.part1()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        fs::read_to_string("src/samples/day15.txt").expect("No input provided")
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&get_input()), 40);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&get_input()), 315);
    }
}
