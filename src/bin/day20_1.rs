use std::{
    collections::{HashMap, VecDeque},
    fs,
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


#[derive(Debug, Copy, Clone)]
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

        // Copy input conunts into each module
        for i in 0..modules.len() {
            modules[i].input_count = input_counts[i];
        }

        let broadcast = *names_map.get("broadcast").unwrap_or(&0);
        Self { modules, broadcast, names }
    }

    fn node_name(&self, n: usize) -> String {
    	self.names.get(n).unwrap_or(&"?".to_string()).clone()
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
}

impl MachineState {
    fn create(machine: &Machine) -> Self {
        MachineState {
            state: vec![0; machine.modules.len()],
            pending_pulses: VecDeque::new(),
            low_pulses: 0,
            high_pulses: 0,
            output_pulses_low: 0,
            output_pulses_high: 0,
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

//        	println!("Pulse: {} -{}-> {}:{}", machine.node_name(_src), level, machine.node_name(module_index), input_index);

            if module_index == usize::MAX {
//            	println!("Output Pulse: {level}");
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

fn main() {
    println!("Hello Day 20 1!");

    let input = fs::read_to_string("inputs/day20").unwrap();
    let lines = input.lines().map(str::to_string).collect::<Vec<_>>();
    let machine = Machine::create(&lines);

    let mut state = MachineState::create(&machine);
    for _i in 0..1000 {
        state.send_pulses(Level::Low, &[(machine.broadcast, 0)], usize::MAX, &machine);
    }

    println!(
        "Low: {} High: {} Product: {}",
        state.low_pulses,
        state.high_pulses,
        state.low_pulses * state.high_pulses
    );
    // 681194780
}
