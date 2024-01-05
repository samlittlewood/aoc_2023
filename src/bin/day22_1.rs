use std::{fs, collections::BinaryHeap};

// 128 mask bits - can fit 10x10 xy dims
const MASK_WIDTH:i32 = 10;
const MASK_HEIGHT:i32 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn zero() -> Point {
        Point { x: 0, y: 0, z: 0 }
    }
    fn at(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }

    fn from_string(s: &str) -> Option<Point> {
        let r = s
            .split_terminator(',')
            .map(str::trim)
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>();

        match r {
            Ok(v) if v.len() == 3 => Some(Point {
                x: v[0],
                y: v[1],
                z: v[2],
            }),
            _ => None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    start: Point,
    end: Point,
    mask: u128,
}

fn brick_label(idx: usize) -> char {
	char::from_u32(65 + idx as u32).unwrap()
}

impl Brick {
    fn from_string(s: &str) -> Option<Brick> {

        let mut pts = s.split('~');
        let mut start = Point::from_string(pts.next()?)?;
        let mut end = Point::from_string(pts.next()?)?;

        // Make sure start z <= end z
        if start.z > end.z {
            (start, end) = (end, start)
        }

        Some(Brick { start, end, mask: Brick::make_mask(start, end) })
    }

    // Make a bitmask of brick's proejction onto xy plane
    fn make_mask(s: Point, e:Point) -> u128 {
    	let mut m = 0;
    	for y in i32::min(s.y,e.y) ..= i32::max(s.y,e.y) {
	    	for x in i32::min(s.x,e.x) ..= i32::max(s.x,e.x) {
	    		m |= 1 << (y * MASK_WIDTH + x) as usize;
	    	}
    	}

    	m
    }

    fn lowest(&self) -> i32 {
    	self.start.z
    }

    fn highest(&self) -> i32 {
    	self.end.z
    }
}

fn main() {
    println!("Hello Day 22 1!");

    let input = fs::read_to_string("inputs/day22").unwrap();

    let mut bricks = Vec::new();
    let mut height = 0;
	// Heap of brick indices sorted by -ve height
    let mut bricks_heap = BinaryHeap::new();

    for l in input.lines().filter(|s| !s.is_empty()) {
    	if let Some(b) = Brick::from_string(l) {
    		bricks_heap.push((-b.lowest(), bricks.len()));
    		height = i32::max(b.lowest(), height);

    		bricks.push(b);
    	}
    }

    // Bitmask of each settled level of pile - level 0 is solid
    let mut pile_masks = vec![0; height as usize];
    pile_masks[0] = ! 0  as u128;

    // Per layer vector of bricks that cross that layer
    let mut pile_bricks = vec![Vec::new(); height as usize];

    // Settle brick from lowest first
    while let Some((_, idx)) = bricks_heap.pop() {
    	let b = &bricks[idx];
    	let mut z = b.lowest() as usize;
    	// drop until supported
    	while b.mask & pile_masks[z -1] == 0 {
    		z -=1;
    	}
    	let delta = b.lowest() - z as i32;

    	// Add settled bricks's mask to the levels it covers
		for bz in (bricks[idx].start.z-delta) as usize ..= (bricks[idx].end.z-delta) as usize {
    		pile_masks[bz] |= b.mask;
    		pile_bricks[bz].push(idx);
    	}

    	if delta > 0 {
    		bricks[idx].start.z -= delta;
    		bricks[idx].end.z -= delta;
		}

    }

    let mut count = 0;
    // Find bricks that cannot be removed .. are any bricks in layer above only suppored
    // by this one?
    'outer: for (_i,b) in bricks.iter().enumerate() {
    	// Mask of support provided by all bricks at top level of this brick - without
    	// this brick
    	let suppport_mask = pile_masks[b.highest() as usize] & !b.mask;
    	for &ub in &pile_bricks[1 + b.highest() as usize] {
    		if bricks[ub].mask & suppport_mask == 0 {
				// This upper brick is only supported by candidate
    			continue 'outer;
    		}
    	}

    	// All upper bricks are supported without this one
    	count += 1;
    }

    println!("Removable: {count}");
}
