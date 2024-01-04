use std::fs;

#[derive(Debug, Clone, Copy)]
struct Colour(u8, u8, u8);

impl Colour {
	fn from_string(s: &str) -> Option<Colour> {
	    let cs = s.chars().collect::<Vec<_>>();
	    if cs.len() != 7 || cs[0] != '#' {
	        return None;
	    }
	    let mut d: [_; 6] = [0; 6];
	    for i in 0..=5 {
	        d[i] = cs[i + 1].to_digit(16)?;
	    }
	    Some(Colour(
	        (d[0] * 16 + d[1]) as u8,
	        (d[2] * 16 + d[3]) as u8,
	        (d[4] * 16 + d[5]) as u8,
	    ))
	}
}

#[derive(Debug, Clone, Copy)]
struct Cmd {
	cmd: char,
	distance: usize,
	colour: Colour
}

#[derive(Debug)]
struct Grid {
	width: usize,
	height: usize,
	elements: Vec<bool>,
	colours: Vec<Colour>,
}

impl Grid {
	fn create(width:usize, height:usize) -> Grid {
		Grid {
			width, height,
			elements: vec![false; width * height],
			colours: vec![Colour(0,0,0); width * height],
		}
	}
}

fn apply_commands<F>(commands: &[Cmd], mut func: F)
	where F: FnMut(i32, i32, i32, Colour) {
	let mut x = 1000;
	let mut y = 1000;

	for c in commands {
		match c.cmd {
			'U' => y -= (c.distance) as i32,
			'D' => y += (c.distance) as i32,
			'L' => x -= (c.distance) as i32,
			'R' => x += (c.distance) as i32,
			_ => panic!("Unknown command: {}", c.cmd)
		}
		func(x, y, c.distance as i32, c.colour);
	}
}

fn main() {
    println!("Hello Day 18 1!");
    let input = fs::read_to_string("inputs/day18").unwrap();

	let mut commands = Vec::new();

    let parens : &[_] = &['(',')'];
    for l in input.lines() {
        let split_line = l.split_whitespace().collect::<Vec<_>>();
        let (cmd, dist, colour) = (
            split_line[0].chars().take(1).next().unwrap(),
            split_line[1].parse::<usize>().unwrap(),
            Colour::from_string(split_line[2].trim_matches(parens)).unwrap()
        );
		commands.push(Cmd { cmd, distance: dist, colour });
    }

    let mut px = 1000;
    let mut py = 1000;
    let mut area = 0;
    let mut distance = 0;
    apply_commands(&commands, |x,y,d,c| {
    	if(y > py) {
    		area += (y-py) * x;
    	} else if (y < py) {
    		area += (y-py) * x;
	   	}
    	distance += d;
//    	println!("{} {}",x, y);
    	(px,py) = (x,y);
    });

   	println!("Area: {area} {distance} {}", area+(distance/2)+1 );
   	// 58550
}
