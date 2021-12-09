use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::str::FromStr;
pub struct Cave {
    floor: Vec<u32>,
    row_len: usize,
    col_len: usize,
}
impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut col_len = 0;
        let mut line_iter = s.lines().peekable();
        let row_len = if let Some(line) = line_iter.peek() {
            line.len()
        } else {
            return Err(anyhow!("empty string"));
        };
        let mut floor = Vec::new();
        for line in line_iter {
            let row: Vec<u32> = line
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if c.is_numeric() && c.is_ascii() {
                        Ok(c.to_digit(10).unwrap())
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
            floor.extend_from_slice(&row);
            col_len += 1;
        }
        Ok(Cave {
            floor,
            row_len,
            col_len,
        })
    }
}

impl Cave {
    fn get_neighbors(&self, pos: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        // Not at the start of a row
        if pos % self.row_len != 0 {
            neighbors.push(pos - 1);
        }
        // not at end of a row
        if pos % self.row_len != self.row_len - 1 {
            neighbors.push(pos + 1);
        }
        // not in first row
        if pos >= self.row_len {
            neighbors.push(pos - self.row_len);
        }
        // not in last row
        if pos / self.row_len < self.col_len - 1 {
            neighbors.push(pos + self.row_len);
        }

        neighbors
    }

    fn get_low_points(&self) -> Vec<usize> {
        self.floor
            .iter()
            .enumerate()
            .filter_map(|(i, &val)| {
                if val
                    < self
                        .get_neighbors(i)
                        .iter()
                        .map(|o| self.floor[*o])
                        .min()
                        .unwrap()
                {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_basin_size(&self, start: usize) -> usize {
        let mut to_visit = vec![start];
        let mut have_visited = HashSet::new();

        while let Some(x) = to_visit.pop() {
            have_visited.insert(x);
            for neighbor in self.get_neighbors(x) {
                if self.floor[neighbor] != 9 && !have_visited.contains(&neighbor) {
                    to_visit.push(neighbor);
                }
            }
        }
        have_visited.len()
    }

    fn get_all_basins(&self) -> Vec<usize> {
        let mut basins = self
            .get_low_points()
            .iter()
            .map(|&pos| self.get_basin_size(pos))
            .collect::<Vec<_>>();
        basins.sort();
        basins.into_iter().rev().collect()
    }
}

fn main() {
    let cave = Cave::from_str(include_str!("../data/real.txt")).unwrap();
    println!(
        "{}",
        cave.get_low_points()
            .iter()
            .map(|&i| cave.floor[i] + 1)
            .sum::<u32>()
    );
    println!(
        "{}",
        cave.get_all_basins()
            .iter()
            .take(3)
            .fold(1, |acc, x| x * acc)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let cave = Cave::from_str(include_str!("../data/test.txt")).unwrap();
        assert_eq!(
            cave.get_low_points()
                .iter()
                .map(|&i| cave.floor[i] + 1)
                .sum::<u32>(),
            15
        );
        assert_eq!(
            cave.get_all_basins()
                .iter()
                .take(3)
                .fold(1, |acc, x| x * acc),
            1134
        );
    }
}
