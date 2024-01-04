use std::fs;

fn main() {
    println!("Hello, Day 1!");
    let mut total  = 0;
    for (i,l) in fs::read_to_string("inputs/day1").unwrap().lines().enumerate() {
        let digits = l.chars().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
        let n = digits.first().unwrap() * 10 + digits.last().unwrap();
        println!(" {} {} {:?}", i,l, n);
        total += n;
    }
    println!("Total {}", total);
}
