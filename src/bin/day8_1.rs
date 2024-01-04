use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    left: usize,
    right: usize,
}

fn graph_from_input<I>(input: &mut I) -> Vec<Node> where I: Iterator<Item = String> {

    let mut names = HashMap::new();

    names.insert("AAA".to_string(), 1);
    names.insert("ZZZ".to_string(), 2);
    let mut last_id = 2;

    let mut graph = Vec::new();

    // XXX sort out string borrowing
    for l in input.filter(|s| !s.is_empty()) {
        let c = l.split(|c:char| !c.is_alphanumeric()).filter(|s| !s.is_empty()).map(|s| s.to_string()).collect::<Vec<_>>();
        let (node,left,right) = (c[0].clone(), c[1].clone(), c[2].clone());

//        println!(":{node} {left} {right}");
        let node_id = *names.entry(node).or_insert_with(|| { last_id +=1; last_id });
        let left_id = *names.entry(left).or_insert_with(|| { last_id +=1; last_id });
        let right_id = *names.entry(right).or_insert_with(|| { last_id +=1; last_id });

//        println!("{node_id} {left_id} {right_id}");
        graph.resize(last_id+1, Node{left:0, right:0});
        graph[node_id] = Node { left:left_id, right: right_id}
    }

    graph
}

fn main() {
    println!("Hello, Day 8 1");

    let input = fs::read_to_string("inputs/day8").unwrap();
    let mut input_iter = input.lines().map(|s| s.to_string());

    let instructions = input_iter.next().unwrap();
    println!("Instructions: {instructions}");

    let graph = graph_from_input(&mut input_iter);

    let mut node = 1;
    let mut count = 0;
    for direction in instructions.chars().cycle() {
        count += 1;
        match(direction) {
            'L' => node = graph[node].left,
            'R' => node = graph[node].right,
            _ => panic!("Unknown direction {:?}", direction),
        }

        if node == 2 {
            break;
        }

    }
    println!("Count: {}", count)


}


#[test]
fn test1() {
}
