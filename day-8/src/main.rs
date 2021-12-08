use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct DiaryEntry {
    signal: Vec<String>,
    output: Vec<String>,
    decoder: HashMap<String, u8>,
}

impl DiaryEntry {
    pub fn build_decoder(&mut self) {
        // first get 1 4 7 8
        self.decoder.insert(
            self.signal
                .iter()
                .find(|o| o.len() == 2)
                .unwrap()
                .to_owned(),
            1,
        );
        let four = self.signal.iter().find(|o| o.len() == 4).unwrap();
        self.decoder.insert(four.to_string(), 4);
        let four = to_set(&four);

        let seven = self.signal.iter().find(|o| o.len() == 3).unwrap();
        self.decoder.insert(seven.to_string(), 7);
        let seven = to_set(&seven);

        let eight = self.signal.iter().find(|o| o.len() == 7).unwrap();
        self.decoder.insert(eight.to_string(), 8);
        let eight = to_set(&eight);
        // Now find 6 9 0; we need 6 & 9 to be sets for later
        let mut six = String::new();
        let mut nine = String::new();
        let mut zero = String::new();
        for s in self.signal.iter().filter(|o| o.len() == 6) {
            if eight
                .difference(&seven)
                .into_iter()
                .cloned()
                .collect::<HashSet<char>>()
                .is_subset(&to_set(s))
            {
                six = s.to_string();
            } else if four.is_subset(&to_set(s)) {
                nine = s.to_string();
            } else {
                zero = s.to_string();
            }
        }
        self.decoder.insert(six.clone(), 6);
        self.decoder.insert(nine.clone(), 9);
        self.decoder.insert(zero, 0);

        let six = to_set(&six);

        // Get the last couple: 2 3 5

        let mut two = String::new();
        let mut three = String::new();
        let mut five = String::new();
        for s in self.signal.iter().filter(|o| o.len() == 5) {
            if six.is_superset(&to_set(s)) {
                five = s.clone();
            } else if eight
                .difference(&four)
                .into_iter()
                .cloned()
                .collect::<HashSet<_>>()
                .is_subset(&to_set(s))
            {
                two = s.to_string();
            } else {
                three = s.to_string();
            }
        }
        self.decoder.insert(two, 2);
        self.decoder.insert(three, 3);
        self.decoder.insert(five, 5);
    }
    fn decode(&self, s: &str) -> u8 {
        self.decoder.get(s).unwrap().clone()
    }
    fn get_output(&self) -> u32 {
        (0..4)
            .rev()
            .map(|i| 10u32.pow(i))
            .zip(self.output.iter().map(|o| self.decode(o)))
            .map(|(place, dig)| place * (dig as u32))
            .sum()
    }
}

fn to_set(in_string: &str) -> HashSet<char> {
    in_string.chars().collect()
}

impl FromStr for DiaryEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: Vec<&str> = s.trim().split('|').map(|o| o.trim()).collect();

        let signal = entry[0]
            .split(' ')
            .map(|o| {
                let mut char_vec: Vec<char> = o.trim().chars().collect();
                char_vec.sort();
                char_vec.into_iter().collect()
            })
            .collect();
        let output = entry[1]
            .split(' ')
            .map(|o| {
                let mut char_vec: Vec<char> = o.trim().chars().collect();
                char_vec.sort();
                char_vec.into_iter().collect()
            })
            .collect();

        let mut diary = DiaryEntry {
            signal,
            output,
            decoder: HashMap::new(),
        };
        diary.build_decoder();
        Ok(diary)
    }
}

fn easy_chars(entries: &[DiaryEntry]) -> usize {
    entries
        .iter()
        .map(|o| {
            o.output
                .iter()
                .map(|x| match x.len() {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let data = include_str!("../data/main.txt")
        .lines()
        .map(DiaryEntry::from_str)
        .collect::<Result<Vec<_>>>()
        .unwrap();
    println!("{}", easy_chars(&data));
    // part two
    println!(
        "answer to part 2: {}",
        data.iter().map(|entry| entry.get_output()).sum::<u32>()
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("../data/test.txt")
            .lines()
            .map(DiaryEntry::from_str)
            .collect::<Result<Vec<_>>>()
            .unwrap();
        assert_eq!(easy_chars(&data), 26);
    }

    #[test]
    fn test_build_decoder() {
        let entry = DiaryEntry::from_str(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        )
        .unwrap();
        /*
        assert_eq!(entry.decode("acedgfb"), 8);
        assert_eq!(entry.decode("cdfbe"), 5);
        assert_eq!(entry.decode("gcdfa"), 2);
        assert_eq!(entry.decode("fbcad"), 3);
        assert_eq!(entry.decode("dab"), 7);
        assert_eq!(entry.decode("cefabd"), 9);
        assert_eq!(entry.decode("cdfgeb"), 6);
        assert_eq!(entry.decode("eafb"), 4);
        assert_eq!(entry.decode("cagedb"), 0);
        assert_eq!(entry.decode("ab"), 1);
        */
        assert_eq!(entry.get_output(), 5353)
    }
}
