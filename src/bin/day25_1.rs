use std::{
    collections::{HashMap},
    fs
};


#[derive(Debug, Clone)]
struct Node {
    name: String,
    connections: Vec<usize>
}

#[derive(Debug)]
struct Machine {
    nodes: Vec<Node>,
	names: Vec<String>
}

impl Machine {
    fn create(source: &[String]) -> Self {
        let mut nodes = Vec::new();
        let mut names_map: HashMap<_, usize> = HashMap::new();
        let mut names = Vec::new();
        let mut last_id = 0;
        for l in source.iter().filter(|l| !l.is_empty()) {
            let (left, right) = l.split_at(l.find(":").unwrap());
            let node_name = left.trim();
            let connection_names = right
                .trim_start_matches(":")
                .split_whitespace()
                .map(str::trim)
                .collect::<Vec<_>>();

            let node = *names_map.entry(node_name.to_string()).or_insert_with(|| { last_id+=1; last_id-1 });
            for cn in connection_names {
                let connection = *names_map.entry(cn.to_string()).or_insert_with(|| { last_id+=1; last_id-1 });
                nodes.resize(last_id, Node { name: "".to_string(), connections: Vec::new() });
                nodes[node].connections.push(connection);
            }
        }

        names.resize(last_id, "".to_string());
        for (k,v) in names_map.into_iter() {
            names[v] = k;
        }
        Self { nodes, names }
    }

    fn node_name(&self, n: usize) -> String {
    	if let Some(s) = self.names.get(n) {
    		s.to_owned()
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
    	println!("strict graph {{");
    	for n in 0..self.nodes.len() {
	    	let outputs = self.nodes[n].connections.iter().map(|n| self.node_name(*n)).collect::<Vec<_>>().join(", ");
    		println!("  {} -- {{ {} }};", self.node_name(n), outputs);
    	}
    	println!("}}");
    }
}


fn main() {
    println!("Hello Day 20 2!");

    let input = fs::read_to_string("inputs/day25").unwrap();
    let lines = input.lines().map(str::to_string).collect::<Vec<_>>();
    let machine = Machine::create(&lines);
    machine.dump_graph();
}
