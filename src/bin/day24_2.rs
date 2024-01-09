use std::fs;

#[derive(Debug, Clone)]
struct Particle {
	position: (i64, i64, i64),
	velocity: (i64, i64, i64)
}

impl Particle {
	fn intersect_xy(&self, other:&Self, low: i64, high: i64) -> Option<(f64, f64)> {
		let (p0x,p0y,v0x,v0y) = (self.position.0, self.position.1, self.velocity.0,  self.velocity.1);
		let (p1x,p1y,v1x,v1y) = (other.position.0, other.position.1, other.velocity.0,  other.velocity.1);

		let (dx,dy) = (p1x - p0x, p1y - p0y);
		let mut det = v1x * v0y - v1y * v0x;
		if det != 0 {
			let mut u_n = dy * v1x - dx * v1y;
			let v_n = dy * v0x - dx * v0y;

			if i64::signum(u_n) == i64::signum(det) && i64::signum(v_n) == i64::signum(det) {
				if det < 0 {
					det = -det;
					u_n = -u_n;
				}
				if p0x as i128 * det as i128 + u_n as i128 * v0x as i128 >= low as i128 * det as i128 &&
				   p0x as i128 * det as i128 + u_n as i128 * v0x as i128 <= high as i128 * det as i128 &&
				   p0y as i128 * det as i128 + u_n as i128 * v0y as i128 >= low as i128 * det as i128 &&
				   p0y as i128 * det as i128 + u_n as i128 * v0y as i128 <= high as i128 * det as i128 {
					return Some((p0x as f64 + (u_n as f64 * v0x as f64) / det as f64, p0y as f64 + (u_n  as f64 * v0y as f64) / det as f64))
				}
			}
		}
		return None;
	}
}

fn main() {
	println!("Hello Day 24 1!");

	// let input = fs::read_to_string("inputs/day24_test").unwrap();
	// let area_low = 7;
	// let area_high = 27;

	let input = fs::read_to_string("inputs/day24").unwrap();
	let area_low = 200000000000000;
	let area_high = 400000000000000;

	let mut particles = Vec::new();

	for l in input.lines() {
		let (ps,vs) = l.split_once('@').unwrap();
		let p = ps.split_terminator(',').map(|s| s.trim().parse::<i64>()).collect::<Result<Vec<_>,_>>().unwrap();
		let v = vs.split_terminator(',').map(|s| s.trim().parse::<i64>()).collect::<Result<Vec<_>,_>>().unwrap();
		particles.push(Particle { position: (p[0], p[1], p[2]), velocity: (v[0], v[1], v[2])});
	}

	let mut count = 0;
	for i in 0 .. particles.len() {
		for j in i+1 .. particles.len() {
			if let Some(_) = particles[i].intersect_xy(&particles[j], area_low, area_high) {
				count += 1;
			}
		}
	}

	println!("Collisions: {count}");
}