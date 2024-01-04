use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Element {
    Empty,
    Mirror1,
    Mirror2,
    SplitV,
    SplitH,
    Edge,
}

impl Element {
    fn from_char(c: char) -> Self {
        match c {
            '/' => Element::Mirror1,
            '\\' => Element::Mirror2,
            '|' => Element::SplitV,
            '-' => Element::SplitH,
            _ => Element::Empty,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Element::Empty => '.',
            Element::Mirror1 => '/',
            Element::Mirror2 => '\\',
            Element::SplitV => '|',
            Element::SplitH => '-',
            Element::Edge => '#',
        }
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    elements: Vec<Element>,
    directions: Vec<u8>,
}

impl Grid {
    fn create(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut elements = Vec::new();

        for l in input.lines() {
            width = usize::max(width, l.len());
            height += 1;
            let mut line_elements = l.chars().map(Element::from_char).collect::<Vec<_>>();
            elements.append(&mut line_elements);
        }

        Grid {
            width,
            height,
            elements,
            directions: vec![0; width * height],
        }
    }

    fn dump(&self) {
        println!("-- {}x{}", self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let e = self.at(x as i32, y as i32);
                let c = self.directions(x as i32, y as i32);
                if e == Element::Empty && c > 0 {
                    print!("{}", Self::direction_char(c))
                } else {
                    print!("{}", e.to_char());
                }
            }
            println!("");
        }
    }

    fn at(&self, x: i32, y: i32) -> Element {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.elements[y as usize * self.width + x as usize]
        } else {
            Element::Edge
        }
    }

    fn directions_mut(&mut self, x: i32, y: i32) -> &mut u8 {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.directions
                .get_mut(y as usize * self.width + x as usize)
                .unwrap()
        } else {
            panic!("Out of range")
        }
    }

    fn directions(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            *self
                .directions
                .get(y as usize * self.width + x as usize)
                .unwrap()
        } else {
            0xff
        }
    }

    fn direction_mask(dx: i32, dy: i32) -> u8 {
        match (dx, dy) {
            (1, 0) => 0x1,
            (-1, 0) => 0x2,
            (0, 1) => 0x4,
            (0, -1) => 0x8,
            _ => 0,
        }
    }

    fn direction_char(mask: u8) -> char {
        match mask {
            0x0 => '.',
            0x1 => '>',
            0x2 => '<',
            0x4 => 'v',
            0x8 => '^',
            _ => char::from_digit(mask.count_ones() % 16, 16).unwrap(),
        }
    }

    fn energised(&self) -> usize {
    	self.directions.iter().filter(|c| **c > 0).count()
    }

    fn trace(&mut self, mut x: i32, mut y: i32, mut dx: i32, mut dy: i32, mut mask: u8) {
        // Follow path until not empty
        loop {
            x += dx;
            y += dy;
            if self.directions(x, y) & mask != 0 {
                break;
            }
            *self.directions_mut(x, y) |= mask;
            match self.at(x, y) {
                Element::Edge => break,
                Element::Empty => (),
                Element::Mirror1 => { (dx, dy) = (-dy, -dx); mask = Self::direction_mask(dx, dy); },
                Element::Mirror2 => { (dx, dy) = (dy, dx); mask = Self::direction_mask(dx, dy); },
                Element::SplitH => {
                    if dx == 0 {
                        self.trace(x, y, 1, 0, Self::direction_mask(1, 0));
                        self.trace(x, y, -1, 0, Self::direction_mask(-1, 0));
                        break;
                    }
                }
                Element::SplitV => {
                    if dy == 0 {
                        self.trace(x, y, 0, 1, Self::direction_mask(0, 1));
                        self.trace(x, y, 0, -1, Self::direction_mask(0, -1));
                        break;
                    }
                }
            }
        }
    }
}

fn main() {
    println!("Hello Day 16 1!");

    let input = fs::read_to_string("inputs/day16").unwrap();

    let mut grid = Grid::create(&input);

    grid.trace(-1, 0, 1, 0, Grid::direction_mask(1, 0));

    grid.dump();
    println!("Energised: {}", grid.energised());
}
