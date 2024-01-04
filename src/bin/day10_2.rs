use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
	Ground =0,
	Vertical =1,
	Horizontal =2,
	CornerNE =3,
	CornerNW =4,
	CornerSE =5,
	CornerSW =6,
	Start =7,
}

fn tile_from_char(c: char) -> Tile {
	match c {
		'|' => Tile::Vertical,
		'-' => Tile::Horizontal,
		'L' => Tile::CornerNE,
		'J' => Tile::CornerNW,
		'7' => Tile::CornerSW,
		'F' => Tile::CornerSE,
		'S' => Tile::Start,
		_ => Tile::Ground,
	}
}

fn find_value<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    for (index, item) in arr.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coord {
	x: i32,
	y: i32
}

impl Coord {
	fn at(x: i32, y: i32) -> Self { Self { x, y} }
	fn offset(&self, dx: i32, dy: i32) -> Self { Self { x: self.x+dx, y: self.y+dy} }
}

#[derive(Debug, Clone)]
struct Map {
	width: usize,
	height: usize,
	tiles: Vec<Tile>,
	start: Coord,
}

impl Map {
	fn from_text(text: &str) -> Map {

		// Firure width and height
		let (mut width, mut height) = (0,0);
		let mut tiles = vec![];

		for l in text.lines() {
			width = usize::max(width, l.len());
			height += 1;
			tiles.extend(l.chars().map(tile_from_char));
		}
		let start_offset = find_value(&tiles, &Tile::Start).unwrap();
		Map {width, height, tiles, start: Coord::at((start_offset % width) as i32 , (start_offset / width) as i32)  }
	}

	fn at(&self, p: Coord) -> Tile {
		if p.x < 0 || p.y < 0 || p.x >= self.width as i32 || p.y > self.height as i32 {
			Tile::Ground
		} else {
			self.tiles[p.y as usize * self.width + p.x as usize]
		}
	}

	fn neighbours(&self, p: Coord) -> Option<(Coord, Coord)> {
		match self.at(p) {
			Tile::Vertical => Some((p.offset(0, -1), p.offset(0, 1))),
			Tile::Horizontal => Some((p.offset(-1,0), p.offset(1, 0))),
			Tile::CornerNE => Some((p.offset(0, -1), p.offset(1, 0))),
			Tile::CornerNW => Some((p.offset(0, -1), p.offset(-1, 0))),
			Tile::CornerSE => Some((p.offset(0, 1), p.offset(1, 0))),
			Tile::CornerSW => Some((p.offset(0, 1), p.offset(-1, 0))),
			Tile::Ground => None,
			Tile::Start => Some((p, p)),
		}
	}

	fn is_connected(&self, a: Coord, b: Coord) -> bool {
		if let Some((n1,n2)) = self.neighbours(b) {
			n1 == a || n2 == a
		} else {
			false
		}
	}

	// Find next tile across from 'c', coming from 'f'
	fn next(&self, c: Coord, f:Coord) -> Option<Coord> {
		match self.neighbours(c) {
			Some((n1,n2)) if n1 == f => Some(n2),
			Some((n1,n2)) if n2 == f => Some(n1),
			Some((_,_))  => None,
			None => None
		}
	}

}

// Fugure signed area of rectangle bounded by left edge of map and the from given points
fn left_area(f:Coord, c:Coord) -> i32 {
	(f.y - c.y) *  c.x
}

fn main()
{
	println!("Hello Day 10 2!");

	let map = Map::from_text(&fs::read_to_string("inputs/day10").unwrap());
	println!("Start: {:?}", map.start);

	// Filter the four neighbours of start tile by connection
	// Should end up with two connected neighbours
	let start_neigbours = vec![
		map.start.offset(0,-1),
	    map.start.offset(0,1),
	    map.start.offset(-1,0),
	    map.start.offset(1,0)]
	    	.into_iter()
	    	.filter(|&c| map.is_connected(map.start, c))
	    	.collect::<Vec<_>>();
	assert_eq!(start_neigbours.len(), 2);

   	let mut from = map.start;
   	let mut current = start_neigbours[0];
   	let mut steps = 1;
   	let mut trail = vec!['.'; map.width * map.height];
   	let mut area = 1;

   	loop {
   		area += left_area(from, current);

	   	trail[map.width * current.y as usize + current.x as usize] = char::from_digit(steps % 16,16).unwrap();

   		if let Some(c) = map.next(current, from) {
   			from = current;
   			current = c;
   		} else {
   			eprintln!("No neighbour: {:?}", current);
   			break;
   		}
   		steps += 1;

		if current == map.start {
			break;
		}
   	}

	area += left_area(from, current);

   	for y in 0..map.height {
   		let line = trail[(y * map.width as usize)..((y+1) * map.width as usize)].iter().collect::<String>();
   		println!("{}", line);
   	}

   	steps /= 2;
   	area = area.abs() - steps as i32;
   	println!("Steps: {steps}");
   	// 6909
   	println!("Area: {area}");
   	// 461
}
