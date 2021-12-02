use anyhow::{anyhow, Result};
use std::convert::TryFrom;

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
                    ..acc
                },
                Command::Up(y) => Position {
                    depth: acc.depth - y,
                    ..acc
                },
                Command::Down(y) => Position {
                    depth: acc.depth + y,
                    ..acc
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
                    ..acc
                },
                Command::Up(y) => PositionWithAim {
                    aim: acc.aim - y,
                    ..acc
                },
                Command::Down(y) => PositionWithAim {
                    aim: acc.aim + y,
                    ..acc
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
