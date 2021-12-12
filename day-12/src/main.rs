use anyhow::{anyhow, Result};
// use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
//use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug)]
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

pub struct CaveSystem {
    caves: HashMap<String, Cave>,
}

#[derive(Copy, Clone, Debug)]
pub enum Part {
    A,
    B,
}
impl CaveSystem {
    pub fn new() -> Self {
        CaveSystem {
            caves: HashMap::new(),
        }
    }
    pub fn count_routes(&self, part: Part) -> u32 {
        // recursor function that returns 1 if it reaches
        // end 0 otherwise
        fn inner(system: &CaveSystem, cur: &Cave, mut visited: HashSet<String>, part: Part) -> u32 {
            if cur.name == "end" {
                return 1;
            }
            let mut to_visit = cur
                .neighbors
                .iter()
                .filter(|s| !visited.contains(*s))
                .collect::<Vec<_>>();
            if to_visit.is_empty() {
                0
            } else {
                if !cur.is_big {
                    visited.insert(cur.name.to_owned());
                }
                let mut total = 0;
                while let Some(x) = to_visit.pop() {
                    let next = system.caves.get(x).unwrap();
                    total += inner(system, &next, visited.clone(), part);
                }
                total
            }
        }
        let cur = self.caves.get("start").unwrap();
        let have_visited = HashSet::new();
        inner(self, cur, have_visited, part)
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
        "Number of routes in this cave system: {}",
        cave_system.count_routes(Part::A)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let cave_system = CaveSystem::from_str(include_str!("../data/test_1.txt")).unwrap();
        assert_eq!(cave_system.count_routes(Part::A), 10);
        let cave_system = CaveSystem::from_str(include_str!("../data/test_2.txt")).unwrap();
        assert_eq!(cave_system.count_routes(Part::A), 19);
        let cave_system = CaveSystem::from_str(include_str!("../data/test_3.txt")).unwrap();
        assert_eq!(cave_system.count_routes(Part::A), 226);
    }
}
