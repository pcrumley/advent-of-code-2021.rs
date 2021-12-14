use anyhow::{anyhow, Result};
use std::{collections::VecDeque, str::FromStr};

pub struct Paper {
    pub rows: Vec<Vec<char>>,
    pub instructions: VecDeque<FoldInstruction>,
}

pub enum FoldInstruction {
    FoldY(usize),
    FoldX(usize),
}
impl FromStr for FoldInstruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("fold along y") {
            if let Some(y) = s.rsplit("=").next() {
                Ok(FoldInstruction::FoldY(y.parse::<usize>()?))
            } else {
                Err(anyhow!("invalid fold y instruction: {}", s))
            }
        } else if s.starts_with("fold along x") {
            if let Some(x) = s.rsplit("=").next() {
                Ok(FoldInstruction::FoldX(x.parse::<usize>()?))
            } else {
                Err(anyhow!("invalid fold x instruction: {}", s))
            }
        } else {
            Err(anyhow!("invalid instruction format: {}", s))
        }
    }
}

impl Paper {
    pub fn fold_y(&mut self, y_fold: usize) {
        let mut folded_section = self.rows.split_off(y_fold);
        folded_section.remove(0);

        if folded_section.len() > self.rows.len() {
            let tmp = self.rows.iter().rev().cloned().collect::<Vec<_>>();
            self.rows = folded_section.iter().rev().cloned().collect::<Vec<_>>();
            folded_section = tmp;
        }
        for (r1, r2) in self.rows.iter_mut().rev().zip(folded_section.iter()) {
            for (x1, x2) in r1.iter_mut().zip(r2.iter()) {
                match x2 {
                    '.' => (),
                    '#' => *x1 = '#',
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn fold_row(mut row: Vec<char>, x_fold: usize) -> Vec<char> {
        let mut folded_section = row.split_off(x_fold);
        folded_section.remove(0);
        if folded_section.len() > row.len() {
            let tmp = row.iter().rev().cloned().collect::<Vec<_>>();
            row = folded_section.iter().rev().cloned().collect::<Vec<_>>();
            folded_section = tmp;
        }
        row.iter()
            .rev()
            .zip(folded_section.into_iter().chain(std::iter::repeat('.')))
            .map(|(&x1, x2)| match x2 {
                '.' => x1,
                '#' => '#',
                _ => unreachable!(),
            })
            .collect()
    }

    pub fn fold_x(&mut self, x_fold: usize) {
        self.rows = self
            .rows
            .iter()
            .cloned()
            .map(|row| Paper::fold_row(row, x_fold))
            .collect()
    }

    pub fn count_dots(&self) -> usize {
        self.rows
            .iter()
            .map(|r| r.iter().filter(|&&x| x == '#').count())
            .sum()
    }

    pub fn fold(&mut self) {
        match self.instructions.pop_front() {
            Some(FoldInstruction::FoldY(y)) => self.fold_y(y),
            Some(FoldInstruction::FoldX(x)) => self.fold_x(x),
            None => (),
        }
    }

    pub fn fold_all(&mut self) {
        while self.instructions.len() != 0 {
            self.fold()
        }
    }
}

impl FromStr for Paper {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn to_tup(s: &str) -> Result<(usize, usize)> {
            let as_vec = s.split(",").collect::<Vec<_>>();
            if as_vec.len() != 2 {
                return Err(anyhow!("invalid input, expected comma separated values"));
            }
            Ok((as_vec[0].parse::<usize>()?, as_vec[1].parse::<usize>()?))
        }
        let max_x = s
            .lines()
            .filter_map(|s| to_tup(s).ok())
            .map(|(x, _y)| x)
            .max()
            .expect("can't fail");
        let max_y = s
            .lines()
            .filter_map(|s| to_tup(s).ok())
            .map(|(_x, y)| y)
            .max()
            .expect("can't fail");
        let mut rows = (0..=max_y)
            .map(|_| vec!['.'; max_x + 1])
            .collect::<Vec<_>>();

        for (x, y) in s.lines().filter_map(|s| to_tup(s).ok()) {
            rows[y][x] = '#';
        }
        let instructions: VecDeque<_> = s
            .lines()
            .filter_map(|s| FoldInstruction::from_str(s).ok())
            .collect();
        Ok(Paper { rows, instructions })
    }
}

fn main() {
    let mut paper = Paper::from_str(include_str!("../data/main.txt")).unwrap();
    paper.fold();
    println!("{}", paper.count_dots());
    paper.fold_all();
    for row in paper.rows {
        let out_str = row.iter().collect::<String>();
        println!("{}", out_str);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut paper = Paper::from_str(include_str!("../data/test.txt")).unwrap();

        let expected = vec![
            vec!['.', '.', '.', '#', '.', '.', '#', '.', '.', '#', '.'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '#', '.', '.', '.', '.', '#', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '.', '.', '#', '.', '#', '#', '.'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        for (row, expected_row) in paper.rows.iter().zip(expected.iter()) {
            assert_eq!(row, expected_row);
        }
        paper.fold();
        let expected = vec![
            vec!['#', '.', '#', '#', '.', '.', '#', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#'],
            vec!['#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '#', '.', '.', '#', '.', '#', '#', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        for (i, (row, expected_row)) in paper.rows.iter().zip(expected.iter()).enumerate() {
            println!("{}", i);
            assert_eq!(row, expected_row);
        }

        assert_eq!(paper.count_dots(), 17);
        paper.fold();
        let expected = vec![
            vec!['#', '#', '#', '#', '#'],
            vec!['#', '.', '.', '.', '#'],
            vec!['#', '.', '.', '.', '#'],
            vec!['#', '.', '.', '.', '#'],
            vec!['#', '#', '#', '#', '#'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];
        for (i, (row, expected_row)) in paper.rows.iter().zip(expected.iter()).enumerate() {
            println!("{}", i);
            assert_eq!(row, expected_row);
        }

        assert_eq!(paper.count_dots(), 16);
    }
}
