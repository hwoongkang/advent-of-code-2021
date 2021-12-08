use std::collections::{HashMap, HashSet};
use std::fs;

pub fn solve() {
    let input = fs::read_to_string("src/days/input.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split("|").skip(1).next().unwrap().trim())
        .map(|part| {
            part.split_whitespace()
                .filter(|word| {
                    let length = word.len();
                    length == 2 || length == 3 || length == 4 || length == 7
                })
                .count()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input.lines().map(|line| solve_for_line(line)).sum()
}

fn solve_for_line(line: &str) -> usize {
    let mut parts = line.split("|");
    let ten_digits: Vec<&str> = parts.next().unwrap().trim().split_whitespace().collect();
    let four_digits: Vec<&str> = parts.next().unwrap().trim().split_whitespace().collect();

    let (segments, numbers) = construct_dicts(&ten_digits);

    four_digits
        .iter()
        .rev()
        .enumerate()
        .map(|(ind, word)| {
            let parsed = parse_word(word, &segments, &numbers);
            10usize.pow(ind as u32) * parsed
        })
        .sum()
}

fn construct_dicts(ten_digits: &[&str]) -> (HashMap<char, usize>, HashMap<usize, usize>) {
    let mut numbers = HashMap::<usize, usize>::new();
    let mut segments: HashMap<char, usize> = HashMap::new();

    let one: HashSet<char> = ten_digits
        .iter()
        .filter(|word| word.len() == 2)
        .next()
        .unwrap()
        .chars()
        .collect();
    let four: HashSet<char> = ten_digits
        .iter()
        .filter(|word| word.len() == 4)
        .next()
        .unwrap()
        .chars()
        .collect();
    let seven: HashSet<char> = ten_digits
        .iter()
        .filter(|word| word.len() == 3)
        .next()
        .unwrap()
        .chars()
        .collect();
    let eight: HashSet<char> = ten_digits
        .iter()
        .filter(|word| word.len() == 7)
        .next()
        .unwrap()
        .chars()
        .collect();
    let zero_six_nine: Vec<HashSet<char>> = ten_digits
        .iter()
        .filter(|word| word.len() == 6)
        .map(|word| word.chars().collect())
        .collect();
    let two_three_five: Vec<HashSet<char>> = ten_digits
        .iter()
        .filter(|word| word.len() == 5)
        .map(|word| word.chars().collect())
        .collect();
    let all: HashSet<char> = ['a', 'b', 'c', 'd', 'e', 'f', 'g']
        .iter()
        .map(|x| *x)
        .collect();
    let a = seven.difference(&one).next().unwrap();
    let horz = two_three_five.iter().fold(all.clone(), |acc, x| {
        x.intersection(&acc).map(|x| *x).collect()
    });
    let d = four.intersection(&horz).next().unwrap();
    let mut bd: HashSet<char> = four.difference(&one).map(|x| *x).collect();
    bd.remove(d);
    let b = bd.iter().next().unwrap();

    let five = two_three_five
        .iter()
        .filter(|word| word.contains(&b))
        .next()
        .unwrap();
    let f = five
        .difference(&horz)
        .filter_map(|x| if x == b { None } else { Some(x) })
        .next()
        .unwrap();

    let c = seven.difference(&five).next().unwrap();
    //abcdf, horz, five
    let ag = five
        .difference(&four)
        .map(|x| *x)
        .collect::<HashSet<char>>();
    let g = ag.difference(&seven).next().unwrap();
    //abcdfg, horz, five
    let ce = eight
        .difference(&five)
        .map(|x| *x)
        .collect::<HashSet<char>>();

    let e = ce.difference(&one).next().unwrap();
    let A = 2;
    let B = 3;
    let C = 5;
    let D = 7;
    let E = 11;
    let F = 13;
    let G = 17;
    segments.insert(*a, A);
    segments.insert(*b, B);
    segments.insert(*c, C);
    segments.insert(*d, D);
    segments.insert(*e, E);
    segments.insert(*f, F);
    segments.insert(*g, G);

    numbers.insert(A * B * C * E * F * G, 0);
    numbers.insert(C * F, 1);
    numbers.insert(A * C * D * E * G, 2);
    numbers.insert(A * C * D * F * G, 3);
    numbers.insert(B * C * D * F, 4);
    numbers.insert(A * B * D * F * G, 5);
    numbers.insert(A * B * D * E * F * G, 6);
    numbers.insert(A * C * F, 7);
    numbers.insert(A * B * C * D * E * F * G, 8);
    numbers.insert(A * B * C * D * F * G, 9);

    (segments, numbers)
}

fn word_to_prime(word: &str, dictionary: &HashMap<char, usize>) -> usize {
    word.chars()
        .map(|ch| dictionary[&ch])
        .fold(1, |acc, x| acc * x)
}

fn parse_word(
    word: &str,
    segments: &HashMap<char, usize>,
    numbers: &HashMap<usize, usize>,
) -> usize {
    let key = word_to_prime(word, segments);
    numbers[&key]
}
