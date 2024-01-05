use std::fs;

#[derive(Debug, Clone, Copy)]
struct Cmd {
	cmd: char,
	distance: usize
}

fn cmd_from_colour(s: &str) -> Option<(char, usize)> {
    let cs = s.chars().collect::<Vec<_>>();
    if cs.len() != 7 || cs[0] != '#' {
        return None;
    }
    let mut d: [_; 6] = [0; 6];
    for i in 0..=5 {
        d[i] = cs[i + 1].to_digit(16)?;
    }
    let cmd = match d[5] {
	    0 => 'R',
	    1 => 'D',
	    2 => 'L',
	    3 => 'U',
	    _ => panic!("Bad command: {}", d[5])
    };

    let mut dist = 0;
    for i in 0..=4 {
    	dist = dist * 16 + d[i] as usize;
    }
    Some((cmd,dist))
}

fn apply_commands<F>(commands: &[Cmd], mut func: F)
	where F: FnMut(i32, i32, i32) {
	let mut x = 0;
	let mut y = 0;

	for c in commands {
		match c.cmd {
			'U' => y -= c.distance as i32,
			'D' => y += c.distance as i32,
			'L' => x -= c.distance as i32,
			'R' => x += c.distance as i32,
			_ => panic!("Unknown command: {}", c.cmd)
		}
		func(x, y, c.distance as i32);
	}
}

fn main() {
    println!("Hello Day 18 2!");
    let input = fs::read_to_string("inputs/day18").unwrap();

	let mut commands = Vec::new();

    let parens : &[_] = &['(',')'];
    for l in input.lines() {
        let split_line = l.split_whitespace().collect::<Vec<_>>();
        let (cmd, dist) = cmd_from_colour(split_line[2].trim_matches(parens)).unwrap();
		commands.push(Cmd { cmd, distance: dist });
    }

    let mut px = 0;
    let mut py = 0;
    let mut area:i64 = 0;
    let mut distance = 0;

    apply_commands(&commands, |x,y,d| {
    	let a = ((y as i64) - (py as i64)) * x as i64;
    	area += a;
    	distance += d;
    	(px,py) = (x,y);
    });

   	println!("Area: {}", area+(distance as i64 /2)+1 );
   	// 47452118468566
}
