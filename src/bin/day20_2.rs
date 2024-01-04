//
// c1 4001
// c2 4027
// c3 3929
// c4 3769
//
// c1,c2 16112027
// c1,c3 15719929
// c1,c4 15079769
// c2,c3 15822083
// c2,c4 15177763
// c3,c4 14808401
//
// c1,c2,c3,c4 238593356738827
//
use std::{
    collections::{HashMap, VecDeque},
    fs
};

use std::fmt::{self, Formatter, Display};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Level {
    Low,
    High,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    	let s = if *self == Level::Low {
	    		"low"
    		} else {
    			"high"
    		};
    	write!(f, "{}", s)
    }
}

#[derive(Debug, Copy, Clone)]
struct Pulse {
    level: Level,
    dest: (usize, usize),
    src: usize,
}


#[derive(Debug, Copy, Clone, PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjunciton,
    Broadcast,
}

#[derive(Debug)]
struct Module {
    typ: ModuleType,
    outputs: Vec<(usize, usize)>,
    input_count: usize,
}

#[derive(Debug)]
struct Machine {
    modules: Vec<Module>,
	broadcast: usize,
	names: Vec<String>
}

impl Machine {
    fn create(source: &[String]) -> Self {
        let mut modules = Vec::new();
        let mut names_map: HashMap<_, usize> = HashMap::new();
        let mut input_counts: Vec<usize> = Vec::new();
        let mut names = Vec::new();

        // two passes to resolve forward refs
        for pass in 0..=1 {
            modules.clear(); // start each pass with empty table
            for l in source.iter().filter(|l| !l.is_empty()) {
                let (left, right) = l.split_at(l.find("->").unwrap());
                let module = left.trim();
                let (module_type, module_name) = match module.chars().next().unwrap() {
                    '%' => (ModuleType::FlipFlop, module.trim_start_matches('%')),
                    '&' => (ModuleType::Conjunciton, module.trim_start_matches('&')),
                    'b' => (ModuleType::Broadcast, "broadcast"),
                    _ => panic!("Unknown module {module}"),
                };

                match pass {
                    0 => {
                        // First pass - just fill slots
                        names_map.insert(module_name.to_string(), modules.len());
                        modules.push(Module {
                            typ: module_type,
                            outputs: Vec::new(),
                            input_count: 0,
                        });
                    }
                    1 => {
                        // Sewcond pass - resolve outputs and inputs
                        let output_names = right
                            .trim_start_matches("->")
                            .split_terminator(',')
                            .map(str::trim)
                            .collect::<Vec<_>>();
                        let outputs = output_names
                            .into_iter()
                            .map(|n| {
                                if let Some(&oi) = names_map.get(n) {
                                    let ii = input_counts[oi];
                                    input_counts[oi] += 1;
                                    (oi, ii)
                                } else {
                                    println!("Unknown name: {n}");
                                    (usize::MAX, 0)
                                }
                            })
                            .collect();

                        names.push(module_name.to_string());
                        modules.push(Module {
                            typ: module_type,
                            outputs,
                            input_count: 0,
                        });
                    }
                    _ => {
                        panic!("Oops")
                    }
                }
            }
            input_counts.resize(modules.len(), 0);
        }

        // Copy input counts into each module
        for i in 0..modules.len() {
            modules[i].input_count = input_counts[i];
        }

        let broadcast = *names_map.get("broadcast").unwrap_or(&0);
        Self { modules, broadcast, names }
    }

    fn node_name(&self, n: usize) -> String {
    	if let Some(s) = self.names.get(n) {
    		if self.modules[n].typ == ModuleType::Conjunciton {
	    		s.to_uppercase()
    		} else {
	    		s.to_owned()
    		}
    	} else {
    		"output".to_string()
    	}
    }

    fn find_node(&self, s: &str) -> Option<usize> {
    	for (i,n) in self.names.iter().enumerate() {
    		if n == s {
    			return Some(i);
    		}
    	}

    	return None;
    }

    fn dump_graph(&self) {
    	println!("strict digraph {{");
    	for n in 0..self.modules.len() {
	    	let outputs = self.modules[n].outputs.iter().map(|n| self.node_name(n.0)).collect::<Vec<_>>().join(", ");
    		println!("  {} -> {{ {} }};", self.node_name(n), outputs);
    	}
    	println!("}}");
    }
}

#[derive(Debug)]
struct MachineState {
    state: Vec<usize>,
    pending_pulses: VecDeque<Pulse>,
    low_pulses: usize,
    high_pulses: usize,
    output_pulses_low: usize,
    output_pulses_high: usize,
    watch: usize,
    step: usize,
}

