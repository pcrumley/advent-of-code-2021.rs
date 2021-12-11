use anyhow::{anyhow, Result};
use std::str::FromStr;
#[derive(Clone, Debug)]
pub struct Octopus {
    val: u32,
    has_flashed: bool,
}
pub struct Octopi {
    octopi: Vec<Octopus>,
    row_len: usize,
    col_len: usize,
}
impl FromStr for Octopi {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut col_len = 0;
        let mut line_iter = s.lines().peekable();
        let row_len = if let Some(line) = line_iter.peek() {
            line.len()
        } else {
            return Err(anyhow!("empty string"));
        };
        let mut octopi = Vec::new();

        for line in line_iter {
            let row: Vec<Octopus> = line
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if c.is_numeric() && c.is_ascii() {
                        Ok(Octopus {
                            val: c.to_digit(10).unwrap(),
                            has_flashed: false,
                        })
                    } else {
                        Err(anyhow!(
                            "INVALID CHAR IN INPUT ON ROW {} AT POS {}",
                            col_len,
                            i
                        ))
                    }
                })
                .collect::<Result<_>>()?;
            if row_len != row.len() {
                return Err(anyhow!("INVALID ROW LENGTH IN ROW {}", col_len));
            }
            octopi.extend_from_slice(&row);
            col_len += 1;
        }
        Ok(Octopi {
            octopi,
            row_len,
            col_len,
        })
    }
}

impl Octopi {
    fn get_neighbors(&self, pos: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        // Not at the start of a row
        let is_first_col = pos % self.row_len == 0;
        let is_last_col = pos % self.row_len == self.row_len - 1;
        let is_first_row = pos < self.row_len;
        let is_last_row = pos >= self.row_len * (self.col_len - 1);

        if !is_first_col {
            neighbors.push(pos - 1);
            if !is_first_row {
                neighbors.push(pos - 1 - self.row_len);
            }
            if !is_last_row {
                neighbors.push(pos - 1 + self.row_len);
            }
        }
        // not at end of a row
        if !is_last_col {
            neighbors.push(pos + 1);
            if !is_first_row {
                neighbors.push(pos + 1 - self.row_len);
            }
            if !is_last_row {
                neighbors.push(pos + 1 + self.row_len);
            }
        }
        // not in first row
        if !is_first_row {
            neighbors.push(pos - self.row_len);
        }
        // not in last row
        if !is_last_row {
            neighbors.push(pos + self.row_len);
        }

        neighbors
    }

    pub fn step(&mut self) -> u64 {
        let mut to_change: Vec<usize> = (0..self.octopi.len()).collect();
        while let Some(i) = to_change.pop() {
            let o = &mut self.octopi[i];
            if !o.has_flashed {
                o.val += 1;
                if o.val > 9 {
                    o.val = 0;
                    o.has_flashed = true;
                    for neighbor in self.get_neighbors(i) {
                        to_change.push(neighbor);
                    }
                }
            }
        }

        let flashes = self.octopi.iter().filter(|o| o.has_flashed).count() as u64;
        for o in self.octopi.iter_mut() {
            o.has_flashed = false;
        }
        flashes
    }
}

fn main() {
    // part a
    let mut octopi = Octopi::from_str(include_str!("../data/main.txt")).unwrap();
    let flashes = (0..100).map(|_| octopi.step()).sum::<u64>();
    println!("{}", flashes);
    //part b
    let mut octopi = Octopi::from_str(include_str!("../data/main.txt")).unwrap();
    let mut step = 1;
    while octopi.step() != 100 {
        step += 1;
    }
    println!("{}", step);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut octopi = Octopi::from_str(include_str!("../data/test.txt")).unwrap();
        let flashes = (0..100).map(|_| octopi.step()).sum::<u64>();
        println!("{:?}", octopi.octopi);
        assert_eq!(flashes, 1656);
    }
}
