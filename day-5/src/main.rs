use anyhow::{anyhow, Result};

pub struct OceanFloor {
    vals: Vec<Vec<u32>>,
    row_len: usize,
    col_len: usize,
}
impl OceanFloor {
    pub fn new(lines: &Vec<Line>, include_diag: bool) -> Self {
        // find xmax and ymax
        let (xmax, ymax) = lines.iter().fold((0, 0), |acc, line| {
            (
                acc.0.max(line.start.x).max(line.end.x),
                acc.1.max(line.start.y).max(line.end.y),
            )
        });
        let (row_len, col_len) = (xmax + 1, ymax + 1);
        let mut vals = (0..col_len)
            .map(|_| vec![0; row_len])
            .collect::<Vec<Vec<u32>>>();
        for line in lines
            .iter()
            .filter(|l| if include_diag { true } else { !l.is_diagonal() })
        {
            for point in line.to_points() {
                vals[point.y][point.x] += 1;
            }
        }

        OceanFloor {
            vals,
            row_len,
            col_len,
        }
    }
    pub fn find_intersections(&self) -> u32 {
        self.vals
            .iter()
            .map(|row| row.iter().filter(|&&v| v > 1).count() as u32)
            .sum()
    }
}
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new<'a, I>(mut data: I) -> Result<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        let x = match data.next().map(|x| x.parse::<usize>()) {
            Some(Ok(v)) => v,
            _ => return Err(anyhow!("no x value")),
        };
        let y = match data.next().map(|o| o.parse::<usize>()) {
            Some(Ok(v)) => v,
            _ => return Err(anyhow!("no y value")),
        };
        Ok(Point { x, y })
    }
}

#[derive(Debug, PartialEq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    // data are format x0,y0 -> x1,y1
    pub fn new(data: &str) -> Result<Line> {
        let mut points: Vec<_> = data
            .split("->")
            .map(|o| Point::new(o.trim().split(",")))
            .collect::<Result<_>>()?;
        assert_eq!(2, points.len());
        let end = points.pop().unwrap();
        let start = points.pop().unwrap();
        Ok(Line { start, end })
    }
    pub fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }

    pub fn to_points(&self) -> Vec<Point> {
        let mut x_range = vec![self.start.x];
        let mut y_range = vec![self.start.y];
        let mut place = 0;
        while x_range[place] != self.end.x {
            if x_range[place] < self.end.x {
                x_range.push(x_range[place] + 1);
            } else {
                x_range.push(x_range[place] - 1);
            }
            place += 1
        }
        let mut place = 0;
        while y_range[place] != self.end.y {
            if y_range[place] < self.end.y {
                y_range.push(y_range[place] + 1);
            } else {
                y_range.push(y_range[place] - 1);
            }
            place += 1
        }

        assert!(!(x_range.len() == 1 && y_range.len() == 1));
        if x_range.len() == 1 {
            std::iter::repeat(x_range[0])
                .zip(y_range)
                .map(|(x, y)| Point { x, y })
                .collect()
        } else if y_range.len() == 1 {
            x_range
                .into_iter()
                .zip(std::iter::repeat(y_range[0]))
                .map(|(x, y)| Point { x, y })
                .collect()
        } else {
            x_range
                .into_iter()
                .zip(y_range)
                .map(|(x, y)| Point { x, y })
                .collect()
        }
    }
}

fn main() {
    let data = include_str!("data.txt");
    let lines = data
        .lines()
        .map(Line::new)
        .collect::<Result<Vec<_>>>()
        .unwrap();
    let ocean_floor = OceanFloor::new(&lines, false);
    println!("{}", ocean_floor.find_intersections());
    let ocean_floor = OceanFloor::new(&lines, true);
    println!("{}", ocean_floor.find_intersections());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_data = include_str!("test.txt");
        let lines = test_data
            .lines()
            .map(Line::new)
            .collect::<Result<Vec<_>>>()
            .unwrap();
        let ocean_floor = OceanFloor::new(&lines, false);
        assert_eq!(5, ocean_floor.find_intersections());
        println!("{:?}", ocean_floor.vals);
        assert_eq!(lines.len(), 10);
        assert_eq!(
            lines[0],
            Line {
                start: Point { x: 0, y: 9 },
                end: Point { x: 5, y: 9 }
            }
        );
        let ocean_floor = OceanFloor::new(&lines, true);
        assert_eq!(12, ocean_floor.find_intersections());
    }
}
