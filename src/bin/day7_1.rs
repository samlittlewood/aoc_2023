use std::{fs, collections::HashMap};
use itertools::Itertools;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    None,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Card {
    None,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn card_from_char(c: char) -> Card{
    match c {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'J' => Card::Jack,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => Card::None
    }
}

fn hand_type(hand: &str) -> HandType
{
    let mut card_map : HashMap<char, usize> = HashMap::new();

    assert_eq!(hand.len(), 5);

    let mut cards = hand.chars();
    while let Some(card) = cards.next() {
        card_map.insert(card, card_map.get(&card).unwrap_or(&0) + 1);
    }

    let mut cards = card_map.iter().collect::<Vec<_>>();
    cards.sort_by_key(|(_,&v)| usize::MAX-v);

    if cards.len() == 1 && *cards[0].1 == 5 {
        HandType::FiveOfAKind
    } else if cards.len() == 2 && *cards[0].1 == 4 {
        HandType::FourOfAKind
    } else if cards.len() == 2 {
        HandType::FullHouse
    } else if cards.len() == 3 && *cards[0].1 == 3 {
        HandType::ThreeOfAKind
    } else if cards.len() == 3 && *cards[0].1 == 2 {
        HandType::TwoPair
    } else if cards.len() == 4 && *cards[0].1 == 2 {
        HandType::OnePair
    } else if cards.len() == 5 {
        HandType::HighCard
    } else {
        HandType::None
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    typ: HandType,
    cards: [Card;5],
    bid: usize
}

fn main() {
    println!("Hello, Day 7 1!");

    // For each card, figure wins
    let input = fs::read_to_string("inputs/day7").unwrap();

    let mut hands = input.lines().map(|l| {
        let (cards, bid) = l.split_whitespace().collect_tuple().unwrap();
        let cs = cards.chars().map(card_from_char).collect::<Vec<_>>();
        Hand { cards:[cs[0],cs[1],cs[2],cs[3],cs[4]], typ:hand_type(cards), bid:bid.parse::<usize>().unwrap() }
    }).collect::<Vec<_>>();

    hands.sort();

    let mut total = 0;
    for (i,h) in hands.iter().enumerate() {
        let rank = i+1;
//        println!("{:?} {:5} {:-5}", h.cards, h.bid, rank);
        total += h.bid * rank;
    };

    println!("Winnings: {total}")
}

#[test]
fn test1() {
    assert_eq!(hand_type("AAAAA"), HandType::FiveOfAKind);
    assert_eq!(hand_type("AA8AA"), HandType::FourOfAKind);
    assert_eq!(hand_type("23332"), HandType::FullHouse);
    assert_eq!(hand_type("TTT98"), HandType::ThreeOfAKind);
    assert_eq!(hand_type("23432"), HandType::TwoPair);
    assert_eq!(hand_type("A23A4"), HandType::OnePair);
    assert_eq!(hand_type("23456"), HandType::HighCard);
}
