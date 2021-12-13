use std::fs;

struct Point {
    r: usize,
    c: usize,
}

struct Octopuses {
    octopuses: [[u8; 10]; 10],
}

impl Octopuses {
    fn new(input: &str) -> Octopuses {
        let mut octopuses = Octopuses {
            octopuses: [[0; 10]; 10],
        };
        input.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                octopuses.octopuses[row][col] = c.to_digit(10).unwrap() as u8;
            });
        });
        octopuses
    }

    fn part2(&mut self) -> usize {
        let mut ind = 0;
        loop {
            ind += 1;
            if self.tick() == 100 {
                return ind;
            }
        }
    }

    fn part1(&mut self) -> usize {
        (0..100).map(|_| self.tick()).sum()
    }

    fn tick(&mut self) -> usize {
        let mut ans = 0;
        for r in 0..10 {
            for c in 0..10 {
                self.dfs(Point { r, c })
            }
        }
        for r in 0..10 {
            for c in 0..10 {
                if self.octopuses[r][c] > 9 {
                    self.octopuses[r][c] = 0;
                    ans += 1;
                }
            }
        }
        ans
    }

    fn dfs(&mut self, start: Point) {
        self.octopuses[start.r][start.c] += 1;
        let num = self.octopuses[start.r][start.c];
        if num == 10 {
            for neighbor in neighbors(&start) {
                self.dfs(neighbor)
            }
        }
    }
}
fn neighbors(point: &Point) -> Vec<Point> {
    let mut neighbors = vec![];
    let row_start = if point.r == 0 { 0 } else { point.r - 1 };
    let row_end = if point.r == 9 { 9 } else { point.r + 1 };
    let col_start = if point.c == 0 { 0 } else { point.c - 1 };
    let col_end = if point.c == 9 { 9 } else { point.c + 1 };
    for r in row_start..=row_end {
        for c in col_start..=col_end {
            if r == point.r && c == point.c {
                continue;
            }
            neighbors.push(Point { r, c });
        }
    }
    neighbors
}

pub fn solve() {
    let input = fs::read_to_string("src/inputs/day11.txt").expect("No input provided");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut octopuses = Octopuses::new(input);
    octopuses.part1()
}

fn part2(input: &str) -> usize {
    let mut octopuses = Octopuses::new(input);
    octopuses.part2()
}
#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        fs::read_to_string("src/samples/day11.txt").expect("No input provided")
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&get_input()), 1656);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(&get_input()), 195);
    }
}
