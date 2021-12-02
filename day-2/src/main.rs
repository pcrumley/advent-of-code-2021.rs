use anyhow::{anyhow, Result};
use std::convert::TryFrom;
use std::ops::Add;

#[derive(Debug)]
struct Position {
    x: i32,
    depth: i32,
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<_> = value.split(" ").collect();
        let split_tup: (&str, i32) = (split[0], split[1].parse()?);
        match split_tup {
            ("forward", x) => Ok(Command::Forward(x)),
            ("down", y) => Ok(Command::Down(y)),
            ("up", y) => Ok(Command::Up(y)),
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

fn parse_and_sum(data: &str) -> Result<Position> {
    data.lines()
        .map(Command::try_from)
        .try_fold(Position { x: 0, depth: 0 }, |acc, command| {
            command.map(|c| match c {
                Command::Forward(x) => Position {
                    x: acc.x + x,
                    depth: acc.depth,
                },
                Command::Up(y) => Position {
                    x: acc.x,
                    depth: acc.depth - y,
                },
                Command::Down(y) => Position {
                    x: acc.x,
                    depth: acc.depth + y,
                },
            })
        })
}

fn parse_with_aim(data: &str) -> Result<PositionWithAim> {
    data.lines().map(Command::try_from).try_fold(
        PositionWithAim {
            x: 0,
            depth: 0,
            aim: 0,
        },
        |acc, command| {
            command.map(|c| match c {
                Command::Forward(x) => PositionWithAim {
                    x: acc.x + x,
                    depth: acc.depth + acc.aim * x,
                    aim: acc.aim,
                },
                Command::Up(y) => PositionWithAim {
                    x: acc.x,
                    depth: acc.depth,
                    aim: acc.aim - y,
                },
                Command::Down(y) => PositionWithAim {
                    x: acc.x,
                    depth: acc.depth,
                    aim: acc.aim + y,
                },
            })
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
