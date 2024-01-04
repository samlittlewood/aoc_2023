use std::fs;
use regex::Regex;

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

fn check_boundary(plan : &Vec<Vec<char>>, xs:usize, xe:usize, y:usize) -> bool {
    if is_symbol(plan[y][xs-1]) || is_symbol(plan[y][xe]) {
        return true;
    }

    for x in (xs-1)..=(xe) {
        if is_symbol(plan[y-1][x]) || is_symbol(plan[y+1][x]) {
            return true;
        }
    }
    false
}

fn main() {
    println!("Hello, Day 3!");
    let re = Regex::new("[0-9]+").unwrap();

    // Get each line
    let file = fs::read_to_string("inputs/day3").unwrap();
    let lines = file.lines().collect::<Vec<_>>();
    let width = lines.iter().fold(0, |acc, l| acc.max(l.len()));

    // Convert to 2d array of chars with border padding of '.'
    let mut plan : Vec<Vec<char>> = vec![];
    plan.push(vec!['.';width+2]);
    for l in &lines {
        let cs = [vec!['.';1], l.chars().collect::<Vec<_>>(), vec!['.';1]].concat();
        plan.push(cs);
    }
    plan.push(vec!['.';width+2]);

    // Find the numbers in each line
    let mut total = 0;
    for (i, l) in lines.iter().enumerate() {
        for m in re.find_iter(l) {
            let number = m.as_str().parse::<u32>().unwrap();
            let (xs,xe) = (m.start()+1, m.end()+1);
            let b = check_boundary(&plan, xs, xe, i+1);
            if b {
                total += number
            }
        }
    }

    println!("Total {}", total);
}
