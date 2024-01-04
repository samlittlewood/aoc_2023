use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    left: usize,
    right: usize,
    startflag: bool,
    endflag: bool,
}

fn graph_from_input<I>(input: &mut I) -> Vec<Node> where I: Iterator<Item = String> {

    let mut names = HashMap::new();

    let mut last_id = 0;

    let mut graph = Vec::new();

    // XXX sort out string borrowing
    for l in input.filter(|s| !s.is_empty()) {
        let c = l.split(|c:char| !c.is_alphanumeric()).filter(|s| !s.is_empty()).map(|s| s.to_string()).collect::<Vec<_>>();
        let (nodename,leftname,rightname) = (c[0].clone(), c[1].clone(), c[2].clone());

        let node = *names.entry(nodename.clone()).or_insert_with(|| { last_id +=1; last_id });
        let left = *names.entry(leftname.clone()).or_insert_with(|| { last_id +=1; last_id });
        let right = *names.entry(rightname.clone()).or_insert_with(|| { last_id +=1; last_id });
        let startflag = nodename.clone().ends_with('A');
        let endflag = nodename.ends_with('Z');

        graph.resize(last_id+1, Node{left:0, right:0, startflag:false, endflag:false});
        graph[node] = Node { left, right, startflag, endflag};
    }

    graph
}

fn distance(graph: &Vec<Node>, directions:&[bool], start: usize) -> (usize, usize) {

    let mut node = start;
    let mut count = 0;

    for d in directions.iter().cycle() {
        count += 1;
        node =  if *d { graph[node].left } else { graph[node].right };
        if graph[node].endflag {
            break
        }
    }
    (node, count)
}

fn main() {
    println!("Hello, Day 8 2");

    let input = fs::read_to_string("inputs/day8").unwrap();
    let mut input_iter = input.lines().map(|s| s.to_string());

    let is = input_iter.next().unwrap();
    let instructions = is.chars().map(|c| {
        match c {
            'L' => true,
            'R' => false,
            _ => panic!("Bad direction {c:?}")
        }
    }).collect::<Vec<_>>();

    let graph = graph_from_input(&mut input_iter);

    // Fnd starts and compute distances for each cycle
    let starts = graph.iter().enumerate().filter(|(_,n)| n.startflag).map(|(i,_)| i);
    let distances =
        starts.map(|node| {let (_,d) = distance(&graph, &instructions, node); d}).collect::<Vec<_>>();
    println!("Distances: {distances:?}");

    let steps = lcm(&distances);

    println!("Steps: {}", steps)
    // 18024643846273
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[test]
fn test1() {
    assert_eq!(lcm(&[1, 2, 3, 4, 5]), 60);
    assert_eq!(lcm(&[2, 4, 6, 8, 10]), 120);
    assert_eq!(lcm(&[3, 6, 9, 12, 15]), 180);
    assert_eq!(lcm(&[10]), 10);
    assert_eq!(lcm(&[21, 110]), 2310);
}
