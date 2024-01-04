use std::fs;

#[derive(Debug, Clone, Copy)]
struct Coord {
	x:i32,
	y:i32
}

#[derive(Debug, Clone)]
struct Image {
	width: usize,
	height: usize,
	pixels: Vec<u32>,
	galaxies: Vec<Coord>
}

impl Image {
	fn from_text(text: &str) -> Image {

		// Figure width and height
		let (mut width, mut height) = (0,0);
		let mut pixels = vec![];
		let mut row_flags = vec![];
		let mut column_flags = vec![];

		for l in text.lines() {
			width = usize::max(width, l.len());
			height += 1;
			let lb = l.chars().map(|c| c == '#').collect::<Vec<_>>();
			column_flags.resize(width, true);
			column_flags.iter_mut().zip(&lb).for_each(|(c,b)| *c &= !b);
			row_flags.push(!lb.iter().any(|b| *b));

			pixels.extend(lb);
		}

		let new_width = width + column_flags.iter().filter(|&&f| f).count();
		let new_height = height + row_flags.iter().filter(|&&f| f).count();
		let row_offsets = row_flags.iter().scan(0, |s, &f| {if f { *s+=1 }; Some(*s)}).collect::<Vec<_>>();
		let column_offsets = column_flags.iter().scan(0, |s, &f| {if f { *s+=1 }; Some(*s)}).collect::<Vec<_>>();

		let mut new_pixels = Vec::new();
		new_pixels.resize(new_width * new_height, 0);

		let mut galaxies = Vec::new();
		galaxies.push(Coord { x:0, y:0 });

		for y in 0 .. height {
			let ny = y + row_offsets[y];
			for x in 0 .. width {
				let nx = x + column_offsets[x];
				if pixels[y * width + x] {
					new_pixels[ny * new_width + nx] = galaxies.len() as u32;
					galaxies.push(Coord { x:nx as i32,y:ny as i32} );
				}
			}
		}
		Image {width: new_width, height: new_height, pixels: new_pixels, galaxies: galaxies}
	}

	fn at(&self, x: i32, y: i32) -> u32 {
		if x < 0 || y < 0 || x >= (self.width as i32) || y >= (self.height as i32) {
			0
		} else {
			self.pixels[self.width * y as usize + x as usize]
		}
	}

	fn dump(&self) {
		println!("{} {}", self.width, self.height);
		for y in 0..self.height {
			for x in 0..self.width {
				let g = self.at(x as i32, y as i32);
				if g != 0 {
					print!("{}", char::from_digit(g % 16, 16).unwrap());
				} else {
					print!(".")

				}
			}
			println!("");
		}
	}
}

fn main() {
	println!("Hello Day 11 1!");

	let input = fs::read_to_string("inputs/day11").unwrap();

	let image = Image::from_text(&input);
	image.dump();

	let mut sum = 0;
	for ga in 1 .. image.galaxies.len() {
		for gb in ga .. image.galaxies.len() {
			let ca = image.galaxies[ga];
			let cb = image.galaxies[gb];
			let dist = ca.x.abs_diff(cb.x) + ca.y.abs_diff(cb.y);
//			println!("{ga:3} {gb:<3} {dist}")
			sum += dist;
		}
	}

	println!("Sum: {sum}")
	// 10490062
}