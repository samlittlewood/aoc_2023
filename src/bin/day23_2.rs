use std::cmp::Ordering;
use std::{collections::BinaryHeap, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    E,
    N,
    W,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Element {
    Path,
    Forest,
}

impl Element {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '>' | '^' | '<' | 'v' => Some(Element::Path),
            '.' => Some(Element::Path),
            '#' => Some(Element::Forest),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
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
    start: (u8, u8),
    end: (u8, u8),
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
                .find(|(_, &c)| c == Element::Path)
                .take()
            {
                if height == 0 {
                    start = (i as u8, 0);
                } else {
                    end = (i as u8, height as u8);
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
#[derive(Debug, Clone, PartialEq, Eq)]
struct Step {
    distance: u16,
    x: u8,
    y: u8,
    direction: Direction,
    prev: usize,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
//        other.distance.cmp(&self.distance)
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
    pending: BinaryHeap<(usize,usize)>,
    steps: Vec<Step>,
    longest: Vec<[u16;4]>
}

impl<'a> Pathfinder<'a> {
    fn create(map: &'a Map) -> Self {
        Pathfinder {
            map: &map,
            pending: BinaryHeap::new(),
            steps: Vec::new(),
            longest: vec![[0,0,0,0]; map.width * map.height]
        }
    }

    fn push_step(&mut self, x: i32, y: i32, direction: Direction, prev: usize, distance: u16) {

        if self.map.at(x, y) == Element::Path && !self.is_visited(x, y, prev) {
//            self.pending.push((usize::MAX - distance as usize, self.steps.len()));
            self.pending.push((distance as usize, self.steps.len()));
            self.steps.push(Step {
                x:x as u8,
                y:y as u8,
                distance,
                direction,
                prev,
            });
        }
    }

    fn is_visited(&mut self, x: i32, y: i32, mut step: usize) -> bool {
        while step != usize::MAX {
            if self.steps[step].x == x as u8 && self.steps[step].y == y as u8 {
                return true;
            }
            step = self.steps[step].prev;
        }
        false
    }

    fn find_paths(&mut self) -> Vec<u16> {
        self.pending.clear();
        let mut path_distances = Vec::new();
        let mut max_distance = 0;

        self.push_step(self.map.start.0 as i32, self.map.start.1 as i32, Direction::S, usize::MAX, 0);

        while let Some((step_dist, step_idx)) = self.pending.pop() {
            let Step { x , y, distance, direction, prev: _ } = self.steps[step_idx];

            if self.map.end.0 == x && self.map.end.1 == y {
                max_distance = u16::max(distance, max_distance);
                println!("Found: {} Max: {} Pending:{}", distance, max_distance, self.pending.len());
                path_distances.push(distance);
                continue;
            }

            let (sx,sy) = (x as i32, y as i32);

            // if self.longest[self.map.offset(sx, sy)][direction as usize] > distance {
            //      continue;
            // }
            // if self.longest[self.map.offset(sx, sy)][direction as usize] > 0 {
            //      continue;
            // }

            self.longest[self.map.offset(sx, sy)][direction as usize] = distance;

            self.push_step(sx+1, sy, Direction::E, step_idx, distance+1);
            self.push_step(sx, sy-1, Direction::N, step_idx, distance+1);
            self.push_step(sx-1, sy, Direction::W, step_idx, distance+1);
            self.push_step(sx, sy+1, Direction::S, step_idx, distance+1);
        }

        path_distances.sort();
        path_distances
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
        while let Some((_,step_idx)) = pending_copy.pop() {
            let step = &self.steps[step_idx];
            pending_map[self.map.offset(step.x as i32, step.y as i32)] = step.distance;
        }

        for y in 0..self.map.height as i32 {
            for x in 0..self.map.width as i32 {
                let e = self.map.at(x as i32, y as i32);

                if pending_map[self.map.offset(x, y)] > 0 {
                    print!("[{:2}]", pending_map[self.map.offset(x, y)]);
                } else {
                    print!("{}{}{} ", e.to_char(), e.to_char(), e.to_char());
                }
            }
            println!("");
        }
    }
}

struct Node {
    position: (u8,u8),
    edges:[u16;4]
}

struct Edge {
    cost: u16,
    nodes: (u16, u16)
}

struct Graph {
    nodes: Vec<Node>,
    nodes: Vec<Edge>,
}

impl Graph {
    fn create(map: &Map) -> Graph {
        let mut nodes = Vec::new();
        let mut pending = BinaryHeap::new();
        let mut visited = vec![false; map.width * map.height];

        // Start node
        nodes.push(Node { position: map.start, edges:[u16::MAX; 4] });
        visited[map.offset(map.start.0 as i32, map.start.1 as i32 )] = true;

        // Push first edge leaving start onto pending heap
        pending.push((0_u16, map.start.0, map.start.1+1, 0_u16));

        while let Some((cost,x,y,node)) = pending.pop() {

        }

        Graph { nodes }
    }
}

fn main() {
    println!("Hello Day 23 1!");

    let input = fs::read_to_string("inputs/day23_test").unwrap();

    let map = Map::from_string(&input);
    map.dump();

    let mut pathfinder = Pathfinder::create(&map);
    let distances = pathfinder.find_paths();
    println!("Distance: {:?}", distances);

    //    pathfinder.dump();
    //    let lowest_cost = map.find_path(0, 0, map.width as i32 -1, map.height as i32 -1);

    //	println!("Best loss: {}", lowest_cost);
}
