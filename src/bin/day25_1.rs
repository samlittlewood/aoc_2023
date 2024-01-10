use std::{
    collections::{HashMap, BinaryHeap},
    fs
};
use rand::Rng;

#[derive(Debug, Clone)]
struct Node {
    edges: Vec<usize>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge (usize, usize);

impl Edge {
    fn cross(&self, from: usize) -> usize {
        if self.0 == from {
            self.1
        } else {
            self.0
        }
    }
}

#[derive(Debug)]
struct Machine {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
	names: Vec<String>
}

fn add_node(map: &mut HashMap<String, usize>, nodes: &mut Vec<Node>, n:&str) -> usize {
    *map.entry(n.to_string()).or_insert_with(|| {
       nodes.push( Node { edges: Vec::new() });
        nodes.len() -1
    })
}

fn add_edge(map: &mut HashMap<Edge, usize>, edges: &mut Vec<Edge>, n0:usize, n1:usize) -> usize {
    let e = Edge(usize::min(n0,n1), usize::max(n0,n1));
    *map.entry(e).or_insert_with(|| {
        edges.push(e);
        edges.len()-1
    })
}

impl Machine {
    fn create(source: &[String]) -> Self {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut names = Vec::new();
        let mut nodes_map: HashMap<_, usize> = HashMap::new();
        let mut edges_map: HashMap<Edge, usize> = HashMap::new();

        for l in source.iter().filter(|l| !l.is_empty()) {
            let (left, right) = l.split_at(l.find(":").unwrap());
            let node_name = left.trim();
            let connection_names = right
                .trim_start_matches(":")
                .split_whitespace()
                .map(str::trim)
                .collect::<Vec<_>>();

            let n = add_node(&mut nodes_map, &mut nodes, node_name);
            for cn in connection_names {
                let c = add_node(&mut nodes_map, &mut nodes, cn);
                let edge = add_edge(&mut edges_map, &mut edges, n, c);
                nodes[n].edges.push(edge);
                nodes[c].edges.push(edge);
            }
        }

        names.resize(nodes.len(), "".to_string());
        for (k,v) in nodes_map.into_iter() {
            names[v] = k;
        }
        Self { nodes, edges, names }
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
    	println!("Nodes:");
    	for (i,n) in self.nodes.iter().enumerate() {
	    	let edges = n.edges.iter().map(|e| format!("{e}")).collect::<Vec<_>>().join(", ");
    		println!("  {:4} {}: {}", i, self.node_name(i), edges);
    	}
        println!("Edges:");
        for (i,e) in self.edges.iter().enumerate() {
            println!("  {:4} {} <-> {}", i, self.node_name(e.0),self.node_name(e.1));
        }
    }

    fn find_route(&self, start: usize, end:usize, edge_counts: &mut [usize]) {
        let mut visited = vec![false; self.nodes.len()];
        let mut pending : BinaryHeap<_> = BinaryHeap::new();
        pending.push((usize::MAX, 0, usize::MAX, start));

        while let Some((_, cost, edge, node)) = pending.pop() {
            if visited[node] {
                continue;
            }

            if edge != usize::MAX {
                edge_counts[edge] += 1;
            }
            if node == end {
                break;
            }
            visited[node] = true;
            for &next_edge in &self.nodes[node].edges {
                let next_node = self.edges[next_edge].cross(node);
                if !visited[next_node] {
                    pending.push((usize::MAX - (cost+1), cost+1, next_edge, next_node));
                }
            }
        }
    }

    fn count_connected(&self, start: usize, hide_edges:&[bool])-> usize {
        let mut visited = vec![false; self.nodes.len()];
        let mut pending : BinaryHeap<_> = BinaryHeap::new();
        pending.push(start);

        let mut count = 0;
        while let Some(node) = pending.pop() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            count +=1;

            for &next_edge in &self.nodes[node].edges {
                if !hide_edges[next_edge] {
                    let next_node = self.edges[next_edge].cross(node);
                    if !visited[next_node] {
                        pending.push(next_node);
                    }
                }
            }
        }
        count
    }
}

fn main() {
    println!("Hello Day 25 1!");

    let input = fs::read_to_string("inputs/day25").unwrap();
    let lines = input.lines().map(str::to_string).collect::<Vec<_>>();
    let machine = Machine::create(&lines);
//    machine.dump_graph();
    let mut edge_counts = vec![0; machine.edges.len()];

    let mut rng = rand::thread_rng();

    for _ in 0 .. 10000 {
        let start = rng.gen_range(0..machine.nodes.len());
        let mut end;
        loop {
             end=rng.gen_range(0..machine.nodes.len());
             if start != end {
                break;
             }
        }
        machine.find_route(start, end, &mut edge_counts);
    }

    let mut edges_sorted = edge_counts.iter().enumerate().map(|(i,c)| (c,i)).collect::<Vec<_>>();
    edges_sorted.sort();

    let mut hide_edges = vec![false; machine.edges.len()];
    let mut joining_edges = Vec::new();

    for _ in 0..3 {
        if let Some((c,e)) = edges_sorted.pop() {
            println!(" Edge {e} {c}");
            hide_edges[e] = true;
            joining_edges.push(e);
        }
    }

    let group0 = machine.count_connected(machine.edges[joining_edges[0]].0, &hide_edges);
    let group1 = machine.count_connected(machine.edges[joining_edges[0]].1, &hide_edges);

    println!("Nodes:{} Edges:{} Group0:{} Group1:{} Product:{}", machine.nodes.len(), machine.edges.len(), group0, group1, group0 * group1);
    // 596376
}
