use std::fs::{self, read_to_string};

const HASH_LIMIT:u32 = 256;

fn hash(s: &str) -> u32 {
	let mut h = 0;

	for c in s.chars() {
		let a = c as u32;
		h += a;
		h = (h * 17) % HASH_LIMIT;
	}
	h
}

fn main() {
    println!("Hello Day 15 1!");

    let input = fs::read_to_string("inputs/day15")
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let steps = input.split(',').collect::<Vec<_>>();

    let mut sum = 0;
    for step in steps {
    	let h = hash(step);
    	println!("Step: {step:?} {h}");
    	sum += h;
    }

    println!("Sum: {sum}");
    // 512797
}

