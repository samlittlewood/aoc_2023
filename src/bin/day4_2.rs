use std::{fs, collections::HashSet};

#[derive(Debug)]
struct Card {
    wins: u32,
    copies: u32,
}

fn main() {
    println!("Hello, Day 4 2!");
    let mut total = 0;

    let mut cards: Vec<Card> = vec![];

    // For each card, figure wins
    for l in  fs::read_to_string("inputs/day4").unwrap().lines() {
        let id_rest = l.split(':').collect::<Vec<_>>();
        let numbers = id_rest[1].split('|').collect::<Vec<_>>();
        let win_numbers = numbers[0].trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<HashSet<_>>();
        let have_numbers = numbers[1].trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<HashSet<_>>();

        let wins = win_numbers.intersection(&have_numbers).count() as u32;

        cards.push(Card{wins, copies:1});
    }

    for i in 0..cards.len() {
        for w in 0..cards[i].wins {
            let idx = i+1+(w as usize);
            if idx < cards.len() {
                cards[idx].copies += cards[i].copies;
            }
        }
        total += cards[i].copies;
    }

    println!("Total {}", total);
}
