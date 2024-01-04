use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

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
        println!("-- {} x {}",self.width, self.height);
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
    		r.extend_from_slice(&self.pixels[y * self.width .. (y+1) * self.width]);
    		output.push(r)
    	}

    	output
    }

    fn columns(&self) -> Vec<Vec<bool>> {
    	let mut output = Vec::new();
    	for x in 0..self.width {
    		let mut r = Vec::new();
    		for y in 0 .. self.height {
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

fn find_reflection(lines: &[Vec<bool>]) -> Option<usize> {
	// For each possible split ...
	let mut output = 0;
	let mut count = 0;
	for refl in 1..lines.len() {
		let distance = usize::min(refl, lines.len()-refl);
		let b = (0..distance).map(|i| lines[refl-i-1] == lines[refl+i]).all(|b| b);
//		println!("  refl:{refl} dist:{distance} b:{b}");
		if b {
			output =refl;
			count += 1;
		}
	}

	if count > 1 {
		println!("More than 1 match: {}", count);
	}

	if count > 0 {
		Some(output)
	} else {
		None
	}
}

fn main() {
    println!("Hello Day 13 1!");

    //	let p = "".to_string();

    let mut lines = read_lines("inputs/day13_test1").unwrap();
    let mut patterns = Vec::new();

    loop {
        let pattern_lines = lines
            .by_ref()
            .take_while(|l| if let Ok(t) = l { !t.is_empty() } else { false })
            .map(Result::unwrap).collect::<Vec<_>>();
        if pattern_lines.is_empty() {
        	break;
        }
		let pattern = Pattern::create(&pattern_lines);
        patterns.push(pattern);
    }

    let mut sum =0;
    for p in &patterns {
//		p.dump();
		let vreflect = find_reflection(&p.columns());
		let hreflect = find_reflection(&p.rows());
		println!("h: {hreflect:?} v:{vreflect:?}");
		if let Some(r) = vreflect {
			sum += r;
		}
		if let Some(r) = hreflect {
			sum += r * 100;
		}
    }

    println!("Sum: {sum}");
}