impl MachineState {
    fn create(machine: &Machine, watch_name: &str) -> Self {
        MachineState {
            state: vec![0; machine.modules.len()],
            pending_pulses: VecDeque::new(),
            low_pulses: 0,
            high_pulses: 0,
            output_pulses_low: 0,
            output_pulses_high: 0,
            watch: machine.find_node(watch_name).unwrap_or(usize::MAX),
            step: 0
        }
    }

    fn send_pulses(&mut self, level: Level, dests: &[(usize, usize)], src:usize, machine: &Machine) {
        match level {
            Level::Low => self.low_pulses += dests.len(),
            Level::High => self.high_pulses += dests.len(),
        }

        for &dest in dests {
        	self.pending_pulses.push_front(Pulse { level, dest, src });
        }
        self.process(machine);
    }

    fn process(&mut self, machine: &Machine) {
        while let Some(Pulse {level, dest:(module_index, input_index), src:_src}) = self.pending_pulses.pop_back() {

        	if module_index == self.watch && level == Level::High {
//	        	println!("Watch: {} {} -{}-> {}:{}", self.step, machine.node_name(_src), level, machine.node_name(module_index), input_index);
        	}

            if module_index == usize::MAX {
		        match level {
        		    Level::Low => self.output_pulses_low += 1,
        		    Level::High => self.output_pulses_high += 1,
        		}
                continue;
            }
            let module = &machine.modules[module_index];
            match module.typ {
                ModuleType::Broadcast => {
	                self.send_pulses(level, &module.outputs, module_index, machine)
                }
                ModuleType::FlipFlop => {
                    if level == Level::Low {
                        self.state[module_index] ^= 1;
                        let level = if self.state[module_index] == 0 {
                            Level::Low
                        } else {
                            Level::High
                        };
                        self.send_pulses(level, &module.outputs, module_index, machine)
                    }
                }
                ModuleType::Conjunciton => {
                    if level == Level::Low {
                        self.state[module_index] &= !(1 << input_index);
                    } else {
                        self.state[module_index] |= 1 << input_index;
                    }

                    let out_level = if self.state[module_index] == (1 << module.input_count) - 1 {
                        Level::Low
                    } else {
                        Level::High
                    };

                    self.send_pulses(out_level, &module.outputs, module_index, machine)
                }
            }
        }
    }
}

fn show_counter(nodes:&[usize], machine: &Machine, state: &MachineState)
{
    for &n in nodes {
    	match machine.modules[n].typ {
    		ModuleType::FlipFlop => if state.state[n] == 0 { print!("-") } else { print!("*") },
    		ModuleType::Conjunciton => print!("{}", char::from_digit(state.state[n].count_ones(), 16).unwrap()),
    		ModuleType::Broadcast => print!("B"),
    	}
    }
}

fn main() {
    println!("Hello Day 20 2!");

    let input = fs::read_to_string("inputs/day20_modified").unwrap();
    let lines = input.lines().map(str::to_string).collect::<Vec<_>>();
    let machine = Machine::create(&lines);
//    machine.dump_graph();
    let mut state = MachineState::create(&machine, "bq");
    let counter_1 = ["vh","gp","zt", "sk", "nd","fs", "sj", "gs", "gq", "xb", "vn", "ql", "qz", "gc"].into_iter().map(|s| machine.find_node(s)).collect::<Option<Vec<_>>>().unwrap();
    let counter_2 = ["lg","zx","hf", "xm", "kh","gf", "fn", "bm", "cp", "gb", "dl", "lh", "lx", "vg"].into_iter().map(|s| machine.find_node(s)).collect::<Option<Vec<_>>>().unwrap();
    let counter_3 = ["sp","jh","kg", "xz", "zs","fx", "qf", "gz", "vv", "mq", "bh", "kr", "db", "kp"].into_iter().map(|s| machine.find_node(s)).collect::<Option<Vec<_>>>().unwrap();
    let counter_4 = ["mh","xs","sh", "pz", "cn","sv", "xf", "xp", "zn", "cv", "hl", "tr", "sd", "tx"].into_iter().map(|s| machine.find_node(s)).collect::<Option<Vec<_>>>().unwrap();

    if true {
	    for step in 0..4096 * 4096 {
	    	state.step = step;
	        state.send_pulses(Level::Low, &[(machine.broadcast, 0)], usize::MAX, &machine);
	       	// print!("{:6} ", step);
	       	// show_counter(&counter_1, &machine, &state);
	       	// print!("    ");
	       	// show_counter(&counter_2, &machine, &state);
	       	// print!("    ");
	       	// show_counter(&counter_3, &machine, &state);
	       	// print!("    ");
	       	// show_counter(&counter_4, &machine, &state);
	       	// println!("");

	        if state.output_pulses_low > 0 {
	        	println!("Output Low: {}", step+1);
	        	break;
	        }
	    }
	}

    println!("Output Low: {} Output High: {}", state.output_pulses_low, state.output_pulses_high);
}
