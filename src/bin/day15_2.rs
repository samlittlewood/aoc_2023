use std::fs::{self, read_to_string};

const HASH_LIMIT: u32 = 256;

fn hash(s: &str) -> u32 {
    let mut h = 0;

    for c in s.chars() {
        let a = c as u32;
        h += a;
        h = (h * 17) % HASH_LIMIT;
    }
    h
}

#[derive(Debug, Clone)]
struct HashEntry {
    label: String,
    value: u32,
}

fn find_slot(slots:&[HashEntry], label:&str) -> Option<usize> {
	for i in 0..slots.len() {
		if slots[i].label == label {
			return Some(i)
		}
	}
	None
}

fn main() {
    println!("Hello Day 15 2!");

    let input = fs::read_to_string("inputs/day15")
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let steps = input.split(',').collect::<Vec<_>>();

    let mut hashmap = Vec::new();
    hashmap.resize(HASH_LIMIT as usize, Vec::<HashEntry>::new());

    for step in steps {
        let label_op = step
            .split_inclusive(|c| c == '=' || c == '-')
            .collect::<Vec<_>>();
        let (label, op, value) = (
            &label_op[0][0..label_op[0].len() - 1],
            label_op[0].chars().last().unwrap(),
            &label_op.get(1).unwrap_or(&"0").parse::<u32>().unwrap(),
        );
        let h = hash(label);
        let mut slots = hashmap.get_mut(h as usize).unwrap();
        let maybe_slot = find_slot(slots, label);
        match op {
        	'=' => {
        		if let Some(i) = maybe_slot {
        			slots[i].value = *value;
        		} else {
        			slots.push(HashEntry { label:label.to_string(), value:*value });
        		}
        	}
        	'-' => {
        		if let Some(i) = maybe_slot {
        			slots.remove(i);
        		}
        	}
			_ => { panic!("Bad op"); }
        }
        println!("Step: {step:?} {label} {op:?} {value}");
    }

    let mut sum = 0;

    for b in 0..hashmap.len() {
	    for s in 0..hashmap[b].len() {
	    	sum += (b+1) * (s+1) * hashmap[b][s].value as usize;
	    }
    }

    println!("Sum: {sum}");
    // 512797
}
