use std::fs::{self, File};
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
enum Element {
	Empty,
	Rock,
	Block
}

impl Element {
	fn from_char(c: char) -> Self {
		match c {
			'O' => Element::Rock,
			'#' => Element::Block,
			_ => Element::Empty
		}
	}
}

#[derive(Debug, Hash)]
struct Platform {
	width: usize,
	height: usize,
	elements: Vec<Element>
}

impl Platform {
	fn create(input: &mut impl Iterator<Item = Result<String,Error>>) -> Self {

		let mut width = 0;
		let mut height = 0;
		let mut elements = Vec::new();

		for l in input.map(Result::unwrap) {
			width = usize::max(width, l.len());
			height += 1;
			let mut line_elements = l.chars().map(Element::from_char).collect::<Vec<_>>();
			elements.append(&mut line_elements);
		}

		Platform { width, height, elements}
	}

	fn dump(&self) {
		println!("-- {}x{}", self.width, self.height);
		for y in 0..self.height {
			for x in 0..self.width {
				let c = match self.elements[y*self.width + x] {
					Element::Empty => '.',
					Element::Rock => 'O',
					Element::Block => '#',
				};
				print!("{}", c);
			}
			println!("");
		}
	}

	fn at(&self, x: i32, y:i32) -> Element {
		if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
			self.elements[y as usize * self.width + x as usize]
		} else {
			Element::Block
		}
	}

	fn set(&mut self, x: i32, y:i32, e:Element) {
		if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
			self.elements[y as usize * self.width + x as usize] = e;
		}
	}

	fn calculate_load(&self) -> usize {
		let mut load = 0;
		for y in 0..self.height  {
			for x in 0..self.width  {
				if self.at(x as i32, y as i32) == Element::Rock {
					load += self.height-y;
				}
			}
		}
		load
	}

	fn tilt_north(&mut self) {
		for y in 0..self.height as i32 {
			for x in 0..self.width as i32 {
				if self.at(x,y) == Element::Rock && self.at(x,y-1) == Element::Empty {
					let mut ny = y;
					while self.at(x,ny-1) == Element::Empty {
						ny -= 1;
					}
					if ny != y {
						self.set(x,ny,Element::Rock);
						self.set(x,y,Element::Empty);
					}
				}
			}
		}
	}

	fn tilt_south(&mut self) {
		for y in (0..self.height as i32).rev() {
			for x in 0..self.width as i32 {
				if self.at(x,y) == Element::Rock && self.at(x,y+1) == Element::Empty {
					let mut ny = y;
					while self.at(x,ny+1) == Element::Empty {
						ny += 1;
					}
					if ny != y {
						self.set(x,ny,Element::Rock);
						self.set(x,y,Element::Empty);
					}
				}
			}
		}
	}

	fn tilt_west(&mut self) {
		for x in 0..self.width as i32 {
			for y in 0..self.height as i32 {
				if self.at(x,y) == Element::Rock && self.at(x-1,y) == Element::Empty {
					let mut nx = x;
					while self.at(nx-1,y) == Element::Empty {
						nx -= 1;
					}
					if nx != x {
						self.set(nx,y,Element::Rock);
						self.set(x,y,Element::Empty);
					}
				}
			}
		}
	}

	fn tilt_east(&mut self) {
		for x in (0..self.width as i32).rev() {
			for y in 0..self.height as i32 {
				if self.at(x,y) == Element::Rock && self.at(x+1,y) == Element::Empty {
					let mut nx = x;
					while self.at(nx+1,y) == Element::Empty {
						nx += 1;
					}
					if nx != x {
						self.set(nx,y,Element::Rock);
						self.set(x,y,Element::Empty);
					}
				}
			}
		}
	}

}

const TOTAL_CYCLES:usize = 1000000000;

fn main() {
	println!("Hello Day 14 1!");

	let mut input = read_lines("inputs/day14").unwrap();

	let mut platform = Platform::create(&mut input);

	let mut cycle_map = HashMap::new();

	let mut cycle_start = 0;
	let mut cycle_length = 0;
	let mut cycle_loads = Vec::new();

	// Evolve platform state until there is a cycle, recording loads of each state
	for cycle in 0..1000 {
		// Update platform state
		platform.tilt_north();
		platform.tilt_west();
		platform.tilt_south();
		platform.tilt_east();

		// Hash the platform state
		let mut hasher = DefaultHasher::new();
		platform.hash(&mut hasher);
		let hash = hasher.finish();

		// Figure load
		let load = platform.calculate_load();

		println!("{cycle:4} {hash} {load}");

		if let Some(&h) = cycle_map.get(&hash) {
			println!("Found Cycle: {h} {cycle} {hash} {load}");
			cycle_start = h;
			cycle_length = cycle - h;
			break;
		} else {
			cycle_map.insert(hash, cycle);
			cycle_loads.push(load);
		}
	}

	// Use cycle description to get load for cycle of interest
	let final_load_idx = (TOTAL_CYCLES - cycle_start -1) % cycle_length;
	let final_load = cycle_loads[final_load_idx + cycle_start];
	println!("Final load:{final_load}");
	// 106390
}

fn read_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
