use std::{fs, collections::BTreeSet};

#[derive(Debug, Clone)]
struct Particle {
	position: [i64; 3],
	velocity: [i64; 3]
}

impl Particle {

	fn intersect_xy(&self, other:&Self, vx:i64, vy:i64) -> Option<(i128, i128, i64)> {
		let (p0x,p0y,v0x,v0y) = (self.position[0], self.position[1], self.velocity[0],  self.velocity[1]);
		let (p1x,p1y,v1x,v1y) = (other.position[0], other.position[1], other.velocity[0],  other.velocity[1]);

		let (dx,dy) = (p1x - p0x, p1y - p0y);
		let mut det = v1x * v0y - v1y * v0x;
		if det != 0 {
			let mut u_n = dy * v1x - dx * v1y;
			let mut v_n = dy * v0x - dx * v0y;

			if i64::signum(u_n) == i64::signum(det) && i64::signum(v_n) == i64::signum(det) {
				if det < 0 {
					det = -det;
					u_n = -u_n;
				}
				return Some((p0x as i128 + (u_n as i128 * v0x as i128), p0y as i128 + (u_n as i128 * v0y as i128), det))
			}
		}
		return None;
	}
}

fn spiral(n: i32) -> (i32, i32) {
    if n == 0 {
        return (0, 0);
    }

    // Calculate the layer (ring) of the spiral containing n
    let k = (((n - 1) as f64).sqrt() as i32 + 1) / 2;

    // Calculate the position within the current layer
    let t = 2 * k;
    let m = (t + 1) * (t + 1);
    let m_1 = m - t;

    if n <= m_1 {
        let x = k;
        let y = k - (m_1 - n);
        (x, y)
    } else if n <= m_1 + t {
        let x = k - (n - m_1);
        let y = -k;
        (x, y)
    } else if n <= m_1 + 2 * t {
        let x = -k;
        let y = -k + (n - (m_1 + t));
        (x, y)
    } else {
        let x = -k + (n - (m_1 + 2 * t));
        let y = k;
        (x, y)
    }
}
fn main() {
	println!("Hello Day 24 1!");

	// let input = fs::read_to_string("inputs/day24_test").unwrap();

	let input = fs::read_to_string("inputs/day24").unwrap();
	let mut particles = Vec::new();

	for l in input.lines() {
		let (ps,vs) = l.split_once('@').unwrap();
		let p = ps.split_terminator(',').map(|s| s.trim().parse::<i64>()).collect::<Result<Vec<_>,_>>().unwrap();
		let v = vs.split_terminator(',').map(|s| s.trim().parse::<i64>()).collect::<Result<Vec<_>,_>>().unwrap();
		particles.push(Particle { position:[p[0],p[1],p[2]], velocity:[v[0],v[1],v[2]] });
	}


	let mut count = 0;
	for i in 0 .. particles.len() {
		for j in i+1 .. particles.len() {
			if let Some((nx,ny,det)) = particles[i].intersect_xy(&particles[j], 0, 0) {
//				println!("Collision: {i} {j} {cx} {cy}");
				count += 1;
			}
		}
	}

	println!("Collisions: {count}");
}