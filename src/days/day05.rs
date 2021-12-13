use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point(usize, usize);

struct Line(Point, Point);

enum LineType {
    Horizontal,
    Vertical,
    IncreasingDiagonal,
    DecreasingDiagonal,
}

impl Line {
    fn is_valid(&self) -> Option<LineType> {
        if self.is_horizontal() {
            Some(LineType::Horizontal)
        } else if self.is_vertical() {
            Some(LineType::Vertical)
        } else {
            self.is_diagonal()
        }
    }
    fn is_horizontal(&self) -> bool {
        self.0 .0 == self.1 .0
    }
    fn is_vertical(&self) -> bool {
        self.0 .1 == self.1 .1
    }
    fn is_diagonal(&self) -> Option<LineType> {
        let (x1, y1) = (self.0 .0 as i32, self.0 .1 as i32);
        let (x2, y2) = (self.1 .0 as i32, self.1 .1 as i32);
        let dx = x2 - x1;
        let dy = y2 - y1;
        if dx == dy {
            Some(LineType::IncreasingDiagonal)
        } else if dx == -dy {
            Some(LineType::DecreasingDiagonal)
        } else {
            None
        }
    }
    fn normalized(&self) -> Line {
        if let Some(line_type) = self.is_valid() {
            match line_type {
                LineType::Vertical | LineType::DecreasingDiagonal => {
                    if self.0 .0 > self.1 .0 {
                        Line(self.1, self.0)
                    } else {
                        Line(self.0, self.1)
                    }
                }
                _ => {
                    if self.0 .1 > self.1 .1 {
                        Line(self.1, self.0)
                    } else {
                        Line(self.0, self.1)
                    }
                }
            }
        } else {
            Line(self.0, self.1)
        }
    }
}

impl FromStr for Point {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(",");
        let x = iter.next().unwrap().trim().parse().unwrap();
        let y = iter.next().unwrap().trim().parse().unwrap();
        Ok(Point(x, y))
    }
}

impl FromStr for Line {
    type Err = ();
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut iter = line.split("->");
        let first = iter.next().unwrap().parse().unwrap();
        let second = iter.next().unwrap().parse().unwrap();
        Ok(Line(first, second).normalized())
    }
}

struct Map {
    points: HashMap<Point, usize>,
    part1: bool,
}

impl Map {
    fn new(part1: bool) -> Map {
        Map {
            points: HashMap::new(),
            part1,
        }
    }

    fn add_line(&mut self, line: &Line) {
        let Line(first, second) = line;
        if let Some(line_type) = line.is_valid() {
            match line_type {
                LineType::Horizontal => {
                    for y in first.1..=second.1 {
                        let count: usize = if let Some(count) = self.points.get(&Point(first.0, y))
                        {
                            *count
                        } else {
                            0
                        } + 1;
                        self.points.insert(Point(first.0, y), count);
                    }
                }
                LineType::Vertical => {
                    for x in first.0..=second.0 {
                        let count: usize = if let Some(count) = self.points.get(&Point(x, first.1))
                        {
                            *count
                        } else {
                            0
                        } + 1;
                        self.points.insert(Point(x, first.1), count);
                    }
                }
                LineType::IncreasingDiagonal => {
                    if self.part1 {
                        return;
                    }
                    let mut x = first.0;
                    let mut y = first.1;
                    while x <= second.0 && y <= second.1 {
                        let count: usize = if let Some(count) = self.points.get(&Point(x, y)) {
                            *count
                        } else {
                            0
                        } + 1;
                        self.points.insert(Point(x, y), count);
                        x += 1;
                        y += 1;
                    }
                }
                LineType::DecreasingDiagonal => {
                    if self.part1 {
                        return;
                    }
                    let mut x = first.0;
                    let mut y = first.1;
                    while x <= second.0 && y >= second.1 {
                        let count: usize = if let Some(count) = self.points.get(&Point(x, y)) {
                            *count
                        } else {
                            0
                        } + 1;
                        self.points.insert(Point(x, y), count);
                        x += 1;
                        if y > 0 {
                            y -= 1;
                        }
                    }
                }
            }
        }
    }
}

pub fn solve() {
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    writeln!(output, "{}", part1(&lines)).unwrap();
    writeln!(output, "{}", part2(&lines)).unwrap();
}

fn part1(lines: &[&str]) -> u16 {
    let mut map = Map::new(true);
    for line in lines {
        let line = line.parse::<Line>().unwrap();
        map.add_line(&line);
    }
    let mut count = 0;
    for (_, val) in map.points.iter() {
        if *val > 1 {
            count += 1;
        }
    }
    count
}

fn part2(lines: &[&str]) -> u16 {
    let mut map = Map::new(false);
    for line in lines {
        let line = line.parse::<Line>().unwrap();
        map.add_line(&line);
    }
    let mut count = 0;
    for (_, val) in map.points.iter() {
        if *val > 1 {
            count += 1;
        }
    }
    count
}
