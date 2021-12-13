use std::fs;

#[derive(Copy, Clone)]
struct Point(usize, usize);

struct HeightMap {
    points: Vec<Vec<u32>>,
    size: Point,
}

impl HeightMap {
    fn new(input: &str) -> Self {
        let points: Vec<Vec<u32>> = input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();

        Self {
            size: Point(points.len(), points[0].len()),
            points,
        }
    }

    fn part1(&self) -> u32 {
        let mut ans = 0;
        for r in 0..self.size.0 {
            for c in 0..self.size.1 {
                if let Some(value) = self.is_local_minimum(&Point(r, c)) {
                    ans += value;
                }
            }
        }
        ans
    }

    fn part2(&mut self) -> usize {
        let mut sizes: Vec<usize> = self
            .minimum_points()
            .iter()
            .map(|point| self.dfs(point))
            .collect();
        sizes.sort_by(|a, b| b.cmp(a));
        sizes
            .iter()
            .zip(0..3)
            .map(|(count, _)| count)
            .fold(1, |acc, &x| acc * x)
    }

    fn dfs(&mut self, point: &Point) -> usize {
        let Point(x, y) = *point;
        if self.points[x][y] == 10 {
            return 0;
        }
        self.points[x][y] = 10;
        let nested: usize = self
            .dfs_adjacent(point)
            .iter()
            .map(|next_point| self.dfs(next_point))
            .sum();
        nested + 1
    }

    fn dfs_adjacent(&self, point: &Point) -> Vec<Point> {
        adjacent_cells(point, &self.size)
            .iter()
            .filter(|Point(x, y)| self.points[*x][*y] != 9)
            .map(|p| *p)
            .collect()
    }

    fn minimum_points(&self) -> Vec<Point> {
        let mut ans = Vec::new();
        for r in 0..self.size.0 {
            for c in 0..self.size.1 {
                let now = Point(r, c);
                if let Some(_) = self.is_local_minimum(&now) {
                    ans.push(now);
                }
            }
        }
        ans
    }

    fn is_local_minimum(&self, point: &Point) -> Option<u32> {
        let Point(x, y) = *point;
        let now = self.points[x][y];
        for Point(nx, ny) in adjacent_cells(point, &self.size) {
            if self.points[nx][ny] <= now {
                return None;
            }
        }
        Some(now + 1)
    }
}

fn adjacent_cells(now: &Point, size: &Point) -> Vec<Point> {
    let mut neighbors = vec![];
    if now.0 > 0 {
        neighbors.push(Point(now.0 - 1, now.1));
    }
    if now.1 > 0 {
        neighbors.push(Point(now.0, now.1 - 1));
    }
    if now.0 < size.0 - 1 {
        neighbors.push(Point(now.0 + 1, now.1));
    }
    if now.1 < size.1 - 1 {
        neighbors.push(Point(now.0, now.1 + 1));
    }
    neighbors
}

pub fn solve() {
    let input = fs::read_to_string("src/inputs/day9.txt").expect("No input provided");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let height_map = HeightMap::new(input);
    height_map.part1()
}

fn part2(input: &str) -> usize {
    let mut height_map = HeightMap::new(input);
    height_map.part2()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        use super::*;
        let input = fs::read_to_string("src/samples/day9.txt").expect("No sample input provided.");
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn part2() {
        use super::*;
        let input = fs::read_to_string("src/samples/day9.txt").expect("No sample input provided.");
        assert_eq!(part2(&input), 1134);
    }
}
