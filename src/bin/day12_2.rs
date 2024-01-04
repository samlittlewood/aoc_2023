use std::collections::HashMap;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Condition {
    E,
    U,
    S,
}

impl Condition {
    fn from_char(c: char) -> Self {
        match c {
            '?' => Condition::U,
            '#' => Condition::S,
            _ => Condition::E,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Record {
    springs: Vec<Condition>,
    groups: Vec<u32>,
    memo: HashMap<(usize, usize, u32), usize>,
}

impl Record {
    fn create(springs_text: &str, groups_text: &str) -> Self {
        let (springs, groups) = (
            springs_text
                .chars()
                .map(Condition::from_char)
                .collect::<Vec<_>>(),
            groups_text
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>(),
        );
        Record {
            springs,
            groups,
            memo: HashMap::new(),
        }
    }

    fn arrangements(&mut self) -> usize {
        self.memo.clear();
        if self.groups.is_empty() {
            self.arrangements_rec(0, 0, 0)
        } else {
            self.arrangements_rec(0, self.groups[0], 1)
        }
    }

    fn arrangements_rec(&mut self, spring: usize, group: u32, next_group: usize) -> usize {
        let key = (spring, next_group, group);
        if let Some(r) = self.memo.get(&key) {
            return *r;
        }
        assert!(!(group == 0 && !next_group == self.groups.len()));

        let r = match self.springs.get(spring) {
            Some(Condition::E) => self.arrangements_rec(spring + 1, group, next_group),
            Some(Condition::S) => self.arrangements_spring(spring + 1, group, next_group),
            Some(Condition::U) => {
                self.arrangements_rec(&spring + 1, group, next_group)
                    + self.arrangements_spring(spring + 1, group, next_group)
            }
            None => {
                if group == 0 && next_group == self.groups.len() {
                    1
                } else {
                    0
                }
            }
        };
        self.memo.insert(key, r);
        r
    }

    fn arrangements_spring(&mut self, spring: usize, group: u32, next_group: usize) -> usize {
        if group > 1 {
            self.arrangements_group_more(spring, group - 1, next_group)
        } else if group > 0 {
            // Consumed current group
            if next_group != self.groups.len() {
                // More groups - must have a separating space
                self.arrangements_group_end(spring, self.groups[next_group], next_group + 1)
            } else {
                self.arrangements_group_end(spring, 0, next_group)
            }
        } else {
            0
        }
    }

    fn arrangements_group_more(&mut self, spring: usize, group: u32, next_group: usize) -> usize {
        assert!(group > 0);
        match self.springs.get(spring) {
            Some(Condition::U) | Some(Condition::S) => {
                self.arrangements_spring(spring + 1, group, next_group)
            }
            Some(Condition::E) => 0,
            None => 0,
        }
    }

    fn arrangements_group_end(&mut self, spring: usize, group: u32, next_group: usize) -> usize {
        match self.springs.get(spring) {
            Some(Condition::E) | Some(Condition::U) => {
                self.arrangements_rec(spring + 1, group, next_group)
            }
            Some(Condition::S) => 0,
            None => self.arrangements_rec(spring, group, next_group),
        }
    }
}

fn main() {
    println!("Hello Day 12 2!");

    assert_eq!(Record::create("#", "1").arrangements(), 1);

    let input = fs::read_to_string("inputs/day12").unwrap();
    let mut sum = 0;
    for l in input.lines() {
        let split = l.split_whitespace().collect::<Vec<_>>();
        let mut r = Record::create(&[split[0]; 5].join("?"), &[split[1]; 5].join(","));
        let arrangements = r.arrangements();

        println!("Arragments:  {arrangements}");

        sum += arrangements;
    }

    println!("Sum: {sum}");
    // 10861030975833
}

#[test]
fn test_1() {
    assert_eq!(Record::create("? 0").arrangements(), 1);
    assert_eq!(Record::create(". 0").arrangements(), 1);
}
