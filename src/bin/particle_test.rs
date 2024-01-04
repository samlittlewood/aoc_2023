struct Particle {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl Particle {
    // fn collides(&self, other: &Particle) -> Option<(i64, i64)> {
    //     // Calculate the relative velocity between the two particles
    //     let dx = self.x - other.x;
    //     let dy = self.y - other.y;
    //     let dvx = self.vx - other.vx;
    //     let dvy = self.vy - other.vy;

    //     // Calculate the coefficients for the quadratic equation for collision time
    //     let a = dvx * dvx + dvy * dvy;
    //     let b = 2.0 * (dx * dvx + dy * dvy);
    //     let c = dx * dx + dy * dy - 1.0; // Assuming particle radius is 1

    //     // Calculate the discriminant
    //     let discriminant = b * b - 4.0 * a * c;

    //     if discriminant >= 0.0 {
    //         // Calculate the collision time
    //         let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    //         let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    //         if t1 >= 0.0 && t1 <= 1.0 {
    //             // Collision occurs at t1
    //             let collision_x = self.x + self.vx * t1;
    //             let collision_y = self.y + self.vy * t1;
    //             return Some((collision_x, collision_y));
    //         } else if t2 >= 0.0 && t2 <= 1.0 {
    //             // Collision occurs at t2
    //             let collision_x = self.x + self.vx * t2;
    //             let collision_y = self.y + self.vy * t2;
    //             return Some((collision_x, collision_y));
    //         }
    //     }

    //     // No collision detected
    //     None
    // }

    fn collides(&self, other: &Particle) -> Option<(i64, i64)> {
    }
}

fn main() {
    let particle1 = Particle {
        x: 0,
        y: 0,
        vx: 1,
        vy: 0,
    };
    let particle2 = Particle {
        x: 3,
        y: 0,
        vx: -1,
        vy: 0,
    };

    if let Some((collision_x, collision_y)) = particle1.collides(&particle2) {
        println!("Collision detected at ({}, {})", collision_x, collision_y);
    } else {
        println!("No collision detected.");
    }
}
