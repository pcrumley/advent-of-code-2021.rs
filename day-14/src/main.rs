use anyhow::{anyhow, Result};
use std::{collections::HashMap, str::FromStr};

pub struct Polymer {
    pub template: String,
    pub instructions: HashMap<String, char>,
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
        let mut instructions = HashMap::new();
        for line in s.lines().filter(|x| !x.is_empty() && x.contains("->")) {
            let map = line.split(" -> ").collect::<Vec<_>>();
            assert!(map.len() == 2);
            assert!(map[0].chars().count() == 2);
            assert!(map[1].chars().count() == 1);
            assert!(instructions
                .insert(String::from(map[0]), map[1].chars().next().unwrap())
                .is_none());
        }
        Ok(Polymer {
            template,
            instructions,
        })
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut polymer = Polymer::from_str(include_str!("../data/test.txt")).unwrap();
    }
}
