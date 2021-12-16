use anyhow::{anyhow, Result};
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    pos: usize,
    cost: u32,
}
// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
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
            let row: Vec<_> = line
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

    fn tile(&self, n: usize) -> Self {
        // first tile all the rows
        let mut y_tiler = Vec::with_capacity(self.col_len * self.row_len * n);
        fn rolled_add(x: u32, i: usize) -> u32 {
            let tmp = x + i as u32;
            if tmp <= 9 {
                tmp
            } else {
                tmp - 9
            }
        }
        for row in 0..self.col_len {
            for i in 0..n {
                y_tiler.extend(
                    self.floor[row * self.row_len..(row + 1) * self.row_len]
                        .iter()
                        .map(|&x| rolled_add(x, i)),
                )
            }
        }
        let floor = (0..n)
            .map(|i| {
                y_tiler
                    .iter()
                    .map(|&x| rolled_add(x, i))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        Cave {
            floor,
            col_len: self.col_len * n,
            row_len: self.row_len * n,
        }
    }
    fn dijkstra_alg(&self, start: usize) -> Option<u32> {
        let mut to_visit: BinaryHeap<_> = self
            .get_neighbors(start)
            .iter()
            .map(|&x| State {
                pos: x,
                cost: self.floor[x],
            })
            .collect();
        let mut have_visited = HashSet::new();
        // no backtracking
        have_visited.insert(start);

        while let Some(State { pos, cost }) = to_visit.pop() {
            if pos == self.floor.len() - 1 {
                return Some(cost);
            }
            if !have_visited.contains(&pos) {
                have_visited.insert(pos);
                for neighbor in self.get_neighbors(pos) {
                    to_visit.push(State {
                        pos: neighbor,
                        cost: cost + self.floor[neighbor],
                    });
                }
            }
        }
        None
    }
}

fn main() {
    let cave = Cave::from_str(include_str!("../data/main.txt")).unwrap();
    println!("{:?}", cave.dijkstra_alg(0).unwrap());
    let tiled_cave = cave.tile(5);
    println!("{:?}", tiled_cave.dijkstra_alg(0).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let cave = Cave::from_str(include_str!("../data/test.txt")).unwrap();
        assert_eq!(cave.dijkstra_alg(0), Some(40));
        let tiled_cave = cave.tile(5);
        assert_eq!(tiled_cave.dijkstra_alg(0), Some(315));
    }
}
