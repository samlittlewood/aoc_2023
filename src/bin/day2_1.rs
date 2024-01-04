use std::fs;

#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_draw(s: &str) -> Draw {
    let mut g = Draw{ red:0, green:0, blue:0 };

    for ball in s.split(",").map(|s| s.trim()) {
        let (num,col) = ball.split_at(ball.find(" ").unwrap());
        let n = num.parse::<u32>().unwrap();
        match col.trim() {
            "red" => g.red = n,
            "green" => g.green = n,
            "blue" => g.blue = n,
            _ => ()
        }
    }

    g
}

fn main() {
    println!("Hello, Day 2!");
    let mut total  = 0;
    for l in fs::read_to_string("inputs/day2").unwrap().lines() {
        let id_draws = l.split(':').collect::<Vec<_>>();
        let id = id_draws.get(0).unwrap()[5..].parse::<u32>().unwrap();
        let draws = id_draws.get(1).unwrap().split(';').map(parse_draw).collect::<Vec<_>>();

        let invalid_draws = draws.iter().filter(|d| d.red > 12 || d.green > 13 || d.blue > 14).count();

        println!("{id} {invalid_draws} {draws:?}");

        if invalid_draws == 0 {
            total += id;
        }
    }
    println!("Total {total}");
}
