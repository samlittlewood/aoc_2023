use std::fs;

#[derive(Debug, Clone, Copy)]
struct Coord {
	x:i64,
	y:i64
}

#[derive(Debug, Clone)]
struct Image {
	galaxies: Vec<Coord>
}

const EXPANSION: i64 = 999999;
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

		let row_offsets = row_flags.iter().scan(0_i64, |s, &f| {if f { *s+=EXPANSION }; Some(*s)}).collect::<Vec<_>>();
		let column_offsets = column_flags.iter().scan(0_i64, |s, &f| {if f { *s+=EXPANSION }; Some(*s)}).collect::<Vec<_>>();

		let mut galaxies = Vec::new();
		galaxies.push(Coord { x:0, y:0 });

		for y in 0 .. height as i64 {
			let ny = y + row_offsets[y as usize];
			for x in 0 .. width as i64 {
				let nx = x + column_offsets[x as usize];
				if pixels[y as usize * width  + x as usize] {
					galaxies.push(Coord { x:nx as i64, y:ny as i64} );
				}
			}
		}
		Image { galaxies: galaxies }
	}
}

fn main() {
	println!("Hello Day 11 1!");

	let input = fs::read_to_string("inputs/day11").unwrap();

	let image = Image::from_text(&input);

	let mut sum = 0;
	for ga in 1 .. image.galaxies.len() {
		for gb in ga .. image.galaxies.len() {
			let ca = image.galaxies[ga];
			let cb = image.galaxies[gb];
			let dist = ca.x.abs_diff(cb.x) + ca.y.abs_diff(cb.y);
			sum += dist;
		}
	}

	println!("Sum: {sum}")
	// 382979724122
}