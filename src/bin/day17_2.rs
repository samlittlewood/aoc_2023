use std::{cmp::Ordering, collections::BinaryHeap, collections::HashMap, fs};

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    losses: Vec<u8>,
    distance: Vec<[usize; 64]>,
    pending: BinaryHeap<Step>,
    pending_set: HashMap<(i32, i32, u8, u8), usize>,
}

// One step in grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Step {
    x: i32,
    y: i32,
    cost: usize,
    steps: u8,
    direction: u8,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort based on cost, then position, steps & direction
        other
            .cost
            .cmp(&self.cost)
            .then(other.x.cmp(&self.x))
            .then(other.y.cmp(&self.y))
            .then(other.steps.cmp(&self.steps))
            .then(other.direction.cmp(&self.direction))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Map {
    fn create(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut losses = Vec::new();

        for l in input.lines() {
            width = usize::max(width, l.len());
            height += 1;
            let mut line_elements = l
                .chars()
                .map(|c| char::to_digit(c, 16).unwrap() as u8)
                .collect::<Vec<_>>();
            losses.append(&mut line_elements);
        }

        Map {
            width,
            height,
            losses,
            distance: vec![[usize::MAX; 64]; width * height],
            pending: BinaryHeap::new(),
            pending_set: HashMap::new(),
        }
    }

    fn at(&self, x: i32, y: i32) -> Option<u8> {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            Some(self.losses[y as usize * self.width + x as usize])
        } else {
            None
        }
    }

    fn distance(&self, x: i32, y: i32, d: u8, s: u8) -> usize {
        self.distance
            .get(y as usize * self.width + x as usize)
            .unwrap()[(d * 16 + s) as usize]
    }

    fn distance_set(&mut self, x: i32, y: i32, d: u8, s: u8, dist: usize) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.distance
                .get_mut(y as usize * self.width + x as usize)
                .unwrap()[(d * 16 + s) as usize] = dist;
        } else {
            panic!("Out of range")
        }
    }

    fn forward(x: i32, y: i32, direction: u8) -> (i32, i32) {
        match direction {
            0 => (x + 1, y),
            1 => (x, y + 1),
            2 => (x - 1, y),
            3 => (x, y - 1),
            _ => panic!("Bad direction"),
        }
    }

    fn left(direction: u8) -> u8 {
        (direction + 3) & 3
    }
    fn right(direction: u8) -> u8 {
        (direction + 1) & 3
    }

    fn push_step(&mut self, x: i32, y: i32, direction: u8, steps: u8, current_cost: usize) {
        if let Some(c) = self.at(x, y) {
            let cost = current_cost + c as usize;

            if let Some(&prev_cost) = self.pending_set.get(&(x, y, steps, direction)) {
                if cost < prev_cost {
                    *self.pending_set.get_mut(&(x, y, steps, direction)).unwrap() = cost;
                }
                return;
            }
            let distance = self.distance(x, y, direction, steps);
            if cost < distance {
                self.pending.push(Step {
                    x,
                    y,
                    cost,
                    steps,
                    direction,
                });
                self.pending_set.insert((x, y, steps, direction), cost);
            }
        }
    }

    fn find_path(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> usize {
        self.pending.clear();
        self.pending_set.clear();

        // Add neighbours around start
        self.push_step(start_x + 1, start_y, 0, 1, 0);
        self.push_step(start_x, start_y + 1, 1, 1, 0);
        self.push_step(start_x - 1, start_y, 2, 1, 0);
        self.push_step(start_x, start_y - 1, 3, 1, 0);

        let mut lowest_cost = 0;

        while let Some(step) = self.pending.pop() {
            self.pending_set
                .remove(&(step.x, step.y, step.steps, step.direction));
            if step.x == end_x && step.y == end_y && step.steps > 3 {
                lowest_cost = step.cost;
                break;
            }

            self.distance_set(step.x, step.y, step.direction, step.steps, step.cost);

            // Add possible next steps
            //   Forward
            if step.steps < 10 {
                let direction = step.direction;
                let (x, y) = Self::forward(step.x, step.y, step.direction);
                self.push_step(x, y, direction, step.steps + 1, step.cost);
            }

            if step.steps > 3 {
                //   Left
                {
                    let direction = Self::left(step.direction);
                    let (x, y) = Self::forward(step.x, step.y, direction);
                    self.push_step(x, y, direction, 1, step.cost);
                }

                //   Right
                {
                    let direction = Self::right(step.direction);
                    let (x, y) = Self::forward(step.x, step.y, direction);
                    self.push_step(x, y, direction, 1, step.cost);
                }
            }
        }

        lowest_cost
    }
}

fn main() {
    println!("Hello Day 17 2!");

    let input = fs::read_to_string("inputs/day17").unwrap();

    let mut map = Map::create(&input);

    let lowest_cost = map.find_path(0, 0, map.width as i32 - 1, map.height as i32 - 1);

    println!("Best loss: {}", lowest_cost);
    // 801
}
