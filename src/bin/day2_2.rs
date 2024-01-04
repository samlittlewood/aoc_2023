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
    println!("Hello, Day 2 2!");
    let mut total  = 0;
    for l in fs::read_to_string("inputs/day2").unwrap().lines() {
        let id_draws = l.split(':').collect::<Vec<_>>();
        let draws = id_draws.get(1).unwrap().split(';').map(parse_draw).collect::<Vec<_>>();

        let minballs = draws.iter().fold(Draw{red:0, green:0, blue:0}, |acc, d| Draw{
                red: d.red.max(acc.red),
                green: d.green.max(acc.green),
                blue: d.blue.max(acc.blue) });

        let power = minballs.red * minballs.green * minballs.blue;

        total += power;
    }
    println!("Total {total}");
}
