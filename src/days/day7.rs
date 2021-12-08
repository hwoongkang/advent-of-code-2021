use std::fs;

pub fn solve() {
    let input = fs::read_to_string("src/days/input.txt").unwrap();
    let positions: Vec<i32> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{}", part1(&positions));
    println!("{}", part2(&positions));
}

fn part1(lines: &[i32]) -> i32 {
    let mut sorted: Vec<i32> = lines.to_vec();
    sorted.sort();
    let length = sorted.len();
    let median = sorted[length / 2];
    sorted
        .iter()
        .map(|&x| if x > median { x - median } else { median - x })
        .sum()
}

fn part2(nums: &[i32]) -> i32 {
    gradient_descent(&nums);
    (481..484).map(|x| cost(x, &nums)).min().unwrap()
}

fn gradient_descent(nums: &[i32]) -> i32 {
    let mut ans = 0;
    let mut count = 2000;
    while count > 0 {
        let mut grad = gradient(ans, &nums);
        println!("grad was: {}", grad);
        count -= 1;
        if grad.abs() < 100 {
            println!("{}, {}", ans, grad);
        }
        while grad.abs() > 10 {
            grad /= 10;
        }
        if grad > 0 {
            grad = 1;
        } else {
            grad = -1;
        }
        println!("moving towards {}, {}", -grad, ans);
        ans -= grad;
    }
    println!(
        "{} , {}, {}",
        nums.iter().max().unwrap(),
        nums.iter().min().unwrap(),
        ans
    );
    ans
}

fn gradient(x: i32, nums: &[i32]) -> i32 {
    nums.iter()
        .map(|&y| 2 * x - 2 * y + if x > y { 1 } else { -1 })
        .sum()
}

fn cost(x: i32, nums: &[i32]) -> i32 {
    nums.iter()
        .map(|&y| {
            let diff = if x > y { x - y } else { y - x };
            diff * (diff + 1) / 2
        })
        .sum()
}
