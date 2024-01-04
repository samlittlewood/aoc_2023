use std::{fs, collections::HashMap};
use itertools::Itertools;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Card {
    None,
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

fn card_from_char(c: char) -> Card{
    match c {
        'J' => Card::Joker,
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => Card::None
    }
}

// Can the given counts be satisfied by cards & jokers?
//
// cards must be in ascending count order (largest last)
// counts must be in descending order (largest first)
fn try_type(cards: &[usize], jokers: usize, counts:&[usize]) -> bool
{
    let mut remaining_cards = cards.len();
    let mut remaining_jokers = jokers;

    for &c in counts {
        if remaining_cards > 0 {
            // Some non jokers left - can we take enough cards to make count?
            remaining_cards -= 1;
            if cards[remaining_cards] + remaining_jokers >= c {
                remaining_jokers -= usize::min(c - cards[remaining_cards], remaining_jokers);
            } else {
                return false;
            }
        } else {
            // Only jokers
            if remaining_jokers >= c {
                remaining_jokers -= c;
            } else {
                return false;
            }
        }
    }

    return true;
}

fn hand_type(hand: &[Card]) -> HandType
{
    assert_eq!(hand.len(), 5);

    let mut card_counts  = HashMap::new();
    let mut jokers = 0;
    for card in hand {
        if *card == Card::Joker {
            jokers += 1;
        } else {
            *card_counts.entry(*card).or_insert(0) += 1 as usize;
        }
    }

    let mut cards = card_counts.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
    cards.sort();

    if try_type(&cards, jokers, &[5]) {
         HandType::FiveOfAKind
    } else if try_type(&cards, jokers, &[4]) {
         HandType::FourOfAKind
    } else if try_type(&cards, jokers, &[3,2]) {
         HandType::FullHouse
    } else if try_type(&cards, jokers, &[3]) {
         HandType::ThreeOfAKind
    } else if try_type(&cards, jokers, &[2,2]) {
         HandType::TwoPair
    } else if try_type(&cards, jokers, &[2]) {
         HandType::OnePair
    } else {
        HandType::HighCard
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    typ: HandType,
    cards: [Card;5],
    bid: usize
}

fn main() {
    println!("Hello, Day 7 2!");

    // For each card, figure wins
    let input = fs::read_to_string("inputs/day7").unwrap();

    let mut hands = input.lines().map(|l| {
        let (cards, bid) = l.split_whitespace().collect_tuple().unwrap();
        let cs = cards.chars().map(card_from_char).collect::<Vec<_>>();
        Hand { cards:[cs[0],cs[1],cs[2],cs[3],cs[4]], typ:hand_type(&cs), bid:bid.parse::<usize>().unwrap() }
    }).collect::<Vec<_>>();

    hands.sort();

    let mut total = 0;
    for (i,h) in hands.iter().enumerate() {
        let rank = i+1;
        total += h.bid * rank;
    };

    println!("Winnings: {total}")
    //243101568
}

#[cfg(test)]
fn to_cards(s: &str) -> Vec<Card> {
    s.chars().map(card_from_char).collect()
}

#[test]
fn test1() {
    assert!(try_type(&vec![(Card::Ace, 5)], 0, vec![5]));
    assert!(try_type(&vec![(Card::Ace, 4)], 1, vec![5]));
    assert!(try_type(&vec![(Card::Ace, 1)], 4, vec![5]));
    assert!(try_type(&vec![], 5, vec![5]));
    assert!(!try_type(&vec![(Card::Ace, 3),(Card::Two, 2)], 0, vec![5]));
    assert!(!try_type(&vec![(Card::Ace, 3),(Card::Two, 2)], 0, vec![4]));
    assert!(try_type(&vec![(Card::Ace, 2),(Card::Two, 3)], 0, vec![3]));
    assert!(try_type(&vec![(Card::Ace, 2),(Card::Two, 3)], 0, vec![3,2]));
    assert!(!try_type(&vec![(Card::Ace, 2),(Card::Two, 3)], 0, vec![3,3]));
    assert!(try_type(&vec![(Card::Ace, 1),(Card::Two, 2)], 2, vec![3,2]));
    assert!(try_type(&vec![(Card::Ace, 1),(Card::Two, 2)], 2, vec![2,2]));
    assert!(try_type(&vec![(Card::Ace, 1),(Card::Two, 1)], 2, vec![2,2]));
    assert!(try_type(&vec![(Card::Ace, 1)], 3, vec![2,2]));

    assert_eq!(hand_type(&to_cards("AAAAA")), HandType::FiveOfAKind);
    assert_eq!(hand_type(&to_cards("AA8AA")), HandType::FourOfAKind);
    assert_eq!(hand_type(&to_cards("23332")), HandType::FullHouse);
    assert_eq!(hand_type(&to_cards("TTT98")), HandType::ThreeOfAKind);
    assert_eq!(hand_type(&to_cards("23432")), HandType::TwoPair);
    assert_eq!(hand_type(&to_cards("A23A4")), HandType::OnePair);
    assert_eq!(hand_type(&to_cards("23456")), HandType::HighCard);

    assert_eq!(hand_type(&to_cards("JAAAA")), HandType::FiveOfAKind);
    assert_eq!(hand_type(&to_cards("AA8JA")), HandType::FourOfAKind);
    assert_eq!(hand_type(&to_cards("23J32")), HandType::FullHouse);
    assert_eq!(hand_type(&to_cards("TTJ98")), HandType::ThreeOfAKind);
    assert_eq!(hand_type(&to_cards("TJJ98")), HandType::ThreeOfAKind);
}
