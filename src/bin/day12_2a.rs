use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Condition {
    E,
    U,
    S,
}

impl Condition {
    fn from_char(c: char) -> Self {
        match c {
            '?' => Condition::U,
            '#' => Condition::S,
            _ => Condition::E,
        }
    }
}

fn count_groups(springs: &[Condition]) -> Vec<u32> {
    let mut g = Vec::new();

    let mut count = 0;
    for &c in springs {
        match c {
            Condition::E => {
                if count > 0 {
                    g.push(count);
                    count = 0
                }
            }
            Condition::S => count += 1,
            Condition::U => (),
        }
    }

    if count > 0 {
        g.push(count);
    }

    g
}

fn fill_unknowns(springs: &[Condition], pattern: u32) -> Vec<Condition> {
    let mut r = Vec::new();
    let mut p = pattern;

    for &c in springs {
        if c == Condition::U {
            if p & 1 == 1 {
                r.push(Condition::S)
            } else {
                r.push(Condition::E)
            }
            p >>= 1;
        } else {
            r.push(c)
        }
    }

    r
}

struct Record {
	springs: Vec<Condition>,
	groups: Vec<u32>,
}

impl Record {
	fn create(text: &str) -> Self {
        let line_parts = text.split_whitespace().collect::<Vec<_>>();
        let (springs, groups) = (
            line_parts[0]
                .chars()
                .map(Condition::from_char)
                .collect::<Vec<_>>(),
            line_parts[1]
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>(),
        );
        Record {springs, groups}
	}
}

fn arrangements(springs: &[Condition], group: u32, next_groups: &[u32]) -> usize {
	assert!(!(group==0 && !next_groups.is_empty()));
	match springs.first() {
		Some(Condition::E) => arrangements(&springs[1..], group, &next_groups),
		Some(Condition::S) => arrangements_spring(&springs[1..], group, &next_groups),
		Some(Condition::U) => arrangements(&springs[1..], group, &next_groups)
									+ arrangements_spring(&springs[1..], group, &next_groups),
		None =>
    		if group == 0 && next_groups.is_empty() {
    		 	1
	   		} else {
	   			0
	   		}
    }
}

fn arrangements_spring(springs: &[Condition], group: u32, next_groups: &[u32]) -> usize {
	if group > 1 {
        arrangements_group_more(&springs, group-1, &next_groups)
    } else if group > 0 {
    	// Consumed current group
        if next_groups.len() > 0 {
        	// More groups - must have a separating space
            arrangements_group_end(&springs, next_groups[0], &next_groups[1..])
        } else {
            arrangements_group_end(&springs, 0, &next_groups)
        }
    } else {
    	0
    }
}

fn arrangements_group_more(springs: &[Condition], group: u32, next_groups: &[u32]) -> usize {
	assert!(group > 0);
    match springs.first() {
        Some(Condition::U) |
        Some(Condition::S) => arrangements_spring(&springs[1..], group, &next_groups),
        Some(Condition::E) => 0,
        None => 0,
    }
}

fn arrangements_group_end(springs: &[Condition], group: u32, next_groups: &[u32]) -> usize {
    match springs.first() {
        Some(Condition::E) |
        Some(Condition::U) => arrangements(&springs[1..], group, &next_groups),
        Some(Condition::S) => 0,
        None => arrangements(&springs, group, &next_groups),
    }
}


fn main() {
	use Condition::*;
    println!("Hello Day 12 2!");

	assert_eq!(arrangements(&vec![S,E], 1, &vec![]), 1);

    let input = fs::read_to_string("inputs/day12").unwrap();
    let mut enum_sum = 0;
    let mut rec_sum = 0;
    for l in input.lines() {
        let line_parts = l.split_whitespace().collect::<Vec<_>>();
        let (springs, groups) = (
            line_parts[0]
                .chars()
                .map(Condition::from_char)
                .collect::<Vec<_>>(),
            line_parts[1]
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>(),
        );

        let num_unknowns = springs.iter().filter(|c| **c == Condition::U).count();
        let num_springs = springs.iter().filter(|c| **c == Condition::S).count();
        let total_springs: u32 = groups.iter().sum();
        let unknown_springs = total_springs - num_springs as u32;

        // for possible combination of umknowns
        //  filter those with correct number of unknown springs
        let mut enum_arrangements = 0;
        for i in (0..2_u32.pow(num_unknowns as u32)).filter(|n| n.count_ones() == unknown_springs) {
            let candidate = fill_unknowns(&springs, i);
            let candidate_groups = count_groups(&candidate);
            //			println!("{i:6} {candidate:?} {candidate_groups:?}");
            if candidate_groups == groups {
                enum_arrangements += 1;
            }
        }
        let mut rec_arrangements = arrangements(&springs, groups[0], &groups[1..]);

        println!("Arragments: {enum_arrangements} {rec_arrangements}");

        enum_sum += enum_arrangements;
        rec_sum += rec_arrangements;
    }

    println!("Sum: {enum_sum} {rec_sum}");
}

#[test]
fn test_1() {
	use Condition::*;

	assert_eq!(arrangements(&vec![U], 0, &vec![]), 1);

	assert_eq!(arrangements(&vec![], 0, &vec![]), 1);
	assert_eq!(arrangements(&vec![], 1, &vec![]), 0);
	assert_eq!(arrangements(&vec![], 2, &vec![]), 0);
	assert_eq!(arrangements(&vec![], 2, &vec![1]), 0);

	assert_eq!(arrangements(&vec![E], 0, &vec![]), 1);
	assert_eq!(arrangements(&vec![S,E], 1, &vec![]), 1);

	assert_eq!(arrangements(&vec![S], 1, &vec![]), 1);
	assert_eq!(arrangements(&vec![S, S], 2, &vec![]), 1);
	assert_eq!(arrangements(&vec![S, E, S], 2, &vec![]), 0);
	assert_eq!(arrangements(&vec![S, E, S], 1, &vec![1]), 1);
	assert_eq!(arrangements(&vec![U, U, E, S, S, S], 1, &vec![3]), 2);
	assert_eq!(arrangements(&vec![U, U, E, U, S, S], 1, &vec![3]), 2);
	assert_eq!(arrangements(&vec![U, U, E, U, U, S], 1, &vec![3]), 2);
	assert_eq!(arrangements(&vec![U, U, E, U, U, U], 1, &vec![3]), 2);
	assert_eq!(arrangements(&vec![E, U, U, E, U, U, U], 1, &vec![3]), 2);
	assert_eq!(arrangements(&vec![E, U, U, E, U, U, E, U, U, U], 1, &vec![1, 3]), 4);
	assert_eq!(arrangements(&vec![E, U, U, E, E, U, U, E, E, E, U, U, U], 1, &vec![1, 3]), 4);
	assert_eq!(arrangements(&vec![E, U, U, E, E, U, U, E, E, E, U, S, S], 1, &vec![1, 3]), 4);
	assert_eq!(arrangements(&vec![E, U, U, E, E, U, U, E, E, E, U, S, S, E], 1, &vec![1, 3]), 4);

	assert_eq!(arrangements(&vec![E, U, U, E, E, U, U, E, E, E, U, S, S, E], 1, &vec![1,3]), 4);

	assert_eq!(arrangements(&vec![U, S, S, S, U, U, U, U, U, U, U, U], 3, &vec![2, 1]), 10);
}