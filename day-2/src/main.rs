use anyhow::{anyhow, Result};
use std::convert::TryFrom;
use std::ops::Add;

#[derive(Debug)]
struct Position {
    x: i32,
    depth: i32,
}

impl TryFrom<&str> for Position {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<_> = value.split(" ").collect();
        let split_tup: (&str, i32) = (split[0], split[1].parse()?);
        match split_tup {
            ("forward", x) => Ok(Position { x, depth: 0 }),
            ("down", y) => Ok(Position { x: 0, depth: y }),
            ("up", y) => Ok(Position { x: 0, depth: -y }),
            _ => Err(anyhow!("invalid input")),
        }
    }
}

#[derive(Debug)]
struct PositionWithAim {
    x: i32,
    depth: i32,
    aim: i32,
}

impl TryFrom<&str> for PositionWithAim {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<_> = value.split(" ").collect();
        let split_tup: (&str, i32) = (split[0], split[1].parse()?);
        match split_tup {
            ("forward", x) => Ok(Self {
                x,
                depth: 0,
                aim: 0,
            }),
            ("down", y) => Ok(Self {
                x: 0,
                depth: 0,
                aim: y,
            }),
            ("up", y) => Ok(Self {
                x: 0,
                depth: 0,
                aim: -y,
            }),
            _ => Err(anyhow!("invalid input")),
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            depth: self.depth + other.depth,
        }
    }
}

fn parse_and_sum(data: &str) -> Result<Position> {
    data.lines()
        .map(Position::try_from)
        .try_fold(
            Position { x: 0, depth: 0 },
            |acc, position| match position {
                Ok(p) => Ok(acc + p),
                Err(e) => Err(e),
            },
        )
}

fn parse_with_aim(data: &str) -> Result<PositionWithAim> {
    data.lines().map(PositionWithAim::try_from).try_fold(
        PositionWithAim {
            x: 0,
            depth: 0,
            aim: 0,
        },
        |acc, position| match position {
            Ok(p) => {
                let ans = PositionWithAim {
                    x: acc.x + p.x,
                    depth: acc.aim * p.x + acc.depth,
                    aim: acc.aim + p.aim,
                };
                // println!("{:#?}", ans);
                Ok(ans)
            }
            Err(e) => Err(e),
        },
    )
}

fn main() -> Result<()> {
    let test_course = include_str!("test_course.txt");
    let course = include_str!("course.txt");
    assert_eq!(150, parse_and_sum(&test_course).map(|o| o.x * o.depth)?);
    println!("{}", parse_and_sum(&course).map(|o| o.x * o.depth)?);
    assert_eq!(900, parse_with_aim(&test_course).map(|o| o.x * o.depth)?);
    println!("{}", parse_with_aim(&course).map(|o| o.x * o.depth)?);

    Ok(())
}
