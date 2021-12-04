use anyhow::{anyhow, Result};

pub trait CallNumber {
    fn call_number(&mut self, _number: i32) {}
}

impl CallNumber for Board {
    fn call_number(&mut self, number: i32) {
        for square in self.squares.iter_mut() {
            square.call_number(number)
        }
        if !self.is_winner {
            self.last_move += 1;
        }
        // see if board has won
        for row in self.squares.chunks(self.row_len) {
            if row.iter().all(|square| square.is_checked) {
                self.is_winner = true;
            }
        }
        // no columns
        for col in 0..self.row_len {
            if self.squares[col..]
                .iter()
                .step_by(self.row_len)
                .all(|square| square.is_checked)
            {
                self.is_winner = true;
            }
        }

        if self.is_winner && self.score.is_none() {
            self.score = Some(
                number
                    * self
                        .squares
                        .iter()
                        .filter_map(|square| match square.is_checked {
                            false => Some(square.number),
                            true => None,
                        })
                        .sum::<i32>(),
            )
        }
    }
}

impl CallNumber for BingoSquare {
    fn call_number(&mut self, number: i32) {
        if number == self.number {
            self.is_checked = true;
        }
    }
}

#[derive(Debug, Clone)]
pub struct BingoSquare {
    pub number: i32,
    pub is_checked: bool,
}

impl BingoSquare {
    pub fn new(val: &str) -> Result<Self> {
        let number = val.parse()?;
        Ok(BingoSquare {
            number,
            is_checked: false,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub squares: Vec<BingoSquare>,
    row_len: usize,
    col_len: usize,
    is_winner: bool,
    score: Option<i32>,
    last_move: usize,
}
impl Board {
    fn add_row(&mut self, data: &str) -> Result<()> {
        let new_squares: Vec<BingoSquare> = data
            .trim()
            .split_whitespace()
            .map(|v| BingoSquare::new(v))
            .collect::<Result<_>>()?;
        if new_squares.len() != self.row_len {
            return Err(anyhow!("got invalid length for row"));
        }
        self.squares.extend_from_slice(&new_squares[..]);
        Ok(())
    }
    fn new<'a, I>(data: &mut I) -> Result<Board>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut board = Board {
            squares: Vec::new(),
            col_len: 5,
            row_len: 5,
            is_winner: false,
            score: None,
            last_move: 0,
        };
        for _ in 0..board.col_len {
            let v = data.next().unwrap();
            if v.is_empty() {
                break;
            }
            board.add_row(v)?;
        }
        Ok(board)
    }
}

fn parse_data(data: &str) -> Result<(Vec<i32>, Vec<Board>)> {
    let mut boards = Vec::new();
    let mut lines = data.lines().peekable();
    let numbers = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i32>().map_err(anyhow::Error::from))
        .collect::<Result<Vec<i32>>>()?;
    assert_eq!(Some(""), lines.next());
    while lines.peek().is_some() {
        boards.push(Board::new(&mut lines)?);
        lines.next();
    }
    Ok((numbers, boards))
}

fn find_winner(boards: &mut Vec<Board>, numbers: &Vec<i32>) -> i32 {
    for num in numbers {
        for board in boards.iter_mut() {
            board.call_number(*num)
        }
        if let Some(top_score) = boards.iter().filter_map(|b| b.score).max() {
            return top_score;
        }
    }
    unreachable!("No boards won bingo");
}

fn find_loser(boards: &mut Vec<Board>, numbers: &Vec<i32>) -> i32 {
    for num in numbers {
        for board in boards.iter_mut() {
            board.call_number(*num)
        }
    }
    // find the boards which one last
    let final_move = boards
        .iter()
        .map(|b| b.last_move)
        .max()
        .expect("unreachable");
    let lowest_of_losers = boards
        .iter()
        .filter(|b| b.last_move == final_move)
        .filter_map(|b| b.score)
        .min()
        .expect("unreachable");
    lowest_of_losers
}

fn main() -> Result<()> {
    let data = include_str!("data.txt");
    let (numbers, mut boards) = parse_data(data)?;
    println!(
        "The winning score is {}",
        find_winner(&mut boards, &numbers)
    );
    println!("The losing score is {}", find_loser(&mut boards, &numbers));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let test_data = include_str!("test.txt");
        let (numbers, mut boards) = parse_data(test_data).unwrap();
        assert_eq!(boards.len(), 3);
        assert_eq!(4512, find_winner(&mut boards, &numbers));
        assert_eq!(1924, find_loser(&mut boards, &numbers));
    }
}
