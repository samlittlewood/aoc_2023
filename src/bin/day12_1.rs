use std::fs;

#[derive(Debug, Copy, Clone,PartialEq)]
enum Condition {
	None,
	Unknown,
	Spring,
}

impl Condition {
	fn from_char(c: char) -> Self {
		match c {
		'?' => Condition::Unknown,
		'#' => Condition::Spring,
		_ => Condition::None
		}
	}
}

fn count_groups(springs: &Vec<Condition>) -> Vec<u32> {
	let mut g = Vec::new();

	let mut count = 0;
	for &c in springs {
		match c {
			Condition::None => if count > 0 {g.push(count); count = 0 },
			Condition::Spring => count += 1,
			Condition::Unknown => () ,
		}
	}

	if count > 0 {
		g.push(count);
	}

	g
}

fn fill_unknowns(springs: &Vec<Condition>, pattern:u32) -> Vec<Condition> {
	let mut r = Vec::new();
	let mut p = pattern;

	for &c in springs {
		if c == Condition::Unknown {
			if p & 1 == 1{
				r.push(Condition::Spring)
			} else {
				r.push(Condition::None)
			}
			p >>= 1;
		} else {
			r.push(c)
		}
	}

	r
}


fn main()
{
	println!("Hello Day 12 1!");

	let input = fs::read_to_string("inputs/day12").unwrap();
	let mut sum = 0;
	for l in input.lines() {
		let line_parts = l.split_whitespace().collect::<Vec<_>>();
		let (springs,groups) = (
			line_parts[0].chars().map(Condition::from_char).collect::<Vec<_>>(),
			line_parts[1].split(',').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>());
		println!("s:{springs:?} g:{groups:?}");

		let num_unknowns = springs.iter().filter(|c| **c == Condition::Unknown).count();
		let num_springs = springs.iter().filter(|c| **c == Condition::Spring).count();
		let total_springs:u32 = groups.iter().sum();
		let unknown_springs = total_springs - num_springs as u32;

		println!("Nu: {num_unknowns}");

		// for possible combination of umknowns
		//  filter those with correct number of unknown springs
		let mut arrangements  = 0;
		for i in (0..2_u32.pow(num_unknowns as u32)).filter(|n| n.count_ones() == unknown_springs) {
			let candidate = fill_unknowns(&springs, i);
			let candidate_groups = count_groups(&candidate);
//			println!("{i:6} {candidate:?} {candidate_groups:?}");
			if candidate_groups == groups {
				arrangements += 1;
			}
		}

		println!("Arragments: {arrangements}");
		sum += arrangements;
	}

	println!("Sum: {sum}");
	// 7771
}
