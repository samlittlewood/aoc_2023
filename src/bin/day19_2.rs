use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashMap;


fn read_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn variable_to_index(v: char) -> usize {
    match v {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!("Bad var:{v}")
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Less(usize, usize),
    Greater(usize, usize),
    Always,
    Accept,
    Reject
}

impl Operation {
    fn from_string(s:&str) -> Self {
        let mut ci = s.chars();
        if let Some(var) = ci.next() {
            if let Some(op) = ci.next() {
                if let Ok(value) = ci.collect::<String>().parse::<usize>() {
                    return match op {
                        '<' => Operation::Less(variable_to_index(var), value),
                        '>' => Operation::Greater(variable_to_index(var), value),
                        _ => panic!("Bad op: {op}")
                    }
                }
            }
        }
        return Operation::Always;
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    vars: [usize;4]
}

impl Part {
    fn from_string(s:&str) -> Self {

        let mut r = Part { vars: [0;4] };

        let parens : &[_] = &['{','}'];

        for v in s.trim_matches(parens).split(',') {
            let va = v.split('=').collect::<Vec<_>>();
            let var = variable_to_index(va[0].chars().next().unwrap());
            let value = va[1].parse::<usize>().unwrap();
            r.vars[var] = value;
        }

        r
    }

    fn sum(&self) -> usize {
        return self.vars.iter().sum();
    }
}

#[derive(Debug, Clone, Copy)]
struct PartRange {
    min: [usize;4],
    max: [usize;4]
}

impl PartRange {
    fn full_range() -> PartRange {
        PartRange { min: [1;4], max: [4000;4] }
    }

    fn combinations(&self) -> usize {
        let mut c = 1;
        for i in 0..4 {
            c *= (self.max[i]+1) - self.min[i];
        }
        c
    }
}

struct Process {
    steps: Vec<(Operation, usize)>,
    symbols : HashMap<String, usize>,
    entry: usize
}

impl Process {
    fn assemble(source: &[String]) -> Self {
        let mut steps = Vec::new();
        let mut symbols : HashMap<_, usize> = HashMap::new();

        symbols.insert("A".to_string(), 0);
        symbols.insert("R".to_string(), 1);

        // Two passes to resolve forward refs.
        for pass in 0..2 {
            // Rest output
            steps = vec![(Operation::Accept,0),(Operation::Reject,0)];
            for rl in source {
                let name_and_rules = rl.split_terminator(|c| c == '{' || c=='}').collect::<Vec<_>>();
                let name =  name_and_rules[0];
                let rules = name_and_rules[1].split_terminator(',').collect::<Vec<_>>();

                symbols.insert(name.to_string(), steps.len());
                for rule in rules {
                    let rc = rule.split(':').collect::<Vec<_>>();
                    let (op,sym) = if rc.len() == 2 {
                        (Operation::from_string(rc[0]), rc[1])
                    } else {
                        (Operation::Always, rc[0])
                    };
                    let next = *symbols.get(sym).unwrap_or(&usize::MAX);
                    if pass == 1 && next == usize::MAX {
                        println!("Unknown symbol: {sym}")
                    }
                    steps.push((op, next));
                }
            }

        }
        // Lookup entry point
        let entry = *symbols.get("in").unwrap();

        Process { steps, symbols, entry }
     }

    fn add_combinations(&self, entry: usize, range: PartRange, combinations: usize) -> usize {
        let mut step = entry;

        loop {
            let (op, next) = &self.steps[step];
            step = match op {
                Operation::Less(var, value) =>
                    if range.min[*var] < *value {
                        if range.max[*var] >= *value {
                            // Range covers decision value - sum both branches with reduced range
                            let mut range_in = range;
                            let mut range_out = range;
                            range_in.max[*var] = *value-1;
                            range_out.min[*var] = *value;
                            let c = self.add_combinations(*next, range_in, combinations);
                            return self.add_combinations(step+1, range_out, c); // tail recursion
                        } else {
                            *next
                        }
                    } else {
                        step+1
                    }

                Operation::Greater(var, value) =>
                    if range.max[*var] > *value {
                        if range.min[*var]  <= *value {
                            // Range covers decision value - sum both branches with reduced range
                            let mut range_in = range;
                            let mut range_out = range;
                            range_in.min[*var] = *value+1;
                            range_out.max[*var] = *value;
                            let c = self.add_combinations(*next, range_in, combinations);
                            return self.add_combinations(step+1, range_out, c); // tail recursion
                        } else {
                            *next
                        }
                    } else {
                        step+1
                    }

                Operation::Always => *next,
                Operation::Reject => return combinations,
                Operation::Accept => return combinations + range.combinations()
            }
        }
    }
}

fn main() {
    println!("Hello Day 19 2!");

    let mut lines = read_lines("inputs/day19").unwrap();

    let mut rule_lines = Vec::new();
    while let Some(Ok(l)) = lines.next() {
        if l.is_empty() {
            break;
        }
        rule_lines.push(l);
    }

    let process = Process::assemble(&rule_lines);

    let range = PartRange::full_range();
    let combinations = process.add_combinations(process.entry, range, 0);

    println!("Total: {combinations}");
    // 131619440296497
}
