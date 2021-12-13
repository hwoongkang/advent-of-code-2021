use std::fs;

#[derive(Copy, Clone)]
struct Point(usize, usize);

#[derive(Debug)]
struct Paper {
    width: usize,
    height: usize,
    dots: Vec<Vec<bool>>,
}

impl Paper {
    fn new(input: &mut std::str::Lines) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut points: Vec<Point> = vec![];
        loop {
            let line = input.next().unwrap();
            if line.len() == 0 {
                break;
            }
            let mut nums = line.split(",");
            let x: usize = nums.next().unwrap().parse().unwrap();
            let y: usize = nums.next().unwrap().parse().unwrap();
            width = if x > width { x } else { width };
            height = if y > height { y } else { height };
            points.push(Point(x, y));
        }
        let mut dots = vec![vec![false; height + 1]; width + 1];
        points.iter().for_each(|Point(x, y)| {
            dots[*x][*y] = true;
        });
        Paper {
            width,
            height,
            dots,
        }
    }
    fn fold(&mut self, current_size: &Point, fold_along: &Point) -> Point {
        let &Point(x2, y2) = current_size;
        let &Point(x0, y0) = fold_along;
        if x0 == 0 {
            for dy in 1..=y0 {
                for x in 0..=x2 {
                    let from = y0 + dy;
                    let to = y0 - dy;
                    self.dots[x][to] = self.dots[x][to] || self.dots[x][from];
                    self.dots[x][from] = false;
                }
            }
            Point(x2, y0)
        } else {
            for dx in 1..=x0 {
                for y in 0..=y2 {
                    let from = x0 + dx;
                    let to = x0 - dx;
                    self.dots[to][y] = self.dots[to][y] || self.dots[from][y];
                    self.dots[from][y] = false;
                }
            }
            Point(x0, y2)
        }
    }
    fn count(&self) -> usize {
        self.dots
            .iter()
            .map(|row| {
                let row_sum: usize = row.iter().map(|&dot| if dot { 1 } else { 0 }).sum();
                row_sum
            })
            .sum()
    }
    fn print(&self, size: Point) {
        (0..size.1).for_each(|col| {
            (0..size.0).for_each(|row| {
                print!("{}", if self.dots[row][col] { "#" } else { "." });
            });
            println!("");
        });
    }
}

pub fn solve() {
    let input = fs::read_to_string("src/inputs/day13.txt").expect("No input provided");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut paper = Paper::new(&mut lines);

    let fold_along = parse_command(lines.next().unwrap());
    let size = Point(paper.width, paper.height);

    paper.fold(&size, &fold_along);
    paper.count()
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let mut paper = Paper::new(&mut lines);
    let mut size = Point(paper.width, paper.height);
    while let Some(line) = lines.next() {
        let fold_along = parse_command(line);
        size = paper.fold(&size, &fold_along);
    }
    paper.print(size);
    0
}

fn parse_command(line: &str) -> Point {
    let mut command = line.split_whitespace().skip(2).next().unwrap().split("=");
    let is_x = command.next().unwrap() == "x";
    let value: usize = command.next().unwrap().parse().unwrap();
    if is_x {
        Point(value, 0)
    } else {
        Point(0, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = fs::read_to_string("src/samples/day13.txt").expect("No input provided");
        let mut lines = input.lines();
        let paper = Paper::new(&mut lines);
        println!("{:?}", paper);
        assert_eq!(super::part1(&input), 17);
    }
}
