use std::{collections::HashMap, str::FromStr};

pub struct Polymer {
    pub counter: HashMap<String, u64>,
    pub instructions: HashMap<String, (String, String)>,
    pub start: char,
    pub end: char,
}

impl Polymer {
    pub fn step(&mut self) {
        let old_count = self.counter.clone();
        for (key, (left_child, right_child)) in self.instructions.iter() {
            let to_add = old_count.get(key).unwrap();
            let count = self.counter.get_mut(key).unwrap();
            *count -= to_add;
            let count = self.counter.get_mut(left_child).unwrap();
            *count += to_add;
            let count = self.counter.get_mut(right_child).unwrap();
            *count += to_add;
        }
    }

    pub fn score(&self) -> u64 {
        let mut counter = HashMap::new();
        for (key, val) in self.counter.iter() {
            for c in key.chars() {
                let count = counter.entry(c).or_insert(0);
                *count += val;
            }
        }
        let count = counter.entry(self.start).or_insert(0);
        *count += 1;
        let count = counter.entry(self.end).or_insert(0);
        *count += 1;
        let min = counter.iter().map(|(_k, v)| v).min().unwrap();
        let max = counter.iter().map(|(_k, v)| v).max().unwrap();
        (max - min) / 2
    }
}
impl FromStr for Polymer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut template = s
            .lines()
            .filter(|x| !x.is_empty() && !x.contains("->"))
            .map(String::from)
            .collect::<Vec<String>>();

        assert!(template.len() == 1);
        let template = template.pop().unwrap();
        let start = template.chars().nth(0).unwrap();
        let end = template.chars().nth_back(0).unwrap();
        let mut instructions = HashMap::new();
        for line in s.lines().filter(|x| !x.is_empty() && x.contains("->")) {
            let map = line.split(" -> ").collect::<Vec<_>>();
            assert!(map.len() == 2);
            assert!(map[0].chars().count() == 2);
            assert!(map[1].chars().count() == 1);
            let mut children = (String::new(), String::new());
            children.0.push(map[0].chars().next().unwrap());
            children.0.push_str(map[1]);
            children.1.push_str(map[1]);
            children.1.push(map[0].chars().nth(1).unwrap());

            assert!(instructions
                .insert(String::from(map[0]), children)
                .is_none());
        }

        let mut counter: HashMap<String, u64> =
            instructions.iter().map(|(k, _v)| (k.clone(), 0)).collect();

        for i in 0..template.len() - 1 {
            let count = counter.get_mut(&template[i..=i + 1]).unwrap();
            *count += 1;
        }

        Ok(Polymer {
            counter,
            instructions,
            start,
            end,
        })
    }
}

fn main() {
    let mut polymer = Polymer::from_str(include_str!("../data/main.txt")).unwrap();
    for _ in 0..10 {
        polymer.step();
    }
    println!("Score for part 1: {}", polymer.score());
    for _ in 10..40 {
        polymer.step();
    }
    println!("Score for part 2: {}", polymer.score());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut polymer = Polymer::from_str(include_str!("../data/test.txt")).unwrap();

        for _ in 0..10 {
            polymer.step();
        }
        println!("{:#?}", polymer.counter);
        assert_eq!(polymer.score(), 1588);
    }
}
