use std::fs;
pub fn solve() {
    let input = fs::read_to_string("inputs/day1.txt").expect("Input not provided");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> usize {
    let depths: Vec<u16> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut prev = u16::MAX;
    let mut ans: usize = 0;
    for &depth in depths.iter() {
        if depth > prev {
            ans += 1;
        }
        prev = depth;
    }
    ans
}

fn part2(input: &str) -> usize {
    let depths: Vec<u16> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut ans = 0;
    let mut prev = u16::MAX;

    for ind in 0..depths.len() - 2 {
        let sum = depths[ind] + depths[ind + 1] + depths[ind + 2];
        if sum > prev {
            ans += 1;
        }
        prev = sum;
    }

    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        use super::*;
        let input = fs::read_to_string("src/samples/day1.txt").expect("Input not provided");
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn part2() {
        use super::*;
        let input = fs::read_to_string("src/samples/day1.txt").expect("Input not provided");
        assert_eq!(part2(&input), 5);
    }
}
