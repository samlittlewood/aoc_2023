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

fn main() {
    println!("Hello Day 19 1!");

    //	let p = "".to_string();

    let mut lines = read_lines("inputs/day19").unwrap();

    let mut rule_lines = Vec::new();
    while let Some(Ok(l)) = lines.next() {
        if l.is_empty() {
            break;
        }
//        println!("Rule: {l}");
        rule_lines.push(l);
    }

    // Build workflows as a single array of operations with a symbol table
    let mut workflow_syms : HashMap<_, usize> = HashMap::new();
    let mut workflow = Vec::new();

    let idx = workflow_syms.insert("A", 0);
    let idx = workflow_syms.insert("R", 1);

    // Two passes to resolve forward refs.
    for pass in 0..2 {
        // Rest output
        workflow = vec![(Operation::Accept,0),(Operation::Reject,0)];
        for rl in &rule_lines {
            let name_and_rules = rl.split_terminator(|c| c == '{' || c=='}').collect::<Vec<_>>();
            let name =  name_and_rules[0];
            let rules = name_and_rules[1].split_terminator(',').collect::<Vec<_>>();
//            println!("{name:?} {rules:?}");

            let idx = workflow_syms.insert(name, workflow.len());
            for rule in rules {
                let rc = rule.split(':').collect::<Vec<_>>();
                let (op,sym) = if rc.len() == 2 {
                    (Operation::from_string(rc[0]), rc[1])
                } else {
                    (Operation::Always, rc[0])
                };
                let next = *workflow_syms.get(sym).unwrap_or(&usize::MAX);
                if(pass == 1 && next == usize::MAX) {
                    println!("Unknown symbol: {sym}")
                }
                workflow.push((op, next));
            }
        }
    }

    // for (i,step) in workflow.iter().enumerate() {
    //     println!("{i:5} {step:?}")
    // }
    let mut sum = 0;

    while let Some(Ok(l)) = lines.next() {
        if l.is_empty() {
            break;
        }
        let part = Part::from_string(&l);
        println!("Part: {part:?}");

        // Run part through workflows
        let mut idx = *workflow_syms.get("in").unwrap();
        loop {
            let (op, next) = &workflow[idx];
            idx = match op {
                Operation::Less(var, value) => if part.vars[*var] < *value { *next } else { idx+1 }
                Operation::Greater(var, value) => if part.vars[*var] > *value { *next } else { idx+1 }
                Operation::Always => *next,
                Operation::Reject => break,
                Operation::Accept => { println!("Accetped: {}", part.sum()); sum += part.sum(); break }
            }
        }
    }

    println!("Sum: {sum}");
}
