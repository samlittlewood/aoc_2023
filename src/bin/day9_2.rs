use std::fs;

fn main()
{
	println!("Hello Day 9 2!");

	let input = fs::read_to_string("inputs/day9").unwrap();
	let mut sum = 0;
	for l in input.lines() {
		let mut m = parse_measurements(l);
		m.reverse();
		let next = find_next(&m);
		println!("{m:?} {next}");
		sum += next;
	}

	println!("Sum: {sum}")
}

fn parse_measurements(text:&str) -> Vec<i64>
{
	text.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect()
}

fn find_next(measurements: &[i64]) -> i64 {
	let l = measurements.len();
	let mut deltas = vec![0; l * l];
	for n in 0..l {
		deltas[n] = measurements[n]
	}
	let mut next = measurements[l-1];

	for m in 1..(l-1) {
		for n in 0..(l-m) {
			deltas[m*l+n] = deltas[(m-1)*l+n+1] - deltas[(m-1)*l+n];
		}
		next += deltas[m*l+(l-m-1)];
	}

	next
}