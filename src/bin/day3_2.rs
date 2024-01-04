use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Part {
    number : u32,
    xs : usize,
    xe : usize,
    y : usize,
}

fn check_number(part: &Part, x: usize, y: usize) -> bool {
    x >= part.xs-1 && x <= part.xe &&
    y >= part.y-1 && y <= part.y+1
}

fn main() {
    println!("Hello, Day 3 2!");
    let re = Regex::new("[0-9]+").unwrap();

    // Get each line
    let file = fs::read_to_string("inputs/day3").unwrap();
    let lines = file.lines().collect::<Vec<_>>();
    let width = lines.iter().fold(0, |acc, l| acc.max(l.len()));

    // Convert to 2d array of chars with border padding of '.'
    let mut plan : Vec<Vec<char>> = vec![];

    plan.push(vec!['.';width+2]);
    for l in &lines {
//        println!("- {l}");
        let cs = [vec!['.';1], l.chars().collect::<Vec<_>>(), vec!['.';1]].concat();
        plan.push(cs);
    }
    plan.push(vec!['.';width+2]);

    // Find the numbers in each line and add to vector
    let mut parts : Vec<Part> = vec![];

    for (y, l) in lines.iter().enumerate() {
        for m in re.find_iter(l) {
            let number = m.as_str().parse::<u32>().unwrap();
            let (xs,xe) = (m.start()+1, m.end()+1);
            parts.push(Part { number, xs, xe, y:y+1 });
        }
    }
//    println!("Parts {:#?}", parts);
    let mut total = 0;

    for y in 1..(plan.len()-1) {
        for x in 1..width {
            if plan[y][x] == '*' {
                let ps = parts.iter().filter(|&p| check_number(p, x, y)).collect::<Vec<_>>();
//                println!("gear {x} {y} {ps:?}");
                if ps.len() == 2 {
                    total += ps[0].number * ps[1].number;
                }
            }
        }
    }

    println!("Total {}", total);
}
