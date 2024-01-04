use std::fs;
use regex::Regex;

fn digit_from_string(s :&str) -> u32 {
    match s {
        "0" | "zero" | "orez" => 0,
        "1" | "one" | "eno" => 1,
        "2" | "two" | "owt" => 2,
        "3" | "three" | "eerht" => 3,
        "4" | "four" | "ruof" => 4,
        "5" | "five" | "evif" => 5,
        "6" | "six" | "xis" => 6,
        "7" | "seven" | "neves" => 7,
        "8" | "eight" | "thgie" => 8,
        "9" | "nine" | "enin" => 9,
        _ => 0
    }
}

fn main() {
    println!("Day 1_2");
    let mut total  = 0;
    let re_f = Regex::new("[0-9]|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_b = Regex::new("[0-9]|orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    for l in fs::read_to_string("inputs/day1").unwrap().lines() {
        let digits_f = re_f.find_iter(l).map(|m| digit_from_string(m.as_str())).collect::<Vec<_>>();
        let digits_b = re_b.find_iter(&l.chars().rev().collect::<String>()).map(|m| digit_from_string(m.as_str())).collect::<Vec<_>>();

        let n = digits_f.first().unwrap() * 10 + digits_b.first().unwrap();
        total += n;
    }
    println!("Total {}", total);
}
