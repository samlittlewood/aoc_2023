use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::iter::zip;


#[derive(Debug)]
struct Pattern {
    width: usize,
    height: usize,
    pixels: Vec<bool>,
}

impl Pattern {
    fn create(text_lines: &[String]) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut pixels = Vec::new();

        for l in text_lines {
            width = width.max(l.len());
            height += 1;
            let mut line = l.chars().map(|c| c == '#').collect::<Vec<_>>();
            pixels.append(&mut line);
        }

        Pattern {
            width,
            height,
            pixels,
        }
    }

    fn dump(&self) {
        println!("-- {} x {}", self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "{}",
                    if self.pixels[y * self.width + x] {
                        '#'
                    } else {
                        '.'
                    }
                )
            }
            println!(" {y}");
        }
    }

    fn rows(&self) -> Vec<Vec<bool>> {
        let mut output = Vec::new();
        for y in 0..self.height {
            let mut r = Vec::new();
            r.extend_from_slice(&self.pixels[y * self.width..(y + 1) * self.width]);
            output.push(r)
        }

        output
    }

    fn columns(&self) -> Vec<Vec<bool>> {
        let mut output = Vec::new();
        for x in 0..self.width {
            let mut r = Vec::new();
            for y in 0..self.height {
                r.push(self.pixels[y * self.width + x])
            }
            output.push(r)
        }

        output
    }
}

fn read_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn bit_errors(a: &[bool], b: &[bool]) -> usize {
    zip(a, b).map(|(a, b)| if a != b { 1 } else { 0 }).sum()
}

fn find_reflection(lines: &[Vec<bool>], smudges: usize) -> Option<usize> {
    // For each possible split ...
    for refl in 1..lines.len() {
        let distance = usize::min(refl, lines.len() - refl);
        // Figure total bit errors over reflection
        let total_bit_errors: usize = (0..distance)
            .map(|i| bit_errors(&lines[refl - i - 1], &lines[refl + i]))
            .sum();
        if total_bit_errors == smudges {
            return Some(refl);
        }
    }

    None
}

fn main() {
    println!("Hello Day 13 2!");

    //	let p = "".to_string();

    let mut lines = read_lines("inputs/day13").unwrap();
    let mut patterns = Vec::new();

    loop {
        let pattern_lines = lines
            .by_ref()
            .take_while(|l| if let Ok(t) = l { !t.is_empty() } else { false })
            .map(Result::unwrap)
            .collect::<Vec<_>>();
        if pattern_lines.is_empty() {
            break;
        }
        let pattern = Pattern::create(&pattern_lines);
        patterns.push(pattern);
    }

    let mut sum = 0;
    let smudges = 1; // 0 for Part 1, 1 for Part 2
    for p in &patterns {
        //		p.dump();
        let vreflect = find_reflection(&p.columns(), smudges);
        let hreflect = find_reflection(&p.rows(), smudges);
        println!("h: {hreflect:?} v:{vreflect:?}");
        if let Some(r) = vreflect {
            sum += r;
        }
        if let Some(r) = hreflect {
            sum += r * 100;
        }
    }

    println!("Sum: {sum}");
    // 41566
}

#[test]
fn test1() {
    assert_eq!(
        bit_errors(
            &vec![true, false, false, true],
            &vec![true, false, false, true]
        ),
        0
    );
    assert_eq!(
        bit_errors(
            &vec![true, false, false, true],
            &vec![true, false, false, false]
        ),
        1
    );
    assert_eq!(
        bit_errors(
            &vec![true, false, false, true],
            &vec![false, true, true, false]
        ),
        4
    );
}
