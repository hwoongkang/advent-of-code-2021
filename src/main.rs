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
        1 => days::day1::solve(),
        2 => days::day2::solve(),
        3 => days::day3::solve(),
        4 => days::day4::solve(),
        5 => days::day5::solve(),
        6 => days::day6::solve(),
        7 => days::day7::solve(),
        8 => days::day8::solve(),
        9 => days::day9::solve(),
        _ => println!("No solution for day {}", day),
    }
}
