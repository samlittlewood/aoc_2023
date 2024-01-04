//
use std::fs;

#[derive(Debug)]
struct Map {
	rocks: Vec<bool>,
	width: usize,
	height: usize,
	start: (i32, i32)
}

impl Map {
	fn from_string(str: &str, repeat: i32) -> Self {
		let mut rocks = Vec::new();
		let mut start = (0,0);
		let mut width = 0;
		let mut height = 0;
		for _ry in 0..repeat {
			for l in str.lines() {
				for _rx in 0..repeat {
					l.chars().enumerate().for_each(|(x,c)| { rocks.push(c=='#'); if c == 'S' {start = (x as i32, height as i32)}} );
				}
				width = usize::max(width, l.len() * repeat as usize);
				height += 1;
			}
		}
		start = (repeat as i32 /2 * 131 + 65, repeat as i32 /2 * 131 + 65);
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
		println!("-- {} x {}. start:{:?}", self.width, self.height, self.start);
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

	fn reachable_in_tile(&self, marks:&[bool], tx:i32, ty:i32, size:i32) -> usize {
		let mut count = 0;
		for y in ty * size .. (ty+1) * size {
			for x in tx * size .. (tx+1) * size {
				if marks[self.offset(x,y).unwrap()] {
					count +=1;
				}
			}
		}
		count
	}
}

fn caclulate_reachable(n: usize) -> usize {
	(5539 + 5526 + 5516 + 5529) + 			// Corners
	n * (944 + 929 + 937 + 943) + 			// Small diagonal edge pieces
	(n-1) * (6442 + 6433 + 6429 + 6423) + 	// Large diagonal edge pieces
	n * n * (7327) + 						// First full inside tile
	(n-1) * (n-1) * (7336) 					// Second full inside tile
}

fn main() {
	println!("Hello Day 21 2!");

	let input = fs::read_to_string("inputs/day21").unwrap();

	for n in 1 .. 7 {
		let map = Map::from_string(&input,1 + 2*n);

		let mut marks = map.start_marks();
		let steps = 65 + 131 * n;
		for step in 1..=steps {
			marks = map.grow_marks(&marks);
		}
		let reachable_count = marks.iter().filter(|&&c| c).count();
		println!("Iteration:{n} steps:{steps} reachable:{reachable_count}");

		let calcualted_reachable = caclulate_reachable(n as usize);
		println!("Calculated reachable: {calcualted_reachable}");

		for ty in 0 .. (1+2*n) {
			for tx in 0 .. (1+2*n) {
				let c = map.reachable_in_tile(&marks, tx, ty, 131);
				if c > 0 {
					print!("{c:5}");
				} else {
					print!("     ");
				}
			}
			println!("");
		}
	}

	// Calcualte final answer
	let iterations = 202300;
	println!("Calcualeted: steps:{} reachable:{}", 65 + iterations * 131, caclulate_reachable(iterations));
	// 600090522932119
}