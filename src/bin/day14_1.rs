use std::fs::{self, File};
use std::io::{self, BufRead, Error};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug)]
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
}

fn main() {
	println!("Hello Day 14 1!");

	let mut input = read_lines("inputs/day14").unwrap();

	let mut platform = Platform::create(&mut input);
//	platform.dump();

	for y in 0..platform.height as i32 {
		for x in 0..platform.width as i32 {
			if platform.at(x,y) == Element::Rock && platform.at(x,y-1) == Element::Empty {
				let mut ny = y;
				while platform.at(x,ny-1) == Element::Empty {
					ny -= 1;
				}
				if ny != y {
					platform.set(x,ny,Element::Rock);
					platform.set(x,y,Element::Empty);
				}
			}
		}
	}

//	platform.dump();
	println!("Load: {}", platform.calculate_load());
	// 106186
}

fn read_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
