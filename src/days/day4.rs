use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, Read, Write};

#[derive(Eq, PartialEq)]
struct BingoResult(usize, u32);

impl Ord for BingoResult {
    fn cmp(&self, other: &BingoResult) -> Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Equal => other.1.cmp(&self.1),
            other => other,
        }
    }
}

impl PartialOrd for BingoResult {
    fn partial_cmp(&self, other: &BingoResult) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Bingo {
    board: Vec<Vec<u32>>,
    checked: Vec<Vec<bool>>,
    indice: HashMap<u32, (usize, usize)>,
    won: bool,
}

impl Bingo {
    fn new(lines: &mut std::slice::Iter<&str>) -> Bingo {
        let mut board: Vec<Vec<u32>> = vec![];
        let mut indice: HashMap<u32, (usize, usize)> = HashMap::new();
        for row in 0..5 {
            board.push(
                lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .enumerate()
                    .map(|(col, word)| {
                        let num: u32 = word.parse().unwrap();
                        indice.insert(num, (row, col));
                        num
                    })
                    .collect(),
            );
        }

        Bingo {
            board,
            checked: (0..5).map(|_| vec![false; 5]).collect(),
            indice,
            won: false,
        }
    }

    fn cross(&mut self, num: u32) -> Option<u32> {
        if self.won {
            return None;
        }
        if let Some(&(row, col)) = self.indice.get(&num) {
            self.checked[row][col] = true;
            if self.check() {
                self.won = true;
                return Some(self.point() * num);
            }
        }
        None
    }

    fn check(&self) -> bool {
        for index in 0..5 {
            if self.checked[index].iter().filter(|&&b| b).count() == 5 {
                return true;
            }
            if self.checked.iter().filter(|row| row[index]).count() == 5 {
                return true;
            }
        }
        false
    }

    fn point(&self) -> u32 {
        let mut ans = 0;
        for r in 0..5 {
            for c in 0..5 {
                if !self.checked[r][c] {
                    ans += self.board[r][c];
                }
            }
        }
        ans
    }

    fn simulate(&mut self, inputs: &[u32]) -> BingoResult {
        for (ind, num) in inputs.iter().enumerate() {
            if let Some(point) = self.cross(*num) {
                return BingoResult(ind, point);
            }
        }
        BingoResult(0, 0)
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

fn part1(lines: &[&str]) -> u32 {
    let mut lines = lines.iter();
    let input: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|word| word.parse().unwrap())
        .collect();
    let mut bingos: Vec<Bingo> = vec![];
    while let Some(_) = lines.next() {
        bingos.push(Bingo::new(&mut lines));
    }
    for num in input {
        if let Some(point) = bingos
            .iter_mut()
            .filter_map(|bingo| bingo.cross(num))
            .max_by(|x, y| x.cmp(y))
        {
            return point;
        }
    }
    0
}

fn part2(lines: &[&str]) -> u32 {
    let mut lines = lines.iter();
    let input: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|word| word.parse().unwrap())
        .collect();
    let mut bingos: Vec<Bingo> = vec![];
    while let Some(_) = lines.next() {
        bingos.push(Bingo::new(&mut lines));
    }
    let results: BinaryHeap<BingoResult> = bingos
        .iter_mut()
        .map(|bingo| bingo.simulate(&input))
        .collect();
    results.peek().unwrap().1
}
