use std::fs;

#[derive(Debug, Clone)]
struct Particle {
	position: [i64; 3],
	velocity: [i64; 3]
}

impl Particle {

	fn intersect_2d(&self, other:&Self, axes:(usize,usize), vxy:(i64,i64))  -> Option<(i64, i64, i64)> {
		let (p0x,p0y,v0x,v0y) = (self.position[axes.0], self.position[axes.1], self.velocity[axes.0]-vxy.0,  self.velocity[axes.1]-vxy.1);
		let (p1x,p1y,v1x,v1y) = (other.position[axes.0], other.position[axes.1], other.velocity[axes.0]-vxy.0,  other.velocity[axes.1]-vxy.1);

		let (dx,dy) = (p1x - p0x, p1y - p0y);
		let det = v0x * v1y - v0y * v1x;
		if det != 0 {
			let t0_n = dx * v1y - dy * v1x;
			let t1_n = dx * v0y - dy * v0x;

			if i64::signum(t0_n) == i64::signum(det) && i64::signum(t1_n) == i64::signum(det) {
				return Some((t0_n, t1_n, det))
			}
		}
		return None;
	}

	fn at(&self, t_n: i64, t_d: i64) -> Option<[i64;3]> {
		if (t_n as i128 * self.velocity[0] as i128 % t_d as i128) != 0 ||
		   (t_n as i128 * self.velocity[1] as i128 % t_d as i128) != 0 ||
		   (t_n as i128 * self.velocity[2] as i128 % t_d as i128) != 0 {
		   	None
		} else {
			Some([
				self.position[0] + (t_n as i128 * self.velocity[0] as i128 / t_d as i128) as i64,
				self.position[1] + (t_n as i128 * self.velocity[1] as i128 / t_d as i128) as i64,
				self.position[2] + (t_n as i128 * self.velocity[2] as i128 / t_d as i128) as i64])
		}
	}
}

fn spiral(n: i64) -> (i64, i64) {
    if n == 0 {
        return (0, 0);
    }

    // Calculate the layer (ring) of the spiral containing n
    let ring = ((n as f64).sqrt() as i64 -1)/2 + 1;

    // Calculate the position within the current layer
    let edge = 2 * ring;
    let ring_start = ( 2*(ring-1) + 1) * (2*(ring-1) + 1);
    let offset_in_ring = n - ring_start;
    let offset_in_edge = offset_in_ring % edge;

    match offset_in_ring / edge {
    0 => (ring - offset_in_edge, ring),
    1 => (-ring, ring -offset_in_edge),
    2 => (-ring + offset_in_edge, -ring),
	3 => (ring, -ring +offset_in_edge),
	_ => (0,0),
    }
}

// Search for a velocity that makes all particles intersect at same point when projected onto 2d plane
//
fn find_velocity_2d(particles: &[Particle], axes: (usize, usize)) -> Option<((i64,i64),i64)> {
	// Spiral outwards for x,y velocities from 0,0
	'outer: for n in  0..1000000 {
		let vxy = spiral(n);
		let mut point = [0,0,0];
		let mut t = 0;

		// Intersect first two particles - must intersect, and hit integer coords.
		if let Some((t0,_t1,det)) = particles[0].intersect_2d(&particles[1], axes, vxy) {

			if let Some(pt) = particles[0].at(t0, det) {
				point = pt;
			}
		} else {
			continue;
		}

		// Check if all remaining particles intersect at same point
		for j in 2 .. particles.len() {
			if let Some((t0,_t1,det)) = particles[0].intersect_2d(&particles[j], axes, vxy) {
				if let Some(pt) = particles[0].at(t0, det) {
					if point != pt {
						continue 'outer;
					}
					t = t0 / det;
				} else {
					continue 'outer;
				}
			} else {
				continue 'outer;
			}
		}
		println!("Found point at {vxy:?}: {point:?}");
		return Some(((vxy), t));
	}
	None
}

fn main() {
	println!("Hello Day 24 2!");

	let input = fs::read_to_string("inputs/day24").unwrap();
	let mut particles = Vec::new();

	for l in input.lines() {
		let (ps,vs) = l.split_once('@').unwrap();
		let p = ps.split_terminator(',').map(|s| s.trim().parse::<i64>()).collect::<Result<Vec<_>,_>>().unwrap();
		let v = vs.split_terminator(',').map(|s| s.trim().parse::<i64>()).collect::<Result<Vec<_>,_>>().unwrap();
		particles.push(Particle { position:[p[0],p[1],p[2]], velocity:[v[0],v[1],v[2]] });
	}

	// Relative to the moving rock, all particles will intersect same point
	// Search for velocities in each plane that make all particles intersect a common point
	// From that, and t of the intersection for p0, figure the starting point.

	if let Some((vxy, txy)) = find_velocity_2d(&particles, (0,1)) {
		if let Some((vyz, _tyz)) = find_velocity_2d(&particles, (1,2)) {
			if let Some((_vzx, _tzx)) = find_velocity_2d(&particles, (2,0)) {
				let vel = [vxy.0, vxy.1, vyz.1];
				let col0 = particles[0].at(txy, 1).unwrap();
				let pos = [col0[0] - txy * vel[0], col0[1] - txy * vel[1], col0[2] - txy * vel[2] ];
				println!("vel:{vel:?} pos:{pos:?} sum:{}", pos[0] + pos[1] + pos[2]);
				// 568386357876600
			}
		}
	}

}
