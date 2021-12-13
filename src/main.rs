use std::env;
mod days;
fn main() {
    let day: u8 = env::args()
        .nth(1)
        .expect("Please provide a day")
        .parse()
        .expect("Please provide a valid day");
    println!("{}", day);

    match day {
        1 => days::day01::solve(),
        2 => days::day02::solve(),
        3 => days::day03::solve(),
        4 => days::day04::solve(),
        5 => days::day05::solve(),
        6 => days::day06::solve(),
        7 => days::day07::solve(),
        8 => days::day08::solve(),
        9 => days::day09::solve(),
        10 => days::day10::solve(),
        11 => days::day11::solve(),
        12 => days::day12::solve(),
        _ => println!("No solution for day {}", day),
    }
}
