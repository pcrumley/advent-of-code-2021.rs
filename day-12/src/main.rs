use anyhow::{anyhow, Result};
// use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
//use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Cave {
    name: String,
    is_big: bool,
    neighbors: Vec<String>,
}

impl Cave {
    pub fn new(name: &str) -> Self {
        Cave {
            name: name.to_string(),
            is_big: name == name.to_uppercase(),
            neighbors: Vec::new(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CaveSystem {
    caves: HashMap<String, Cave>,
}

impl CaveSystem {
    pub fn new() -> Self {
        CaveSystem {
            caves: HashMap::new(),
        }
    }
    pub fn count_routes(&self, is_part_a: bool) -> u32 {
        // recursor function that returns 1 if it reaches
        // end 0 otherwise
        fn inner(
            system: &CaveSystem,
            cur: &Cave,
            visited: HashMap<String, u8>,
            is_part_a: bool,
        ) -> u32 {
            if cur.name == "end" {
                return 1;
            }
            let mut visited = visited.clone();
            let times = visited.entry(cur.name.clone()).or_insert(0);
            if !cur.is_big {
                *times += 1;
            }
            let mut count = 0;

            for n in cur.neighbors.iter() {
                let next = system.caves.get(n).unwrap();
                if next.name == "start".to_string() {
                    continue;
                }
                if next.is_big {
                    count += inner(system, &next, visited.clone(), is_part_a)
                } else if !visited.contains_key(&next.name) {
                    count += inner(system, &next, visited.clone(), is_part_a);
                } else if !is_part_a {
                    if *visited.get(&cur.name).unwrap() < 2
                        && !visited.iter().map(|(_key, val)| val).any(|&x| x >= 2)
                    {
                        count += inner(system, &next, visited.clone(), is_part_a);
                    }
                }
            }
            count
        }
        let cur = self.caves.get("start").unwrap();

        let visited = HashMap::new();
        // visited.insert("start".into(), 1);
        inner(self, cur, visited, is_part_a)
    }
}

impl FromStr for CaveSystem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cave_system = CaveSystem::new();
        for edge in s.lines() {
            let caves = edge.split("-").collect::<Vec<_>>();
            if caves.len() != 2 {
                return Err(anyhow!("Invavlid edge descriptor: {}", edge));
            }
            // update left_cave
            let left_cave = cave_system
                .caves
                .entry(caves[0].to_owned())
                .or_insert(Cave::new(caves[0]));
            left_cave.neighbors.push(caves[1].to_string());
            let right_cave = cave_system
                .caves
                .entry(caves[1].to_owned())
                .or_insert(Cave::new(caves[1]));
            right_cave.neighbors.push(caves[0].to_string())
        }
        Ok(cave_system)
    }
}

fn main() {
    let cave_system = CaveSystem::from_str(include_str!("../data/main.txt")).unwrap();
    println!(
        "Number of routes in this cave system: Part 1: {}, Part 2: {}",
        cave_system.count_routes(true),
        cave_system.count_routes(false),
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let cave_system = CaveSystem::from_str(include_str!("../data/test_1.txt")).unwrap();
        assert_eq!(cave_system.count_routes(true), 10);
        assert_eq!(cave_system.count_routes(false), 36);
        let cave_system = CaveSystem::from_str(include_str!("../data/test_2.txt")).unwrap();
        assert_eq!(cave_system.count_routes(true), 19);
        assert_eq!(cave_system.count_routes(false), 103);
        let cave_system = CaveSystem::from_str(include_str!("../data/test_3.txt")).unwrap();
        assert_eq!(cave_system.count_routes(true), 226);
        assert_eq!(cave_system.count_routes(false), 3509);
    }
}
