use std::fs;

#[derive(Debug)]
struct Map {
	rocks: Vec<bool>,
	width: usize,
	height: usize,
	start: (i32, i32)
}

impl Map {
	fn from_string(str: &str) -> Self {
		let mut rocks = Vec::new();
		let mut start = (0,0);
		let mut width = 0;
		let mut height = 0;
		for l in str.lines() {
			l.chars().enumerate().for_each(|(x,c)| { rocks.push(c=='#'); if c == 'S' {start = (x as i32, height as i32)}} );
			width = usize::max(width, l.len());
			height += 1;
		}

		Map { rocks, width, height, start}
	}

	fn offset(&self, x:i32, y:i32) -> Option<usize> {
		if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
			None
		} else {
			Some(y as usize * self.width + x as usize)
		}
	}

	fn start_marks(&self) -> Vec<bool> {
		let mut m = vec![false; self.width * self.height];
		m[self.offset(self.start.0, self.start.1).unwrap()] = true;
		m
	}

	fn grow_marks(&self, prev: &[bool]) -> Vec<bool> {
		let mut next = vec![false; self.width * self.height];
		for y in 0 .. self.height as i32 {
			for x in 0 .. self.width as i32 {
				let o = self.offset(x, y).unwrap();
				// If a clear cell, and any of neighbours are live in previous step, this cell is marked
				if !self.rocks[o] {
					let mut c = false;
					if let Some(n) = self.offset(x, y-1) {
						if prev[n] {
							c = true;
						}
					}
					if let Some(s) = self.offset(x, y+1) {
						if prev[s] {
							c = true;
						}
					}
					if let Some(e) = self.offset(x+1, y) {
						if prev[e] {
							c = true;
						}
					}
					if let Some(w) = self.offset(x-1, y) {
						if prev[w] {
							c = true;
						}
					}
					next[o] = c;
				}
			}
		}
		next
	}


	fn dump_marks(&self, marks:&[bool]) {
		println!("-- {} x {}", self.width, self.height);
		for y in 0 .. self.height as i32 {
			for x in 0 .. self.width as i32 {
				let c = match (marks[self.offset(x, y).unwrap()], self.rocks[self.offset(x, y).unwrap()]) {
					(false, false) => '.',
					(true, false) => 'O',
					(false, true) => '#',
					(true, true) => '?',
				};
				print!("{}", c);
			}
			println!("");
		}
	}

}

fn main() {
	println!("Hello Day 21 1!");

	let input = fs::read_to_string("inputs/day21").unwrap();

	let map = Map::from_string(&input);

	let mut marks = map.start_marks();

	for step in 1..=64 {
		println!("Step: {step}");
		marks = map.grow_marks(&marks);
	}
	map.dump_marks(&marks);

	let reachable_count = marks.iter().filter(|&&c| c).count();
	println!("Reachable: {reachable_count}");
	// 3574
}