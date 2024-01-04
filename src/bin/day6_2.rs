use std::fs;


fn race_distance(race_time: i64, button_time:i64) -> i64
{
    button_time * race_time - button_time * button_time
}

fn isqrt(x: i64) -> i64
{
    (x as f64).sqrt() as i64
}

fn solve_win(race_time: i64, race_distance:i64) -> (i64,i64)
{
    // Solve quadratic
    let a = -1;
    let b = race_time;
    let c = -race_distance;

    let r = isqrt(b*b - 4*a*c);
    ((-b + r ) / 2 * a, (-b - r )/ 2 * a)
}

fn max_distance(race_time: i64, button_time:i64) -> i64
{
    // distance = r * b - b * b
    // d(b) = r - 2b
    // max =  b/2
    return race_time / 2;
}

fn main() {
    println!("Hello, Day 6 2!");

    // For each card, figure wins
    let input = fs::read_to_string("inputs/day6").unwrap();
    let mut input_iter = input.lines();
    let time = input_iter.next().unwrap()[5..].chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<i64>().unwrap();
    let distance = input_iter.next().unwrap()[9..].chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<i64>().unwrap();

    println!("{time:?}");
    println!("{distance:?}");

    let (mut b1, mut b2) = solve_win(time, distance);

    // Fix up rounding
    while race_distance(time, b1) < distance {
        b1 += 1;
    }

    while race_distance(time, b2) < distance {
        b2 -= 1;
    }

    let wins = b2 - b1 + 1;
    println!("Wins:{wins}");
}

#[test]
fn test1() {
    assert_eq!(distance(7, 1), 6);
    assert_eq!(distance(7, 2), 10);
    assert_eq!(distance(7, 3), 12);
    assert_eq!(distance(7, 4), 12);
    assert_eq!(distance(7, 7), 0);
}
