use std::{collections::BinaryHeap, fs};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    E,
    N,
    W,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Element {
    Slope(Direction),
    Path,
    Forest,
}

impl Element {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '>' => Some(Element::Slope(Direction::E)),
            '^' => Some(Element::Slope(Direction::N)),
            '<' => Some(Element::Slope(Direction::W)),
            'v' => Some(Element::Slope(Direction::S)),
            '.' => Some(Element::Path),
            '#' => Some(Element::Forest),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Element::Slope(Direction::E) => '>',
            Element::Slope(Direction::N) => '^',
            Element::Slope(Direction::W) => '<',
            Element::Slope(Direction::S) => 'v',
            Element::Path => '.',
            Element::Forest => '#',
        }
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    elements: Vec<Element>,
    start: (i32, i32),
    end: (i32, i32),
}

impl Map {
    fn from_string(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut elements = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);

        for l in input.lines() {
            let mut line_elements = l
                .chars()
                .map(Element::from_char)
                .collect::<Option<Vec<_>>>()
                .unwrap();

            if let Some((i, _)) = line_elements
                .iter()
                .enumerate()
                .find(|(i, &c)| c == Element::Path)
                .take()
            {
                if height == 0 {
                    start = (i as i32, 0);
                } else {
                    end = (i as i32, height as i32);
                }
            }

            elements.append(&mut line_elements);

            width = usize::max(width, l.len());
            height += 1;
        }

        Map {
            width,
            height,
            elements,
            start,
            end,
        }
    }

    fn dump(&self) {
        println!(
            "-- {} x {}  Start: {},{} End {},{}",
            self.width, self.height, self.start.0, self.start.1, self.end.0, self.end.1
        );
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.at(x as i32, y as i32).to_char());
            }
            println!("");
        }
    }
    fn at(&self, x: i32, y: i32) -> Element {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.elements[y as usize * self.width + x as usize]
        } else {
            Element::Forest
        }
    }

    fn offset(&self, x: i32, y: i32) -> usize {
        assert!(x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32);
        y as usize * self.width + x as usize
    }
}

// One step in grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Step {
    distance: i32,
    x: i32,
    y: i32,
    direction: Direction,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug)]
struct Pathfinder<'a> {
    map: &'a Map,
    pending: BinaryHeap<Step>,
    path: Vec<i32>,
}

impl<'a> Pathfinder<'a> {
    fn create(map: &'a Map) -> Self {
        Pathfinder {
            map: &map,
            pending: BinaryHeap::new(),
            path: vec![0; map.width * map.height],
        }
    }

    fn push_step(&mut self, x: i32, y: i32, distance: i32, direction: Direction) {
        self.pending.push(Step {
            x,
            y,
            distance,
            direction,
        })
    }

    fn find_paths(&mut self) -> Vec<i32> {
        self.pending.clear();
        let mut paths_lengths = Vec::new();

        // Add start point
        self.push_step(self.map.start.0, self.map.start.1, 0, Direction::S);

        while let Some(step) = self.pending.pop() {
            self.path[self.map.offset(step.x, step.y)] = step.distance;

            if self.map.end.0 == step.x && self.map.end.1 == step.y {
//                println!("Found: {}", step.distance);
                paths_lengths.push(step.distance);
                continue;
            }
            let e = self.map.at(step.x, step.y);

            // E
            if step.direction != Direction::W
                && (e == Element::Path || e == Element::Slope(Direction::E))
            {
                match self.map.at(step.x + 1, step.y) {
                    Element::Path | Element::Slope(_) => {
                        self.push_step(step.x + 1, step.y, step.distance + 1, Direction::E)
                    }
                    _ => (),
                }
            }
            // N
            if step.direction != Direction::S
                && (e == Element::Path || e == Element::Slope(Direction::N))
            {
                match self.map.at(step.x, step.y - 1) {
                    Element::Path | Element::Slope(_) => {
                        self.push_step(step.x, step.y - 1, step.distance + 1, Direction::N)
                    }
                    _ => (),
                }
            }
            // W
            if step.direction != Direction::E
                && (e == Element::Path || e == Element::Slope(Direction::W))
            {
                match self.map.at(step.x - 1, step.y) {
                    Element::Path | Element::Slope(_) => {
                        self.push_step(step.x - 1, step.y, step.distance + 1, Direction::W)
                    }
                    _ => (),
                }
            }
            // S
            if step.direction != Direction::N
                && (e == Element::Path || e == Element::Slope(Direction::S))
            {
                match self.map.at(step.x, step.y + 1) {
                    Element::Path | Element::Slope(_) => {
                        self.push_step(step.x, step.y + 1, step.distance + 1, Direction::S)
                    }
                    _ => (),
                }
            }
        }

        paths_lengths.sort();
        paths_lengths
    }

    fn dump(&self) {
        println!(
            "-- {} x {}  Start: {},{} End {},{}",
            self.map.width,
            self.map.height,
            self.map.start.0,
            self.map.start.1,
            self.map.end.0,
            self.map.end.1
        );

        let mut pending_map = vec![0; self.map.width * self.map.height];
        let mut pending_copy = self.pending.clone();
        while let Some(s) = pending_copy.pop() {
            pending_map[self.map.offset(s.x, s.y)] = s.distance;
        }

        for y in 0..self.map.height as i32 {
            for x in 0..self.map.width as i32 {
                let e = self.map.at(x as i32, y as i32);

                if pending_map[self.map.offset(x, y)] > 0 {
                    print!("[{:2}]", pending_map[self.map.offset(x, y)]);
                } else {
                    if self.path[self.map.offset(x, y)] != 0 {
                        print!("{}{:2} ", e.to_char(), self.path[self.map.offset(x, y)]);
                    } else {
                        print!("{}{}{} ", e.to_char(), e.to_char(), e.to_char());
                    }
                }
            }
            println!("");
        }
    }
}

fn main() {
    println!("Hello Day 23 1!");

    let input = fs::read_to_string("inputs/day23").unwrap();

    let mut map = Map::from_string(&input);
//    map.dump();

    let mut pathfinder = Pathfinder::create(&map);
    let distances = pathfinder.find_paths();
    println!("Distance: {:?}", distances);

//    pathfinder.dump();
    //    let lowest_cost = map.find_path(0, 0, map.width as i32 -1, map.height as i32 -1);

    //	println!("Best loss: {}", lowest_cost);
}
