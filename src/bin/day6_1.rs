use std::fs;

use itertools::Itertools;


fn distance(race_time: i64, button_time:i64) -> i64
{
    button_time * race_time - button_time * button_time
}

fn max_distance(race_time: i64, button_time:i64) -> i64
{
    // distance = r * b - b * b
    // d(b) = r - 2b
    // max =  b/2
    return race_time / 2;
}

fn main() {
    println!("Hello, Day 6 1!");

    // For each card, figure wins
    let input = fs::read_to_string("inputs/day6").unwrap();
    let mut input_iter = input.lines();
    let times = input_iter.next().unwrap()[5..].split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect_vec();
    let distances = input_iter.next().unwrap()[9..].split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect_vec();
    assert_eq!(times.len(), distances.len());

    println!("{times:?}");
    println!("{distances:?}");

    let mut wins_product = 1;
    for (race_time,race_best_d) in times.into_iter().zip(distances.into_iter()) {
//        println!("{t} {d}");
        let mut wins =0;
        for b in 0..=race_time {
            let d = distance(race_time, b);
            if d > race_best_d {
                wins += 1
            }
        }
        println!("Wins:{wins}");
        wins_product *= wins;
    }

    println!("Wins Product:{wins_product}");
}

#[test]
fn test1() {
    assert_eq!(distance(7, 1), 6);
    assert_eq!(distance(7, 2), 10);
    assert_eq!(distance(7, 3), 12);
    assert_eq!(distance(7, 4), 12);
    assert_eq!(distance(7, 7), 0);
}
