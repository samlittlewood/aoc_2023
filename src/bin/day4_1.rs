use std::{fs, collections::HashSet};

fn main() {
    println!("Hello, Day 4 1!");
    let mut total = 0;

    // For each card
    for l in  fs::read_to_string("inputs/day4").unwrap().lines() {
        let id_rest = l.split(':').collect::<Vec<_>>();
        let _id = id_rest[0][5..].trim().parse::<u32>().unwrap();
        let numbers = id_rest[1].split('|').collect::<Vec<_>>();
        let win_numbers = numbers[0].trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<HashSet<_>>();
        let have_numbers = numbers[1].trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<HashSet<_>>();

        let winning = win_numbers.intersection(&have_numbers).collect::<Vec<_>>();
        let points = if winning.is_empty() { 0 } else { 2u32.pow((winning.len()-1) as u32) };
        total += points;
//        println!("{id} {numbers:?} {win_numbers:?} {have_numbers:?} {winning:?} {points}");
    }


    println!("Total {}", total);
}
